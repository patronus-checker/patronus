// Basically #include <patronus/patronus-provider.h>
#[macro_use]
extern crate patronus_provider;

// Import some names into the scope so we do not have to type
// qualified names all the time
use patronus_provider::*;
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};

// Rust, like C++ uses name mangling by default. Here, we disable
// it and also set force the function to use C calling conventions.
#[no_mangle]
pub extern "C" fn patronus_provider_version() -> c_int {
    // Simply return 1, Rust returns the last statement
    // in the function
    1
}

#[no_mangle]
pub extern "C" fn get_name() -> *const c_char {
    // We use macro to convert a string to C-style string
    // at compile-time and then return it.
    static_cstr!("Sample checker")
}

/// This is the main checking method – normally we would use
/// functions imported from some library but here, for simplicity,
/// we just create our own grammar checking library.
///
/// It finds all the occurrences of “mistakes are good” string
/// in the input text and suggests correction to one of contrary
/// statements.
fn check_text_english(text: Cow<str>) -> *mut AnnotationArray {
    // Here we first create an iterator with all occurrences
    // of the string and their indices, then we immediately
    // convert (map) them to an iterator of annotations
    let mistakes: Vec<Annotation> = text
        .match_indices("mistakes are good")
        .map(|(offset, text)| {
            // Just preparing some data structures.
            let length = text.len() as usize;
            let suggestions = vec![
                static_cstr!("mistakes are never good"),
                static_cstr!("mistakes are bad"),
            ].into();
            Annotation {
                offset: offset,
                length: length,
                message: static_cstr!("Are you sure about mistakes being good?"),
                kind: AnnotationKind::Suggestion,
                suggestions: Box::into_raw(Box::new(suggestions)),
            }
        })
        .collect();
    // Boxing is Rust’s way of creating a pointer safely
    // into_raw will then convert it to a raw C pointer.
    Box::into_raw(Box::new(mistakes.into()))
}

/// This is the function called by Patronus for checking text
/// it handles the properties, calls the library function, and
/// usually converts the result to annotation vector. Here, our
/// “library” is already producing the vector so we do not need to.
extern "C" fn check_text(
    props: *const Properties,
    text: *const c_char,
    _data: *mut c_void,
) -> *mut AnnotationArray {
    // Converts a C string into a Rust owned String.
    let lang_code = unsafe {
        CStr::from_ptr((*props).primary_language)
            .to_string_lossy()
            .into_owned()
    };
    // Same here but we do not need the ownership.
    let text = unsafe { CStr::from_ptr(text).to_string_lossy() };
    // We remove the country code, our checker is good for any English.
    let lang = lang_code
        .splitn(2, '_')
        .nth(0)
        .expect("not enough language code components");

    // This checker only knows English, so we return an empty vector
    // otherwise
    match lang {
        "en" => check_text_english(text),
        _ => Box::into_raw(Box::new(Vec::new().into())),
    }
}

unsafe extern "C" fn free_annotations(ptr: *mut AnnotationArray) {
    // Unboxing changes the ownership of the value to this scope
    // so it is immediately freed as not used.
    let anns = Box::from_raw(ptr);
    for i in 0..anns.len {
        let ann = &*anns.data.offset(i as isize);
        Box::from_raw(ann.suggestions);
    }
}

unsafe extern "C" fn free_provider(ptr: *mut Provider) {
    assert!(!ptr.is_null(), "Trying to clean a NULL value");
    let _provider = Box::from_raw(ptr);
}

/// Initialize the provider with the functions
#[no_mangle]
pub extern "C" fn patronus_provider_init() -> *mut Provider {
    Box::into_raw(Box::new(Provider {
        name: get_name,
        check: check_text,
        free_annotations: free_annotations,
        free_provider: free_provider,
        // We do not really need to store anything here, so we just fill in null
        data: std::ptr::null_mut(),
    }))
}

// Some simple tests for the library
#[test]
fn test_english_single_mistake() {
    let text = "mistakes are good";
    let result = check_text_english(text.into());
    let length = unsafe { (*result).len };
    assert!(length == 1);
}

#[test]
fn test_english_multiple_mistakes() {
    let text =
        "Hello. It is true that mistakes are good. Mistakes are good, you know, mistakes are good!";
    let result = check_text_english(text.into());
    let length = unsafe { (*result).len };
    assert!(length == 2);
}

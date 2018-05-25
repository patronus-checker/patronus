#[macro_use]
extern crate patronus_provider;

use patronus_provider::*;
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};

#[no_mangle]
pub extern "C" fn patronus_provider_version() -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn get_name() -> *const c_char {
    static_cstr!("Sample checker")
}

fn check_text_english(text: Cow<str>) -> *mut AnnotationArray {
    let mistakes: Vec<Annotation> = text
        .match_indices("mistakes are good")
        .map(|(offset, text)| {
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
    Box::into_raw(Box::new(mistakes.into()))
}

extern "C" fn check_text(
    props: *const Properties,
    text: *const c_char,
    _data: *mut c_void,
) -> *mut AnnotationArray {
    let lang_code = unsafe {
        CStr::from_ptr((*props).primary_language)
            .to_string_lossy()
            .into_owned()
    };
    let text = unsafe { CStr::from_ptr(text).to_string_lossy() };
    let lang = lang_code
        .splitn(2, '_')
        .nth(0)
        .expect("not enough language code components");

    match lang {
        "en" => check_text_english(text),
        _ => Box::into_raw(Box::new(Vec::new().into())),
    }
}

unsafe extern "C" fn free_annotations(ptr: *mut AnnotationArray) {
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

#[no_mangle]
pub extern "C" fn patronus_provider_init() -> *mut Provider {
    Box::into_raw(Box::new(Provider {
        name: get_name,
        check: check_text,
        free_annotations: free_annotations,
        free_provider: free_provider,
        data: std::ptr::null_mut(),
    }))
}

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

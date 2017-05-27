extern crate patronus;
extern crate patronus_provider;

use patronus_provider::{Annotation, AnnotationArray, Properties, Suggestion};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Opaque wrapper for `Patronus` struct.
#[repr(C)]
pub struct Patronus(patronus::Patronus);

/// Creates an instance of `Patronus` checker.
/// The returned value should be cleaned using `patronus_free` after use.
#[no_mangle]
pub extern "C" fn patronus_create() -> *mut Patronus {
    Box::into_raw(Box::new(Patronus(patronus::Patronus::new())))
}

/// Cleans up the `Patronus` object returned by `patronus_create`.
#[no_mangle]
pub unsafe extern "C" fn patronus_free(ptr: *mut Patronus) {
    assert!(!ptr.is_null(), "Trying to free a NULL pointer.");
    Box::from_raw(ptr);
}

/// Checks provided text for mistakes.
/// The returned value should be cleaned using `patronus_free_annotations` after use.
#[no_mangle]
pub unsafe extern "C" fn patronus_check(ptr: *mut Patronus,
                                        props: *const Properties,
                                        text: *const std::os::raw::c_char)
                                        -> *mut AnnotationArray {
    assert!(!ptr.is_null(), "Trying to use a NULL pointer.");
    assert!(!props.is_null(), "Trying to use a NULL pointer.");

    let patronus = &(*ptr).0;

    let properties = {
        let Properties { primary_language } = *props;
        patronus::Properties {
            primary_language: CStr::from_ptr(primary_language)
                .to_string_lossy()
                .into_owned(),
        }
    };

    let anns = patronus
        .check(&properties, &CStr::from_ptr(text).to_string_lossy())
        .iter()
        .map(|&patronus::Annotation {
                  offset,
                  length,
                  ref message,
                  kind,
                  ref suggestions,
              }| {
            let msg = CString::new(message.clone())
                .expect("cannot create C string")
                .into_raw() as *const c_char;
            let suggestions: Vec<Suggestion> = suggestions
                .into_iter()
                .map(|sugg| {
                         CString::new((*sugg).clone())
                             .expect("cannot create C string")
                             .into_raw() as *const c_char
                     })
                .collect();
            Annotation {
                offset: offset,
                length: length,
                message: msg,
                kind: kind,
                suggestions: Box::into_raw(Box::new(suggestions.into())),
            }
        })
        .collect::<Vec<Annotation>>()
        .into();
    Box::into_raw(Box::new(anns))
}

/// Cleans up the `AnnotationArray` returned by `patronus_check`.
#[no_mangle]
pub unsafe extern "C" fn patronus_free_annotations(ptr: *mut AnnotationArray) {
    assert!(!ptr.is_null(), "Trying to use a NULL pointer.");

    let anns = Box::from_raw(ptr);
    for i in 0..anns.len {
        let ann = &*anns.data.offset(i as isize);
        let suggs = Box::from_raw(ann.suggestions);
        for i in 0..suggs.len {
            let sugg = *suggs.data.offset(i as isize);
            CString::from_raw(sugg as *mut c_char);
        }
        CString::from_raw(ann.message as *mut c_char);
    }
}

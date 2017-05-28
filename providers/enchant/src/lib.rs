extern crate enchant;
#[macro_use]
extern crate patronus_provider;

use enchant::Broker;
use patronus_provider::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

#[no_mangle]
pub extern "C" fn patronus_provider_version() -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn get_name() -> *const c_char {
    static_cstr!("Enchant")
}

extern "C" fn check_text(props: *const Properties,
                         text: *const c_char,
                         data: *mut c_void)
                         -> *mut AnnotationArray {
    let broker = unsafe { &mut *(data as *mut Broker) };

    let lang = unsafe { CStr::from_ptr((*props).primary_language).to_string_lossy() };
    let text = unsafe { CStr::from_ptr(text).to_string_lossy().into_owned() };

    let mut offset = 0;
    let mut result = Vec::new();

    if let Ok(dict) = broker.request_dict(&lang) {
        let words = text.split(|c: char| !c.is_alphabetic());
        for word in words {
            let length = word.len();
            if length > 0 {
                if !dict.check(word).unwrap_or(true) {
                    let suggestions: Vec<Suggestion> = dict.suggest(word)
                        .into_iter()
                        .map(|sugg| {
                                 CString::new(sugg)
                                     .expect("cannot create C string")
                                     .into_raw() as *const c_char
                             })
                        .collect();
                    let ann = Annotation {
                        offset: offset,
                        length: length,
                        message: static_cstr!("Word was not found in the dictionary"),
                        kind: AnnotationKind::Spelling,
                        suggestions: Box::into_raw(Box::new(suggestions.into())),
                    };
                    result.push(ann);
                }
                offset += length + 1;
            }
        }
    }
    Box::into_raw(Box::new(result.into()))
}

unsafe extern "C" fn free_annotations(ptr: *mut AnnotationArray) {
    let anns = Box::from_raw(ptr);
    for i in 0..anns.len {
        let ann = &*anns.data.offset(i as isize);
        let suggs = Box::from_raw(ann.suggestions);
        for i in 0..suggs.len {
            let sugg = *suggs.data.offset(i as isize);
            CString::from_raw(sugg as *mut c_char);
        }
    }
}

unsafe extern "C" fn free_provider(ptr: *mut Provider) {
    assert!(!ptr.is_null(), "Trying to clean a NULL value");
    let provider = Box::from_raw(ptr);
    Box::from_raw(provider.data as *mut Broker);
}

#[no_mangle]
pub extern "C" fn patronus_provider_init() -> *mut Provider {
    let broker: *mut Broker = Box::into_raw(Box::new(Broker::new()));

    Box::into_raw(Box::new(Provider {
                               name: get_name,
                               check: check_text,
                               free_annotations: free_annotations,
                               free_provider: free_provider,
                               data: broker as *mut c_void,
                           }))
}

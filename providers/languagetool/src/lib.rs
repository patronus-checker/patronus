extern crate config;
extern crate languagetool;
extern crate patronus_provider;
extern crate xdg;

use languagetool::{LanguageTool, Request, Response};
use patronus_provider::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

const CONFIG_INSTANCE_URL: &str = "providers.languagetool.instance_url";

#[no_mangle]
pub extern "C" fn patronus_provider_version() -> c_int {
    1
}

#[no_mangle]
pub extern "C" fn get_name() -> *const c_char {
    static_cstr!("Language Tool")
}

extern "C" fn check_text(
    props: *const Properties,
    text: *const c_char,
    data: *mut c_void,
) -> *mut AnnotationArray {
    let lt = unsafe { &mut *(data as *mut LanguageTool) };

    let lang = unsafe {
        CStr::from_ptr((*props).primary_language)
            .to_string_lossy()
            .into_owned()
    };
    let text = unsafe { CStr::from_ptr(text).to_string_lossy().into_owned() };

    let req = Request::new(text, lang);

    let anns = {
        if let Ok(Response {
            matches: Some(matches),
            ..
        }) = lt.check(req)
        {
            matches
                .into_iter()
                .map(|mtch| {
                    let offset = mtch.offset as usize;
                    let length = mtch.length as usize;
                    let suggestions: Vec<Suggestion> = mtch
                        .replacements
                        .into_iter()
                        .filter_map(|replacement| replacement.value)
                        .map(|sugg| {
                            CString::new(sugg)
                                .expect("cannot create C string")
                                .into_raw() as *const c_char
                        })
                        .collect();
                    let ann = Annotation {
                        offset: offset,
                        length: length,
                        message: CString::new(mtch.message)
                            .expect("cannot create C string")
                            .into_raw(),
                        kind: AnnotationKind::Grammar,
                        suggestions: Box::into_raw(Box::new(suggestions.into())),
                    };
                    ann as Annotation
                })
                .collect::<Vec<Annotation>>()
        } else {
            Vec::<Annotation>::new()
        }
    };
    Box::into_raw(Box::new(anns.into()))
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
        CString::from_raw(ann.message as *mut c_char);
    }
}

unsafe extern "C" fn free_provider(ptr: *mut Provider) {
    assert!(!ptr.is_null(), "Trying to clean a NULL value");
    let provider = Box::from_raw(ptr);
    Box::from_raw(provider.data as *mut LanguageTool);
}

#[no_mangle]
pub extern "C" fn patronus_provider_init() -> *mut Provider {
    let mut c = config::Config::new();
    c.set_default(CONFIG_INSTANCE_URL, "http://localhost:8081/")
        .expect("Cannot set default value for instance url.");
    if let Ok(xdg_dirs) = xdg::BaseDirectories::with_prefix("patronus") {
        if let Some(path) = xdg_dirs.find_config_file("config.toml") {
            let user_config = config::File::new(&path.to_string_lossy(), config::FileFormat::Toml)
                .required(false);
            c.merge(user_config)
                .expect("Cannot  merge LanguageTool provider configuration.");
        }
    }

    let instance_url = c
        .get_str(CONFIG_INSTANCE_URL)
        .expect("Could not determine instance URL.");
    match LanguageTool::new(&instance_url) {
        Err(msg) => {
            panic!("Cannot create Language Tool instance: {}", msg);
        }
        Ok(lt) => {
            let lt: *mut LanguageTool = Box::into_raw(Box::new(lt));

            Box::into_raw(Box::new(Provider {
                name: get_name,
                check: check_text,
                free_annotations: free_annotations,
                free_provider: free_provider,
                data: lt as *mut c_void,
            }))
        }
    }
}

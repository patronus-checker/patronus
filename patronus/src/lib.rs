extern crate libloading as lib;
extern crate patronus_provider;

use self::error::Error;
use patronus_provider as provider;
pub use patronus_provider::AnnotationKind;
use std::borrow::Cow;
use std::env;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs;
use std::os::raw::c_int;
use std::path::Path;
use std::path::PathBuf;

mod error;

/// Represents a profile to be passed down to checkers.
/// Currently only primary language is supported.
pub struct Properties {
    pub primary_language: String,
}

/// Unified annotation produced by the checkers.
#[derive(Debug)]
pub struct Annotation {
    pub offset: usize,
    pub length: usize,
    pub message: String,
    pub kind: AnnotationKind,
    pub suggestions: Vec<String>,
}

const PROVIDER_VERSION_FUNCTION: &[u8] = b"patronus_provider_version\0";
const PROVIDER_INIT_FUNCTION: &[u8] = b"patronus_provider_init\0";

/// Provider wrapper.
/// Keeps the associated dynamically loaded library so it could be properly freed.
pub struct Provider {
    internal: *mut provider::Provider,
    library: *mut lib::Library,
}

impl Provider {
    /// Checks a text for mistakes using given provider.
    pub fn check(&self, props: *const provider::Properties, text: &Cow<str>) -> Vec<Annotation> {
        let text = CString::new(text.clone().into_owned()).expect("cannot create C string");

        let response = unsafe { (*self.internal).check(props, text.as_ptr()) };
        let annotations = unsafe { &*response.annotations };
        let length = annotations.len;
        let mut anns = Vec::with_capacity(length);

        unsafe {
            if !annotations.data.is_null() {
                for i in 0..length {
                    let provider::Annotation {
                        offset,
                        length,
                        message,
                        kind,
                        suggestions,
                    } = *annotations.data.offset(i as isize);

                    let suggestions = &*suggestions;
                    let suggestions = {
                        let length = suggestions.len;
                        let mut suggs = Vec::with_capacity(length);
                        if !suggestions.data.is_null() {
                            for i in 0..length {
                                let sugg = *suggestions.data.offset(i as isize);
                                suggs.push(CStr::from_ptr(sugg).to_string_lossy().into_owned())
                            }
                        }
                        suggs
                    };

                    anns.push(Annotation {
                        offset,
                        length,
                        message: CStr::from_ptr(message).to_string_lossy().into_owned(),
                        kind,
                        suggestions,
                    });
                }
            }
        }
        anns
    }

    /// Get name of the provider provider.
    pub fn name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr((*self.internal).name()).to_string_lossy() }
    }
}

impl Drop for Provider {
    fn drop(&mut self) {
        unsafe {
            ((*self.internal).free_provider)(self.internal);

            Box::from_raw(self.library);
        }
    }
}

/// Main struct holding providers and other relevant data.
pub struct Patronus {
    pub providers: Vec<Provider>,
}

impl Patronus {
    /// Initializes Patronus and loads the providers.
    pub fn new() -> Self {
        Self {
            providers: Self::load_providers().expect("cannot load providers"),
        }
    }

    /// Checks a text for mistakes using all loaded providers.
    pub fn check(&self, props: &Properties, text: &Cow<str>) -> Vec<Annotation> {
        let primary_language =
            CString::new(&*props.primary_language).expect("Cannot create language C String");
        let properties = provider::Properties {
            primary_language: primary_language.as_ptr(),
        };

        let mut res = Vec::new();
        for provider in &self.providers {
            res.extend(provider.check(&properties, text))
        }
        res
    }

    /// Traverses provider directories and tries to load all shared libraries.
    /// The main provider directory is set during compile time from `PATRONUS_PROVIDER_DIR`
    /// environment variable (/usr/lib/patronus by default).
    /// Additionally, the directories listed in `PATRONUS_PROVIDER_PATH` at runtime are crawled as well.
    fn load_providers() -> Result<Vec<Provider>, Error> {
        let mut provider_locations = vec![PathBuf::from(env!("PATRONUS_PROVIDER_DIR"))];
        if let Some(provider_path) = env::var_os("PATRONUS_PROVIDER_PATH") {
            for path in env::split_paths(&provider_path) {
                provider_locations.push(path);
            }
        }

        let mut result = Vec::new();
        for location in provider_locations {
            if location.is_dir() {
                for entry in fs::read_dir(location).map_err(|source| Error::IoError { source })? {
                    let path = entry.map_err(|source| Error::IoError { source })?.path();
                    if path.is_file() && path.is_dylib() {
                        let lib = Box::new(unsafe {
                            lib::Library::new(&path)
                                .map_err(|source| Error::LibloadingError { source })?
                        });
                        let version = unsafe {
                            match lib.get(PROVIDER_VERSION_FUNCTION)
                                as Result<lib::Symbol<unsafe extern "C" fn() -> c_int>, lib::Error>
                            {
                                Ok(get_version) => get_version(),
                                Err(_) => continue,
                            }
                        };
                        match version {
                            1 => {
                                let internal_provider = unsafe {
                                    let init_provider: lib::Symbol<
                                        unsafe extern "C" fn() -> *mut provider::Provider,
                                    > = lib
                                        .get(PROVIDER_INIT_FUNCTION)
                                        .map_err(|source| Error::LibloadingError { source })?;
                                    init_provider()
                                };
                                result.push(Provider {
                                    internal: internal_provider,
                                    library: Box::into_raw(lib),
                                });
                            }
                            _ => panic!(
                                "Unsupported provider version {} for provider {:?}",
                                version, path
                            ),
                        }
                    }
                }
            }
        }
        Ok(result)
    }
}

trait DylibTestable {
    /// Checks whether given object is a dynamic library.
    fn is_dylib(&self) -> bool;
}

/// `Path`is probably a dynamic library when it ends with certain extension.
/// The extension is platform specific – `dylib` for MacOS, `dll` for Windows and `so`
/// everywhere else.
impl DylibTestable for Path {
    #[cfg(target_os = "macos")]
    fn is_dylib(&self) -> bool {
        self.extension().map_or(false, |ext| ext == "dylib")
    }
    #[cfg(target_os = "windows")]
    fn is_dylib(&self) -> bool {
        self.extension().map_or(false, |ext| ext == "dll")
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    fn is_dylib(&self) -> bool {
        self.extension().map_or(false, |ext| ext == "so")
    }
}

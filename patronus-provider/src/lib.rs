mod macros;
mod parray;

pub use parray::PArray;

/// Properties of the text to be checked.
#[derive(Debug)]
#[repr(C)]
pub struct Properties {
    pub primary_language: *const std::os::raw::c_char,
}

/// Type of annotation.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum AnnotationKind {
    Spelling = 1,
    Grammar = 2,
    Style = 3,
    Typography = 4,
    Suggestion = 5,
}

/// Suggestion string
pub type Suggestion = *const std::os::raw::c_char;
/// Array of suggestions
pub type SuggestionArray = PArray<Suggestion>;
/// Array of annotations
pub type AnnotationArray = PArray<Annotation>;

/// C-ABI compatible `Annotation` struct.
#[derive(Debug)]
#[repr(C)]
pub struct Annotation {
    pub offset: usize,
    pub length: usize,
    pub message: *const std::os::raw::c_char,
    pub kind: AnnotationKind,
    pub suggestions: *mut SuggestionArray,
}

/// Wrapper for provider response allowing automatic cleanup.
#[derive(Debug)]
pub struct Response {
    pub annotations: *mut AnnotationArray,
    cleanup: unsafe extern "C" fn(*mut AnnotationArray),
}

impl Drop for Response {
    fn drop(&mut self) {
        unsafe { (self.cleanup)(self.annotations) }
    }
}

/// Provider struct to be returned `patronus_provider_init` function
/// from dynamic library of a provider.
#[repr(C)]
pub struct Provider {
    pub name: unsafe extern "C" fn() -> *const std::os::raw::c_char,
    pub check: unsafe extern "C" fn(
        props: *const Properties,
        text: *const std::os::raw::c_char,
        data: *mut std::os::raw::c_void,
    ) -> *mut AnnotationArray,
    pub free_annotations: unsafe extern "C" fn(*mut AnnotationArray),
    pub free_provider: unsafe extern "C" fn(*mut Provider),
    pub data: *mut std::os::raw::c_void,
}

impl Provider {
    pub fn check(&self, props: *const Properties, text: *const std::os::raw::c_char) -> Response {
        Response {
            annotations: unsafe { (self.check)(props, text, self.data) },
            cleanup: self.free_annotations,
        }
    }

    pub fn name(&self) -> *const std::os::raw::c_char {
        unsafe { (self.name)() }
    }
}

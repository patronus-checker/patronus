#[macro_export]

/// Creates a static string (usually stored in [`.rodata`]) and returns a pointer to it (`*const c_char`).
/// [`.rodata`]: https://en.wikipedia.org/wiki/.rodata
macro_rules! static_cstr {($x:expr) => (concat!($x, "\0").as_ptr() as *const _ ) }

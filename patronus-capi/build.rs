extern crate cheddar;

use std::fs::File;
use std::io::{Read, Write};

fn main() {
    const GENERATED_HEADER_FILE: &str = "extra/patronus/patronus.h";

    cheddar::Cheddar::new()
        .expect("could not read manifest")
        .insert_code(include_str!("extra/patronus-types.h"))
        .run_build(GENERATED_HEADER_FILE);

    // Prefix the type names: https://github.com/Sean1708/rusty-cheddar/issues/43
    let mut file = File::open(GENERATED_HEADER_FILE).expect("Cannot  open header file.");
    let mut data = String::new();
    file.read_to_string(&mut data).expect(
        "Cannot read header file.",
    );
    let data = data.replace("Spelling =", "AnnotationKindSpelling =")
        .replace("Grammar =", "AnnotationKindGrammar =")
        .replace("Style =", "AnnotationKindStyle =")
        .replace("Typography =", "AnnotationKindTypography =")
        .replace("Suggestion =", "AnnotationKindSuggestion =")
        .replace("Annotation", "PatronusAnnotation")
        .replace("Suggestion", "PatronusSuggestion")
        .replace("Properties", "PatronusProperties");
    let mut file =
        File::create(GENERATED_HEADER_FILE).expect("Cannot open header file for writing.");
    file.write_all(data.as_bytes()).expect(
        "Cannot write header file.",
    );

    let prefix = option_env!("BUILD_PREFIX").unwrap_or("/usr");
    let libdir = option_env!("BUILD_LIBDIR").unwrap_or("${prefix}/lib");
    let includedir = option_env!("BUILD_INCLUDEDIR").unwrap_or("${prefix}/include");

    let description = env!("CARGO_PKG_DESCRIPTION");
    let version = env!("CARGO_PKG_VERSION");

    // Create a pkgconfig file
    let pkgconfig_contents = format!(
        "prefix={}
exec_prefix=${{prefix}}
libdir={}
includedir={}

Name: libpatronus
Description: {}
Version: {}
Libs: -L${{libdir}} -lpatronus
Cflags: -I${{includedir}}/patronus
",
        prefix,
        libdir,
        includedir,
        description,
        version
    );
    let mut pkgconfig = File::create("extra/patronus.pc").expect("Cannot create pkgconfig file.");
    pkgconfig.write(pkgconfig_contents.as_bytes()).expect(
        "Cannot write pkgconfig file.",
    );
}

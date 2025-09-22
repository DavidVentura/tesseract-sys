extern crate bindgen;
extern crate cmake;
extern crate cc;

use std::env;
use std::fs;
use std::path::PathBuf;
#[cfg(windows)]
use vcpkg;

fn capi_bindings(clang_extra_include: &[String]) -> bindgen::Bindings {
    let mut capi_bindings = bindgen::Builder::default()
        .header("wrapper_capi.h")
        .allowlist_function("^Tess.*")
        .blocklist_type("Boxa")
        .blocklist_type("Pix")
        .blocklist_type("Pixa")
        .blocklist_type("_IO_FILE")
        .blocklist_type("_IO_codecvt")
        .blocklist_type("_IO_marker")
        .blocklist_type("_IO_wide_data");

    for inc in clang_extra_include {
        capi_bindings = capi_bindings.clang_arg(format!("-I{}", *inc));
    }

    capi_bindings
        .generate()
        .expect("Unable to generate capi bindings")
}

#[cfg(not(target_os = "macos"))]
fn public_types_bindings(clang_extra_include: &[String]) -> String {
    let mut public_types_bindings = bindgen::Builder::default()
        .header("wrapper_public_types.hpp")
        .rustified_enum("tesseract::OcrEngineMode")
        .rustified_enum("tesseract::Orientation")
        .rustified_enum("tesseract::PageIteratorLevel")
        .rustified_enum("tesseract::PageSegMode")
        .rustified_enum("tesseract::ParagraphJustification")
        .rustified_enum("tesseract::PolyBlockType")
        .rustified_enum("tesseract::TextlineOrder")
        .rustified_enum("tesseract::WritingDirection")
        .blocklist_item("^kPolyBlockNames")
        .blocklist_item("^tesseract::kPolyBlockNames");

    for inc in clang_extra_include {
        public_types_bindings = public_types_bindings.clang_arg(format!("-I{}", *inc));
    }

    public_types_bindings
        .generate()
        .expect("Unable to generate public types bindings")
        .to_string()
        .replace("tesseract_", "")
}

// MacOS clang is incompatible with Bindgen and constexpr
// https://github.com/rust-lang/rust-bindgen/issues/1948
// Hardcode the constants rather than reading them dynamically
#[cfg(target_os = "macos")]
fn public_types_bindings(_clang_extra_include: &[String]) -> &'static str {
    include_str!("src/public_types_bindings_mac.rs")
}

fn main() {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=User32");
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=Crypt32");
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=Advapi32");

    let leptonica_lib = env::var("DEP_LEPT_LIB").expect("leptonica-sys should provide lib path");

    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();

    let build_dir = format!("{}/tesseract-build-{}", out_dir, target);
    let _dst = cmake::Config::new("tesseract")
        .define("BUILD_TRAINING_TOOLS", "OFF")
        .define("BUILD_TESTS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("DISABLE_TIFF", "ON")
        .define("DISABLE_ARCHIVE", "ON")
        .define("DISABLE_CURL", "ON")
        .define("GRAPHICS_DISABLED", "ON")
        .define("CMAKE_INSTALL_CONFIG", "OFF")
        // Tesseract requires C++17
        .define("CMAKE_CXX_STANDARD", "17")
        .define("CMAKE_CXX_STANDARD_REQUIRED", "ON")
        .define("CMAKE_CXX_EXTENSIONS", "OFF")
        .define("CMAKE_PREFIX_PATH", &leptonica_lib)
        .define("Leptonica_DIR", &leptonica_lib)
        .out_dir(&build_dir)
        .always_configure(true)
        .build_target("libtesseract")
        .build();

    // The library is built in the cmake build directory, not installed
    let lib_path = format!("{}/build", build_dir);
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=static=tesseract");

    println!("cargo:rustc-link-lib=stdc++");

    // Set up include paths for bindgen - use source headers since we're not installing
    let include_path = format!(
        "{}/include",
        env::current_dir().unwrap().join("tesseract").display()
    );

    // Compile our custom C API extensions
    cc::Build::new()
        .cpp(true)
        .include(&include_path)
        .include("tesseract/include")
        .file("custom_capi.cpp")
        .compile("custom_capi");
    let clang_extra_include = vec![include_path];

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    capi_bindings(&clang_extra_include)
        .write_to_file(out_path.join("capi_bindings.rs"))
        .expect("Couldn't write capi bindings!");
    fs::write(
        out_path.join("public_types_bindings.rs"),
        public_types_bindings(&clang_extra_include),
    )
    .expect("Couldn't write public types bindings!");
}

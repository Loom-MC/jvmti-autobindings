use std::env;
use std::path::PathBuf;

fn main() {
    let java_home = std::env::var("JAVA_HOME")
        .expect("Please set JAVA_HOME to your JDK path.");

    let include_dir = format!("{}/include", java_home);
    let include_os_dir = if cfg!(target_os = "macos") {
        format!("{}/include/darwin", java_home)
    } else if cfg!(target_os = "linux") {
        format!("{}/include/linux", java_home)
    } else {
        format!("{}/include/win32", java_home)
    };

    let bindings = bindgen::Builder::default()
        .header(format!("{}/jvmti.h", include_dir))
        .clang_arg(format!("-I{}", include_dir))
        .clang_arg(format!("-I{}", include_os_dir))
        .allowlist_function(".*JVMTI.*|.*jvmti.*")
        .allowlist_type(".*jvmti.*")
        .allowlist_var(".*JVMTI.*")
        .allowlist_var(".*jvmti.*")
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings for JVMTI");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("jvmti_bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=build.rs");
}

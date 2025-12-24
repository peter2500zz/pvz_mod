use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=third_party/Detours");

    // 获取当前 crate 的清单文件所在目录
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    // 构建 third_party/Detours 的绝对路径
    let detours_dir = PathBuf::from(&manifest_dir)
        .join("third_party")
        .join("Detours");

    println!("cargo:warning=Building Detours in: {:?}", detours_dir);

    let status = match Command::new("nmake").current_dir(&detours_dir).status() {
        Ok(status) => status,
        Err(e) => {
            println!("cargo::error={}{}", "Failed to execute nmake: ", e);
            panic!()
        }
    };

    if !status.success() {
        println!(
            "cargo::error={}{:?}",
            "nmake build failed with exit code: ",
            status.code()
        );

        panic!()
    }

    // 设置链接搜索路径（使用绝对路径）
    let lib_dir = detours_dir.join("lib.X86");
    println!("cargo:rustc-link-search={}", lib_dir.display());

    println!("cargo:rustc-link-lib=static=detours");
    println!("cargo:rustc-link-lib=static=syelog");
}

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out = build();
    link(&out);
}

/// Build the nappgui library
fn build() -> PathBuf {
    let mut dst = cmake::Config::new("nappgui_src");
    dst.define("NAPPGUI_DEMO", "NO");
    dst.define("NAPPGUI_WEB", "NO");

    if cfg!(feature = "webview") {
        dst.define("NAPPGUI_WEB", "YES");
    }

    if cfg!(target_os = "macos") {
        // Get sysroot from xcrun
        let output = Command::new("xcrun")
            .args(&["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .expect("failed to run xcrun");

        let mut sysroot = String::from_utf8(output.stdout)
            .expect("invalid utf8")
            .trim()
            .to_string();

        if Path::new(&sysroot)
            .file_name()
            .is_some_and(|name| name == "MacOSX.sdk")
        {
            if let Some(versioned_sysroot) = versioned_macos_sdk(&sysroot) {
                sysroot = versioned_sysroot;
            }
        }

        dst.define("CMAKE_OSX_SYSROOT", &sysroot);
    }

    if cfg!(target_os = "windows") {
        dst.profile("release"); // Always set to Release on Windows in order to prevent link to _CrtDumpMemoryLeaks ...
        dst.define("CMAKE_C_FLAGS", "-D_WINDOWS");
        dst.define("CMAKE_CXX_FLAGS", "-D_WINDOWS");
        
        if std::env::var("TARGET").unwrap().contains("msvc") {
            dst.cflag("/utf-8");
            dst.cxxflag("/utf-8");
        }
    } else {
        dst.profile(&env::var("PROFILE").unwrap());
    }
    dst.build()
}

fn versioned_macos_sdk(sysroot: &str) -> Option<String> {
    let sdk_dir = Path::new(sysroot).parent()?;
    let mut sdks = std::fs::read_dir(sdk_dir)
        .ok()?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name()?.to_str()?;
            let version = name.strip_prefix("MacOSX")?.strip_suffix(".sdk")?;

            if version.is_empty() {
                return None;
            }

            Some((parse_version(version), path))
        })
        .collect::<Vec<_>>();

    sdks.sort_by(|(a, _), (b, _)| a.cmp(b));
    sdks.pop().map(|(_, path)| path.display().to_string())
}

fn parse_version(version: &str) -> Vec<u32> {
    version
        .split('.')
        .map(|part| part.parse::<u32>().unwrap_or(0))
        .collect()
}

/// Link the nappgui library
fn link(out: &std::path::PathBuf) {
    println!("cargo:rustc-link-search=native={}/lib", out.display());
    println!("cargo:rustc-link-lib=static=core");
    println!("cargo:rustc-link-lib=static=draw2d");
    println!("cargo:rustc-link-lib=static=geom2d");
    println!("cargo:rustc-link-lib=static=gui");
    println!("cargo:rustc-link-lib=static=inet");
    println!("cargo:rustc-link-lib=static=ogl3d");
    println!("cargo:rustc-link-lib=static=osapp");
    println!("cargo:rustc-link-lib=static=osbs");
    println!("cargo:rustc-link-lib=static=osgui");
    println!("cargo:rustc-link-lib=static=sewer");
    println!("cargo:rustc-link-lib=static=encode");

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=comctl32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=oleaut32");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=uuid");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=comdlg32");
        println!("cargo:rustc-link-lib=winspool");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=kernel32");
        println!("cargo:rustc-link-lib=odbc32");
        println!("cargo:rustc-link-lib=gdiplus");
        println!("cargo:rustc-link-lib=UxTheme");
        println!("cargo:rustc-link-lib=Shlwapi");
        println!("cargo:rustc-link-lib=wininet");
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-lib=framework=UniformTypeIdentifiers");
        println!("cargo:rustc-link-lib=dylib=objc");
    }
}

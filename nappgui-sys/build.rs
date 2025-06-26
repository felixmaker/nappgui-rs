use std::path::PathBuf;

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

    dst.profile("Release");
    dst.build()
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
    }
}

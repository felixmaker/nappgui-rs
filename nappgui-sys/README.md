# nappgui-sys

Rust raw bindings to nappgui library.

NAppGUI is a professional SDK to build cross-platform desktop applications using The C Programming Language (ANSI C90). NAppGUI has built as a lightweight layer on top of the operating system native APIs, that allows create portable programs extremelly fast and small without any external dependency.

See [frang75/nappgui_src](https://github.com/frang75/nappgui_src) and its [home page](https://nappgui.com/) for more information.


## Requirements

 - Git: for some patches on Windows build
 - CMake: for building the NAppGui static library
 - Windows SDK: only needed when using MSVC


## Generate bindings

cmd /c .\bind.bat


@echo off
cd nappgui_src/src
bindgen --use-core --no-layout-tests --blocklist-function _.* --ctypes-prefix=libc --disable-name-namespacing --disable-nested-struct-naming --default-enum-style=rust_non_exhaustive -o ../../src/nappgui.rs nappgui.h  -- -I. -D_WINDOWS

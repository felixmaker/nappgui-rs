@echo off
cd nappgui_src/src
bindgen --use-core --no-layout-tests --blocklist-function _.* --ctypes-prefix=::libc ^
 --disable-name-namespacing --disable-nested-struct-naming --no-doc-comments ^
 --default-enum-style=consts ^
 -o ../../src/nappgui.rs nappgui.h  -- -I. -D_WINDOWS

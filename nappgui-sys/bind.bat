@echo off
bindgen --use-core --no-layout-tests --blocklist-function _.* --ctypes-prefix=::libc ^
 --disable-name-namespacing --disable-nested-struct-naming --no-doc-comments ^
 --default-enum-style=consts ^
 -o src/nappgui.rs bind.h  -- -I. -I./nappgui_src/src -D_WINDOWS -D__GNUC__=4 -D__GNUC_MINOR__=2

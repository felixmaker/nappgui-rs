pub(crate) fn array_u32(array: *mut nappgui_sys::ArrStuint32_t) -> Option<Vec<u32>> {
    if array.is_null() {
        return None;
    }

    let array = unsafe { *array };

    if array.content.is_null() {
        return None;
    }

    let content = unsafe { *array.content };

    let elem = &content.elem;

    Some(elem.to_vec())
}

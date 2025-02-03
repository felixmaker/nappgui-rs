use nappgui_sys::{
    heap_aligned_calloc_imp, heap_aligned_malloc_imp, heap_aligned_realloc, heap_auditor_add,
    heap_auditor_delete, heap_calloc_imp, heap_end_mt, heap_free, heap_leaks, heap_malloc_imp,
    heap_realloc, heap_start_mt, heap_stats, heap_verbose,
};

/// Start a multi-threaded section.
pub fn start_mt() {
    unsafe { heap_start_mt() };
}

/// End a multi-threaded section.
pub fn end_mt() {
    unsafe { heap_end_mt() };
}

/// Enable/disable memory auditor 'verbose' mode.
///
/// # Remarks
/// By default FALSE.
pub fn verbose(verbose: bool) {
    unsafe { heap_verbose(verbose as i8) };
}

/// Enable/disable memory auditor statistics.
///
/// # Remarks
/// By default TRUE.
pub fn stats(stats: bool) {
    unsafe { heap_stats(stats as i8) };
}

/// Returns TRUE if there are memory leaks at the end of execution.
pub fn leaks() -> bool {
    unsafe { heap_leaks() != 0 }
}

/// Reserve a memory block with the default alignment sizeof(void*).
///
/// # Remarks
/// Pointer to the new block. Must be released with heap_free when it is no longer necessary.
pub fn malloc(size: u32, name: &str, equal_size: bool) -> *mut u8 {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { heap_malloc_imp(size, name.as_ptr(), equal_size as i8) }
}

/// Like heap_malloc, but initializing the block with 0s.
pub fn calloc(size: u32, name: &str, equal_size: bool) -> *mut u8 {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { heap_calloc_imp(size, name.as_ptr(), equal_size as i8) }
}

/// Reallocs an existing memory block due to the expansion or reduction of it. Guarantees that the previous
/// content of the block is preserved min(size, new_size). Try to do it without moving memory (in situ), but
/// if it is not possible look for a new zone. It also guarantees the default alignment sizeof(void*) if you
/// have to reserve a new block.
pub fn realloc(ptr: *mut u8, size: u32, new_size: u32, name: &str) -> *mut u8 {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { heap_realloc(ptr, size, new_size, name.as_ptr()) }
}

/// Reserve a memory block with alignment.
pub fn aligned_malloc(size: u32, align: u32, name: &str, equal_size: bool) -> *mut u8 {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { heap_aligned_malloc_imp(size, align, name.as_ptr(), equal_size as i8) }
}

/// Like heap_aligned_malloc , but initializing the block with 0s.
pub fn aligned_calloc(size: u32, align: u32, name: &str, equal_size: bool) -> *mut u8 {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { heap_aligned_calloc_imp(size, align, name.as_ptr(), equal_size as i8) }
}

/// Like heap_realloc, but guaranteeing memory alignment.
pub fn aligned_realloc(ptr: *mut u8, size: u32, new_size: u32, align: u32, name: &str) -> *mut u8 {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { heap_aligned_realloc(ptr, size, new_size, align, name.as_ptr()) }
}


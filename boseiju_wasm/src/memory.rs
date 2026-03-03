/// Allocate a given number of bytes for JS.
///
/// This forgets the allocation to prevent Rust from freeing it.
#[unsafe(no_mangle)]
pub extern "C" fn alloc(len: usize) -> *const u8 {
    let block: Vec<u8> = vec![0u8; len];
    let ptr = block.as_ptr();
    /* Prevent Rust auto dealloc */
    std::mem::forget(block);
    ptr
}

/// Free a given block of bytes, provided as a pointer / length pair.
#[unsafe(no_mangle)]
pub extern "C" fn free(ptr: *mut u8, len: usize) {
    /* Recreate the block, and let Rust dealloc it */
    let block = unsafe { Vec::from_raw_parts(ptr, len, len) };
    drop(block);
}

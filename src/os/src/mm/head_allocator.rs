use crate::config;
use buddy_system_allocator as sysalloc;

#[global_allocator]
static HEAP_ALLOCATOR: sysalloc::LockedHeap = sysalloc::LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Failed to allocate on heap, layout = {:?}", layout);
}

static mut HEAP_SPACE: [u8; config::KERNEL_HEAP_SIZE] = [0; config::KERNEL_HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, config::KERNEL_HEAP_SIZE)
    }
}

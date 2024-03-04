<<<<<<< HEAD
// reference: https://github.com/xfoxfu/rust-xos/blob/main/kernel/src/allocator.rs

=======
use crate::proc::PageTableContext;
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
use linked_list_allocator::LockedHeap;
use x86_64::structures::paging::{
    mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
};
use x86_64::VirtAddr;

pub const USER_HEAP_START: usize = 0x4000_0000_0000;
pub const USER_HEAP_SIZE: usize = 1024 * 1024; // 1 MiB
const USER_HEAP_PAGE: usize = USER_HEAP_SIZE / crate::memory::PAGE_SIZE as usize;

pub static USER_ALLOCATOR: LockedHeap = LockedHeap::empty();

<<<<<<< HEAD
=======
// NOTE: export mod user / call in the kernel init / after frame allocator
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4
pub fn init() {
    init_user_heap().expect("User Heap Initialization Failed.");
    info!("User Heap Initialized.");
}

pub fn init_user_heap() -> Result<(), MapToError<Size4KiB>> {
<<<<<<< HEAD
    let mapper = &mut *super::get_page_table_for_sure();
    let frame_allocator = &mut *super::get_frame_alloc_for_sure();

    let page_range = {
        let heap_start = VirtAddr::new(USER_HEAP_START as u64);
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = heap_start_page + USER_HEAP_PAGE as u64 - 1u64;
        Page::range(heap_start_page, heap_end_page)
    };

    debug!(
        "User Heap        : 0x{:016x}-0x{:016x}",
        page_range.start.start_address().as_u64(),
        page_range.end.start_address().as_u64()
    );

    let (size, unit) = super::humanized_size(USER_HEAP_SIZE as u64);
    info!("User Heap Size   : {:>7.*} {}", 3, size, unit);

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }
=======
    // Get current pagetable mapper
    let mapper = &mut PageTableContext::new().mapper();
    // Get global frame allocator
    let frame_allocator = &mut *super::get_frame_alloc_for_sure();

    // FIXME: use elf::map_range to allocate & map
    //        frames (R/W/User Access)
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4

    unsafe {
        USER_ALLOCATOR
            .lock()
            .init(USER_HEAP_START as *mut u8, USER_HEAP_SIZE);
    }

    Ok(())
}

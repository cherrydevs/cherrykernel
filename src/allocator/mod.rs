use fixed_size_block::FixedSizeBlockAllocator;
use spin::{Mutex, MutexGuard};
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

pub mod bump;
pub mod fixed_size_block;
pub mod memory;

#[global_allocator]
static ALLOCATOR: Lock<FixedSizeBlockAllocator> = Lock::new(FixedSizeBlockAllocator::new());

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 2000 * 1024; // 2MB

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    prepare_pages(mapper, frame_allocator, HEAP_START, HEAP_SIZE)?;
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
    Ok(())
}

pub fn prepare_pages(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    heap_start: usize,
    heap_size: usize,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(heap_start as u64);
        let heap_end = heap_start + heap_size - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    Ok(())
}

/// A wrapper around spin::Mutex to permit trait implementations.
pub struct Lock<A>(Mutex<A>);

impl<A> Lock<A> {
    pub const fn new(inner: A) -> Self {
        Lock(Mutex::new(inner))
    }

    pub fn lock(&self) -> MutexGuard<A> {
        self.0.lock()
    }
}

/// Align the given address `addr` upwards to alignment `align`.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

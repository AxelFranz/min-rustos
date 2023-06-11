use x86_64::{
    VirtAddr,
    structures::paging::{PageTable, OffsetPageTable},
};

use x86_64::{
    structures::paging::{Page, PhysFrame, Mapper, Size4KiB, FrameAllocator}
};

/// Inits a Page Table at the offset given
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level4table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level4table,physical_memory_offset)
}

/// Creates a level 4 Page Table by the offset given
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level4_table_frame, _) = Cr3::read();
    let phys = level4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr:  *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr

}
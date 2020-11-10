use x86_64::{
    structures::paging::PageTable,
    VirtAddr,
};

pub unsafe fn active_level_4_table(phys_addr: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phy = level_4_table_frame.start_address();
    let virt = phys_addr + phy.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}
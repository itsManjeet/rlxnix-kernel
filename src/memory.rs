use x86_64::{
    structures::paging::{
        FrameAllocator,
        OffsetPageTable,
        PageTable,
        PhysFrame,
        Size4KiB,
    },
    VirtAddr,
    PhysAddr,
};
use bootloader::bootinfo::{
    MemoryMap,
    MemoryRegionType
};


pub unsafe fn initialize(phy_addr: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(phy_addr);
    OffsetPageTable::new(level_4_table, phy_addr)
}


unsafe fn active_level_4_table(phys_addr: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phy = level_4_table_frame.start_address();
    let virt = phys_addr + phy.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn initialize(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();

        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        let addr_range = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addr = addr_range.flat_map(|r| r.step_by(4096));

        frame_addr.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}


unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
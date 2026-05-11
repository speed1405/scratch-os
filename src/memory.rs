use x86_64::structures::paging::{
    PageTable, PhysFrame, Size4KiB, FrameAllocator, OffsetPageTable
};
use x86_64::{PhysAddr, VirtAddr};

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

extern "C" {
    static __kernel_start: u8;
    static __kernel_end: u8;
}

pub struct BootInfoFrameAllocator {
    memory_areas: &'static [multiboot2::MemoryArea],
    kernel_start: PhysAddr,
    kernel_end: PhysAddr,
    multiboot_start: PhysAddr,
    multiboot_end: PhysAddr,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static multiboot2::MemoryMapTag,
                       boot_info_start: usize, boot_info_end: usize) -> Self {
        BootInfoFrameAllocator {
            memory_areas: memory_map.memory_areas(),
            kernel_start: PhysAddr::new(unsafe { &__kernel_start as *const u8 as u64 }),
            kernel_end: PhysAddr::new(unsafe { &__kernel_end as *const u8 as u64 }),
            multiboot_start: PhysAddr::new(boot_info_start as u64),
            multiboot_end: PhysAddr::new(boot_info_end as u64),
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_areas.iter();
        let usable_regions = regions
            .filter(|r| r.typ() == multiboot2::MemoryAreaType::Available);
        let addr_ranges = usable_regions
            .map(|r| r.start_address()..r.end_address());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

        let kernel_start = self.kernel_start.as_u64();
        let kernel_end = self.kernel_end.as_u64();
        let multiboot_start = self.multiboot_start.as_u64();
        let multiboot_end = self.multiboot_end.as_u64();

        frame_addresses
            .filter(move |&addr| {
                // Check if the frame is not within the kernel or multiboot structure
                let is_kernel = addr >= kernel_start && addr < kernel_end;
                let is_multiboot = addr >= multiboot_start && addr < multiboot_end;
                !is_kernel && !is_multiboot
            })
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

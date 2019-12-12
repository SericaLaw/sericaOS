use super::{Page, ActivePageTable, Frame, FrameAllocator};
use super::table::{Table, Level1};

pub struct TemporaryPage {
    page: Page,
    allocator: TinyAllocator,
}

impl TemporaryPage {
    pub fn new<A>(page: Page, allocator: &mut A) -> TemporaryPage
        where A: FrameAllocator
    {
        TemporaryPage {
            page: page,
            allocator: TinyAllocator::new(allocator),
        }
    }
    /// Maps the temporary page to the given frame in the active table.
    /// Returns the start address of the temporary page.
    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable)
        -> usize
    {
        use super::entry::EntryBits;
        println!("map temporary page {:x?} to given frame {:x?} in the active table", self.page, frame);
        assert!(active_table.translate_page(self.page).is_none(),
                "temporary page is already mapped");
        active_table.map_to(self.page, frame, EntryBits::ReadWrite.val(), &mut self.allocator);
        println!("temporary map done, start address:0x{:x?}\n", self.page.start_address());

        use super::super::print_entry;
        print_entry(2, [self.page.p2_index(), 0, 0]);
        print_entry(1, [self.page.p2_index(), self.page.p1_index(), 0]);
        print_entry(0, [self.page.p2_index(), self.page.p1_index(), 1022]);
        print_entry(0, [self.page.p2_index(), self.page.p1_index(), 1023]);

        self.page.start_address()
    }

    /// Unmaps the temporary page in the active table.
    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
        println!("temporary page unmap");
        active_table.unmap(self.page, &mut self.allocator)
    }

    /// Maps the temporary page to the given page table frame in the active
    /// table. Returns a reference to the now mapped table.
    pub fn map_table_frame(&mut self,
                           frame: Frame,
                           active_table: &mut ActivePageTable)
                           -> &mut Table<Level1> {
        unsafe { &mut *(self.map(frame, active_table) as *mut Table<Level1>) }
    }

    pub fn p2_index(&self) -> usize {
        self.page.p2_index()
    }

    pub fn p1_index(&self) -> usize {
        self.page.p1_index()
    }
}

struct TinyAllocator([Option<Frame>; 1]);

impl TinyAllocator {
    fn new<A>(allocator: &mut A) -> TinyAllocator
        where A: FrameAllocator
    {
        let mut f = || allocator.allocate_frame();
        let frames = [f()];
        TinyAllocator(frames)
    }
}

impl FrameAllocator for TinyAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                return frame_option.take();
            }
        }
        None
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        for frame_option in &mut self.0 {
            if frame_option.is_none() {
                *frame_option = Some(frame);
                return;
            }
        }
        panic!("Tiny allocator can hold only 3 frames.");
    }
}
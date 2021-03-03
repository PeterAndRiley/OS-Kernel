pub mod attr;
pub mod handler;
pub mod area; 

use area::MemoryArea;
use attr::MemoryAttr;
use crate::memory::paging::PageTableImpl;
use crate::consts::*;
use handler::{
    MemoryHandler,
    Linear
};
use alloc::{
    boxed::Box,
    vec::Vec
};
use crate::memory::access_pa_via_va;

pub struct MemorySet {
    areas: Vec<MemoryArea>,
    // Use the page table to manage all its mappings
    page_table: PageTableImpl,
}

impl MemorySet {
    pub fn push(&mut self, start: usize, end: usize, attr: MemoryAttr, handler: impl MemoryHandler) {
        assert!(start <= end, "invalid memory area!");
        assert!(self.test_free_area(start, end), "memory area overlap!");
        // Construct MemoryArea
        let area = MemoryArea::new(start, end, Box::new(handler), attr);
        // Refresh the map
        area.map(&mut self.page_table);
        // Refresh the MemoryArea set
        self.areas.push(area);
    } 
    fn test_free_area(&self, start: usize, end: usize) -> bool {
        self.areas
            .iter()
            .find(|area| area.is_overlap_with(start, end))
            .is_none()
    }
    /*
        Switch the virtual address space where 
        the CPU is located to this MemorySet
    */ 
    pub unsafe fn activate(&self) {
        self.page_table.activate();
    }
	pub fn new() -> Self {
        let mut memory_set = MemorySet {
            areas: Vec::new(),
            page_table: PageTableImpl::new_bare(),
        };
        // Insert kernel segments and physical memory segments
        memory_set.map_kernel_and_physical_memory();
        memory_set
    }
    pub fn map_kernel_and_physical_memory(&mut self) {
        extern "C" {
            fn stext();
            fn etext();
            fn srodata();
            fn erodata();
            fn sdata();
            fn edata();
            fn sbss();
            fn ebss();
            fn end();
        }
        let offset = PHYSICAL_MEMORY_OFFSET;
        // .text read-only & executable
        self.push(
            stext as usize,
            etext as usize,
            MemoryAttr::new().set_readonly().set_execute(),
            Linear::new(offset),
        );
        // .rodata, read-only
        self.push(
            srodata as usize,
            erodata as usize,
            MemoryAttr::new().set_readonly(),
            Linear::new(offset),
        );
        // .data, readable & writable
        self.push(
            sdata as usize,
            edata as usize,
            MemoryAttr::new(),
            Linear::new(offset)
        );
        // .bss, readable & writable
        self.push(
            sbss as usize,
            ebss as usize,
            MemoryAttr::new(),
            Linear::new(offset)
        );
        // physical memory, readable & writable
        self.push(
            (end as usize / PAGE_SIZE + 1) * PAGE_SIZE, 
            access_pa_via_va(PHYSICAL_MEMORY_END),
            MemoryAttr::new(),
            Linear::new(offset),
        );
    }
}
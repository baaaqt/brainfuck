use std::ops::{Index, IndexMut};

/// Minimal memory unit
pub type Cell = u8;

pub trait Memory: Index<usize, Output = Cell> + IndexMut<usize> {}

const PAGE_SIZE: usize = 1024;

struct PageTableIndex {
    page_index: usize,
    local_index: usize,
}

impl From<(usize, usize)> for PageTableIndex {
    fn from((page_index, local_index): (usize, usize)) -> Self {
        Self {
            page_index,
            local_index,
        }
    }
}

impl Into<(usize, usize)> for PageTableIndex {
    fn into(self) -> (usize, usize) {
        (self.page_index, self.local_index)
    }
}

pub struct PageTableMemory {
    pages: Vec<Option<Box<Vec<Cell>>>>,
}

impl PageTableMemory {
    pub fn new(init_pages: usize) -> Self {
        Self {
            pages: vec![None; init_pages],
        }
    }

    fn to_index(&self, index: usize) -> PageTableIndex {
        (index / PAGE_SIZE, index % PAGE_SIZE).into()
    }

    fn ensure_page(&mut self, index: &PageTableIndex) -> &mut Vec<Cell> {
        if index.page_index >= self.pages.len() {
            self.pages.resize_with(index.page_index + 1, || None);
        }
        self.pages[index.page_index].get_or_insert_with(|| Box::new(vec![0; PAGE_SIZE]))
    }
}

impl Index<usize> for PageTableMemory {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        let index = self.to_index(index);
        if index.page_index >= self.pages.len() {
            return &0;
        }

        if let Some(page) = &self.pages[index.page_index].as_ref() {
            &page[index.local_index]
        } else {
            &0
        }
    }
}

impl IndexMut<usize> for PageTableMemory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = self.to_index(index);
        let page = self.ensure_page(&index);
        &mut page[index.local_index]
    }
}

impl Memory for PageTableMemory {}

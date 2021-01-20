use crate::consts::MAX_PHYSICAL_PAGES;

pub struct SegmentTreeAllocator {
    a: [u8; MAX_PHYSICAL_PAGES << 1], // table and its tree
    m: usize, // the container length of table, also the begin index of table
    n: usize, // the really length of table
    offset: usize, // the offset for physical address
}

impl SegmentTreeAllocator {
    // {l: accessible, r: inaccessible} the frames' range
    pub fn init(&mut self, l: usize, r: usize) {
        // initialization for basic arguments
        self.offset = l;
        self.n = r - l;
        self.m = 1;
        while self.m < self.n {
            self.m = self.m << 1;
        }
        if self.m < MAX_PHYSICAL_PAGES {
            println!("self.m = {}, self.n= {}, MAX_PHYSICAL_PAGES = {}",
                     self.m, self.n, MAX_PHYSICAL_PAGES);
            panic!("self.m is too long for MAX_PHYSICAL_PAGES");
        }

        // initialization for array `a`
        // 1. set all accessible leaf node to 0(accessible)
        for i in self.m..(self.m + self.n) { self.a[i] = 0; }
        // 2. set all inaccessible leaf nodes to 1(inaccessible)
        for i in (self.m + self.n)..(self.m << 1) { self.a[i] = 1; }
        // 3. set all the nonleaf node
        for i in (1..self.m).rev() {
            self.a[i] = self.a[i << 1] & self.a[(i << 1) | 1];
        }
    }

    // return a alloced address
    pub fn alloc(&mut self) -> usize {
        if self.a[1] == 1 {
            panic!("physical memory depleted!");
        }

        // make the p point the first accessible frame
        let mut p = 1;
        while p < self.m {
            p = if self.a[p << 1] == 0 {p << 1} else {(p << 1) | 1};
        }

        // edit the tree before return the result
        self.a[p] = 1;
        p >>= 1;
        while p > 0 {
            self.a[p] = self.a[p << 1] & self.a[(p << 1) | 1];
        }

        // return the address of the aim frame
        p + self.offset - self.m
    }

    // free memory by address
    pub fn dealloc(&mut self, n: usize) {
        // set the frame accessible by `u`
        let mut p = n - self.offset + self.m;
        assert!(self.a[p] == 1);
        self.a[p] = 0;

        // edit the tree.
        p >>= 1;
        while p > 0 {
            self.a[p] = self.a[p << 1] & self.a[(p << 1) | 1];
            p >>= 1;
        }
    }
}

use spin::Mutex;

pub static SEGMENT_TREE_ALLOCATOR: Mutex<SegmentTreeAllocator> = Mutex::new(
    SegmentTreeAllocator{
        a: [0; MAX_PHYSICAL_PAGES << 1],
        m: 0,
        n: 0,
        offset: 0
    });


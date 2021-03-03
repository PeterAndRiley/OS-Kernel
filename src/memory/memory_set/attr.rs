use crate::memory::paging::PageEntry;

#[derive(Clone,Debug, Default)]
pub struct MemoryAttr {
    user : bool,     // Is user mode accessible to
    readonly : bool, // Is read only
    execute : bool   // Is executable
}

impl MemoryAttr {
    // By default, user mode is inaccessible ; writeable; not executable
    pub fn new() -> Self{
        MemoryAttr {
            user : false,
            readonly : false,
            execute : false,
        }
    }
    // Visitors to modify the permissions
    pub fn set_user(mut self) -> Self {
        self.user = true;
        self
    }
    pub fn set_readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
    pub fn set_execute(mut self) -> Self {
        self.execute = true;
        self
    }
    // Modify page table items according to 
    // the set permission requirements
    pub fn apply(&self, entry : &mut PageEntry) {
        entry.set_present(true);
        entry.set_user(self.user);
        entry.set_writable(!self.readonly);
        entry.set_execute(self.execute);
    }
}
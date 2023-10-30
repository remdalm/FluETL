#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Id(u32);

impl Id {
    pub fn new(id: u32) -> Self {
        Id(id)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

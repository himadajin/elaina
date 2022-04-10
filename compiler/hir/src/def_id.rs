#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DefId(usize);

impl DefId {
    pub fn from_usize(id: usize) -> DefId {
        DefId(id)
    }
}

pub struct DefIdGen(usize);

impl DefIdGen {
    pub fn new() -> DefIdGen {
        Self(0)
    }

    pub fn new_id(&mut self) -> DefId {
        let id = DefId::from_usize(self.0);
        self.0 += 1;

        id
    }
}

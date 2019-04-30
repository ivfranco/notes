pub trait Resource {
    fn empty() -> Self;
    fn fit_in(&self, other: &Self) -> bool;
    fn add(&self, other: &Self) -> Self;
}

#[derive(Clone, Copy)]
pub struct AluMem {
    alu: u32,
    mem: u32,
}

impl AluMem {
    pub fn new(alu: u32, mem: u32) -> Self {
        AluMem { alu, mem }
    }
}

impl Resource for AluMem {
    fn empty() -> Self {
        AluMem { alu: 0, mem: 0 }
    }

    fn fit_in(&self, other: &Self) -> bool {
        self.alu <= other.alu && self.mem <= other.mem
    }

    fn add(&self, other: &Self) -> Self {
        AluMem {
            alu: self.alu + other.alu,
            mem: self.mem + other.mem,
        }
    }
}

impl std::fmt::Debug for AluMem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} ALU resources, {} MEM resources", self.alu, self.mem)
    }
}

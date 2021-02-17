use crate::{Attrs, DepWithNames, NameRegister};

pub struct MVD {
    pub source: Attrs,
    pub target: Attrs,
}

impl MVD {
    pub fn new(source: Attrs, target: Attrs) -> Self {
        let target = &target - &source;
        Self { source, target }
    }

    pub fn with_names<'a>(&'a self, register: &'a NameRegister) -> DepWithNames<'a> {
        DepWithNames {
            arrow: "->>",
            source: &self.source,
            target: &self.target,
            register,
        }
    }

    pub fn is_deformed(&self) -> bool {
        self.source.is_empty() || self.target.is_empty()
    }
}

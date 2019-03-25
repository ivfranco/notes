use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

type SharedObject = Rc<RefCell<Object>>;

#[derive(Clone)]
struct Object {
    name: String,
    ref_count: usize,
    referee: Vec<SharedObject>,
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}: {}", self.name, self.ref_count)
    }
}

impl Object {
    fn new(name: String, ref_count: usize) -> Rc<RefCell<Self>> {
        let object = Object {
            name,
            ref_count,
            referee: vec![],
        };

        Rc::new(RefCell::new(object))
    }

    fn decrement_counter(&mut self) {
        if let Some(c) = self.ref_count.checked_sub(1) {
            self.ref_count = c;
            if c == 0 {
                self.propagate();
            }
        }
    }

    fn delete(&mut self) {
        self.ref_count = 0;
        self.propagate();
    }

    fn propagate(&mut self) {
        for r in self.referee.drain(..) {
            r.borrow_mut().decrement_counter();
        }
    }

    fn refer(&mut self, other: SharedObject) {
        other.borrow_mut().ref_count += 1;
        self.referee.push(other);
    }

    fn deref(&mut self, name: &str) {
        if let Some((i, object)) = self
            .referee
            .iter()
            .enumerate()
            .find(|(_, object)| object.borrow().name == name)
        {
            object.borrow_mut().decrement_counter();
            self.referee.remove(i);
        }
    }
}

#[derive(Clone, Default)]
pub struct Network {
    objects: HashMap<String, SharedObject>,
}

impl Network {
    pub fn new() -> Self {
        Network::default()
    }

    pub fn create_object(&mut self, name: &str, count: usize) {
        let object = Object::new(name.to_owned(), count);
        self.objects.insert(name.to_owned(), object);
    }

    pub fn create_reference(&mut self, source: &str, dest: &str) {
        let s = &self.objects[source];
        let t = &self.objects[dest];

        s.borrow_mut().refer(t.clone());
    }

    pub fn remove_reference(&mut self, source: &str, dest: &str) {
        let s = &self.objects[source];
        s.borrow_mut().deref(dest);
    }

    pub fn remove_object(&mut self, name: &str) {
        for object in self.objects.values() {
            object.borrow_mut().deref(name);
        }
        let object = &self.objects[name];
        object.borrow_mut().delete();
    }
}

impl Debug for Network {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut pairs: Vec<_> = self.objects.iter().collect();
        pairs.sort_by_key(|(name, _)| name.as_str());

        for (_, object) in pairs {
            object.borrow().fmt(f)?;
            writeln!(f, "")?;
        }

        Ok(())
    }
}

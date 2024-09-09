use crate::object::Object;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Environment {
    store: Rc<RefCell<HashMap<String, Object>>>,
}

impl std::default::Default for Environment {
    fn default() -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

impl Environment {
    pub fn new_rc() -> Rc<Environment> {
        Rc::new(Self::default())
    }

    pub(super) fn get(&self, name: impl AsRef<str>) -> Object {
        self.store
            .borrow()
            .get(name.as_ref())
            .unwrap_or(&Object::Null)
            .to_owned()
    }

    pub(super) fn set(&self, name: impl ToString, value: Object) -> Object {
        self.store
            .borrow_mut()
            .insert(name.to_string(), value.clone());
        value
    }
}

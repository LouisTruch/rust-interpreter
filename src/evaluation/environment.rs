use crate::object::Object;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    store: Rc<RefCell<HashMap<String, Object>>>,
    outer: Option<Rc<Environment>>,
}

impl std::default::Default for Environment {
    fn default() -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new())),
            outer: None,
        }
    }
}

impl Environment {
    pub fn new_rc() -> Rc<Environment> {
        Rc::new(Self::default())
    }

    pub(super) fn with_outer(outer: Rc<Environment>) -> Rc<Environment> {
        Rc::new(Self {
            outer: Some(outer),
            ..Default::default()
        })
    }

    pub(super) fn get(&self, name: impl AsRef<str>) -> Object {
        self.store
            .borrow()
            .get(name.as_ref())
            .unwrap_or({
                &match self.outer.clone() {
                    Some(outer) => outer.get(name),
                    None => Object::Null,
                }
            })
            .to_owned()
    }

    pub(super) fn set(&self, name: impl ToString, value: Object) -> Object {
        self.store
            .borrow_mut()
            .insert(name.to_string(), value.clone());
        value
    }
}

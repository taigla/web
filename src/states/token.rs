use std::{rc::Rc, fmt::Debug};
use dioxus::prelude::*;
use web_sys::window;

#[derive(Clone)]
pub struct Token(pub Rc<RefCell<String>>);

impl Token {
    pub fn set(&self, value: &str) {
        *self.0.borrow_mut() = value.to_string()
    }

    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }

    pub fn get<'a>(&'a self) -> String {
        (*self.0.borrow()).clone()
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token ({})", *self.0.borrow())
    }
}

impl Default for Token {
    fn default() -> Self {
        let window = window().unwrap();
        let token = window.local_storage().unwrap().unwrap().get("token").unwrap();
        if let Some(token) = token {
            Self(Rc::new(RefCell::new(token)))
        } else {
            Self(Rc::new(RefCell::new("".to_string())))
        }
    }
}

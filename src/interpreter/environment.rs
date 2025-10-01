use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::parser::Value;

#[derive(Debug)]
pub struct Environment {
  variables: RefCell<HashMap<String, Value>>,
  enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      variables: RefCell::new(HashMap::new()),
      enclosing: None,
    }
  }

  pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Environment {
    Environment {
      variables: RefCell::new(HashMap::new()),
      enclosing: Some(enclosing),
    }
  }

  pub fn enclosing(&self) -> Rc<RefCell<Environment>> {
    if let Some(enclosing) = &self.enclosing {
      Rc::clone(enclosing)
    } else {
      panic!("expected enclosing to exist");
    }
  }

  pub fn declare(&mut self, variable: String, value: Value) {
    self.variables.borrow_mut().insert(variable, value);
  }

  pub fn get_at_depth(&self, depth: usize, variable: &String) -> Value {
    if depth == 0 {
      self
        .variables
        .borrow()
        .get(variable)
        .map(|v| v.clone())
        .expect("expected env at depth to contain reference")
    } else {
      self
        .enclosing
        .as_ref()
        .expect("expected env at depth to exist")
        .borrow()
        .get_at_depth(depth - 1, variable)
    }
  }

  pub fn assign_at_depth(&mut self, depth: usize, variable: &String, value: &Value) -> Option<()> {
    if depth == 0 {
      self
        .variables
        .borrow_mut()
        .get_mut(variable)
        .and_then(|v| {
          *v = value.clone();
          Some(())
        })
        .expect("expected env at depth to contain reference");
      Some(())
    } else {
      self
        .enclosing
        .as_ref()
        .expect("expected env at depth to exist")
        .borrow_mut()
        .assign_at_depth(depth - 1, variable, value)
    }
  }
}

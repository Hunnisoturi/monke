use std::fmt::{Display, Formatter, Result};

pub struct ObjectType(String);

pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null,
}

pub struct Integer {
    pub value: i64,
}

pub struct Boolean {
    pub value: bool,
}

pub struct Null;

impl Integer {
    pub fn inspect(&self) -> String {
        self.value.to_string()
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Boolean {
    pub fn inspect(&self) -> String {
        self.value.to_string()
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.value)
    }
}

impl Null {
    pub fn inspect(&self) -> String {
        "null".to_string()
    }
}

impl Object {
    pub fn object_type(&self) -> ObjectType {
        match self {
            Object::Integer(_) => ObjectType("INTEGER".to_string()),
            Object::Boolean(_) => ObjectType("BOOLEAN".to_string()),
            Object::Null => ObjectType("NULL".to_string()),
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => format!("{}", value),
            Object::Boolean(value) => format!("{}", value),
            Object::Null => "null".to_string(),
        }
    }
}

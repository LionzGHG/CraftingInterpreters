use std::{any::Any, fmt::{Debug, Display}, panic::{AssertUnwindSafe, UnwindSafe}};

pub mod error;

#[derive(Clone, Debug)]
pub enum Value {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl Value {
    pub fn is_i32(&self) -> bool {
        if let Value::Integer(..) = self {
            return true;
        }  
        return false;
    }
    pub fn is_f64(&self) -> bool {
        if let Value::Float(..) = self { return true };
        return false;
    }
    pub fn is_boolean(&self) -> bool {
        if let Value::Boolean(..) = self { return true };
        return false;
    }
    pub fn is_string(&self) -> bool {
        if let Value::String(..) = self { return true };
        return false;
    }
}

impl core::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(fl) => write!(f, "{fl}"),
            Self::Integer(i) => write!(f, "{i}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Null => write!(f, "null"),
        }
    }
} 

#[deprecated]
pub trait Object: Any + Debug {
    fn clone_box(&self) -> Box<dyn Object>;
    fn as_string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn as_f64(&self) -> Option<f64> {
        None
    }
}

#[deprecated]
#[derive(Debug, Clone)]
pub struct Number(pub f64);

impl Number {
    pub fn as_f64(&self) -> f64 {
        return self.0;
    }
}

impl<T> Object for T
where
    T: 'static + Any + Clone + Debug,
{
    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }

    fn as_string(&self) -> String {
        format!("{:?}", self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_f64(&self) -> Option<f64> {
        if let Some(value) = downcast_obj::<f64>(self).cloned() {
            return Some(value);
        }
        None
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.as_ref().clone_box()
    }
}

impl Display for Box<dyn Object> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[deprecated]
/// Downcast's an Object to T.
pub fn downcast_obj<T: Any>(object: &dyn Object) -> Option<&T> {
    object.as_any().downcast_ref::<T>()
}

#[deprecated]
/// Downcast's an Object instance to f64.
pub fn downcast_to_f64(obj: &dyn Object) -> Option<f64> {
    let opt_opt: Option<Option<f64>> = obj.as_any().downcast_ref::<Option<Option<f64>>>().unwrap().clone();
    
    if let Some(n1) = opt_opt {
        if let Some(n) = n1 {
            return Some(n);
        }
    }

    return None;
}

#[deprecated]
/// Downcast's an Option<Option<T>> to T.
pub fn downcast_to<T: 'static + Clone>(obj: &dyn Object) -> Option<T> {
    let opt_opt: Option<Option<T>> = obj.as_any().downcast_ref::<Option<Option<T>>>().expect("Expected Option<Optiony<T>>").clone();

    if let Some(n1) = opt_opt {
        if let Some(n) = n1 {
            return Some(n);
        }
    }

    return None;
}
use std::{any::Any, fmt::{Debug, Display}};

pub trait Object: Any + Debug {
    fn clone_box(&self) -> Box<dyn Object>;
    fn as_string(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn as_f64(&self) -> Option<f64>;
    //fn downcast_ref<R: Any>(&self) -> Option<&R>;
}

impl<T> Object for T
where
    T: 'static + Any + Clone + Debug, 
{
    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }

    fn as_string(&self) -> String {
        format!("{self:?}")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_f64(&self) -> Option<f64> {
        if let Some(value) = downcast_obj::<f64>(self).cloned() {
            return Some(value);
        }
        return None;
    }
    
    /*fn downcast_ref<R: Any>(&self) -> Option<&R> {
        self.as_any().downcast_ref::<R>()
    }*/
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

pub fn downcast_obj<T: Any>(object: &dyn Object) -> Option<&T> {
    object.as_any().downcast_ref::<T>()
}

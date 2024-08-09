use std::{
    any::{Any, TypeId},
    cell::RefCell,
    rc::Rc,
    sync::Arc,
};

use hashbrown::HashMap;

trait ArcDowncaster {
    fn downcast<T: 'static + Any>(self: Arc<Self>) -> Result<Arc<T>, Arc<Self>>;
}

impl ArcDowncaster for dyn Any {
    fn downcast<T: 'static + Any>(self: Arc<Self>) -> Result<Arc<T>, Arc<Self>> {
        if self.is::<T>() {
            let ptr = Arc::into_raw(self) as *const T;
            Ok(unsafe { Arc::from_raw(ptr) })
        } else {
            Err(self)
        }
    }
}

pub struct ServiceGroup(Rc<RefCell<HashMap<TypeId, Arc<dyn Any>>>>);

impl ServiceGroup {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(HashMap::new())))
    }

    pub fn has<T: 'static + Any>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.0.borrow().contains_key(&type_id)
    }

    pub fn set<T: 'static + Any>(&self, service: T) {
        let type_id = TypeId::of::<T>();
        self.0.borrow_mut().insert(type_id, Arc::new(service));
    }

    pub fn get<T: 'static + Any>(&self) -> Arc<T> {
        let type_id = TypeId::of::<T>();
        self.0
            .borrow()
            .get(&type_id)
            .expect("Service not found")
            .clone()
            .downcast::<T>()
            .expect("Failed to downcast service")
    }
}

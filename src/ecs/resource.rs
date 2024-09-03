use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Debug, Default)]
pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn add(&mut self, data: impl Any) {
        let type_id = data.type_id();
        self.data.insert(type_id, Box::new(data));
    }

    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get(&type_id) {
            data.downcast_ref()
        } else {
            None
        }
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get_mut(&type_id) {
            data.downcast_mut()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_resource() {
        let resources = initialize_resource();

        let stored_resource = resources.data.get(&TypeId::of::<WorldWidth>()).unwrap();
        let world_width = stored_resource.downcast_ref::<WorldWidth>().unwrap();
        assert_eq!(world_width.0, 100.0);
    }

    #[test]
    fn get_resource() {
        let resources = initialize_resource();

        if let Some(world_width) = resources.get_ref::<WorldWidth>() {
            assert_eq!(world_width.0, 100.0);
        }
    }

    #[test]
    fn get_resource_mut() {
        let mut resources = initialize_resource();
        {
            let world_width = resources.get_mut::<WorldWidth>().unwrap();
            world_width.0 += 1.0;
        }
        let world_width = resources.get_ref::<WorldWidth>().unwrap();
        assert_eq!(world_width.0, 101.0);
    }

    fn initialize_resource() -> Resources {
        let mut resources = Resources::default();
        let world_width = WorldWidth(100.0);

        resources.add(world_width);
        resources
    }

    struct WorldWidth(pub f32);
}

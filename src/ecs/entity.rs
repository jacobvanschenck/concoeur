pub mod query;

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

pub type Component = Rc<RefCell<dyn Any>>;
pub type Components = HashMap<TypeId, Vec<Option<Component>>>;

#[derive(Debug, Default)]
pub struct Entities {
    components: Components,
    bit_masks: HashMap<TypeId, u32>,
    map: Vec<u32>,
    inserting_into_index: usize,
}

impl Entities {
    pub fn register_component<T: Any + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, vec![]);
        self.bit_masks.insert(type_id, 1 << self.bit_masks.len());
    }

    pub fn create_entity(&mut self) -> &mut Self {
        if let Some((index, _)) = self
            .map
            .iter()
            .enumerate()
            .find(|(_index, mask)| **mask == 0)
        {
            self.inserting_into_index = index;
        } else {
            self.components
                .iter_mut()
                .for_each(|(_key, components)| components.push(None));
            self.map.push(0);
            self.inserting_into_index = self.map.len() - 1;
        }
        self
    }

    pub fn with_component(&mut self, data: impl Any) -> Result<&mut Self, &'static str> {
        let type_id = data.type_id();
        let index = self.inserting_into_index;
        if let Some(components) = self.components.get_mut(&type_id) {
            let component = components
                .get_mut(index)
                .ok_or("Create component never called")?;
            *component = Some(Rc::new(RefCell::new(data)));
            let bitmask = self.bit_masks.get(&type_id).unwrap();
            self.map[index] |= *bitmask;
        } else {
            return Err("Component not registered");
        }
        Ok(self)
    }

    pub fn get_bitmask(&self, type_id: &TypeId) -> Option<u32> {
        self.bit_masks.get(&type_id).copied()
    }
}

#[cfg(test)]
mod test {

    use std::any::TypeId;

    use super::Entities;

    struct Health(pub u32);
    struct Speed(pub u32);

    #[test]
    fn register_an_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let type_id = TypeId::of::<Health>();
        let health_components = entities.components.get(&type_id).unwrap();
        assert_eq!(health_components.len(), 0)
    }

    #[test]
    fn bitmask_updated_when_registering_entities() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let type_id = TypeId::of::<Health>();
        let mask = entities.bit_masks.get(&type_id).unwrap();
        assert_eq!(*mask, 0b0000_0001);

        entities.register_component::<Speed>();
        let type_id = TypeId::of::<Speed>();
        let mask = entities.bit_masks.get(&type_id).unwrap();
        assert_eq!(*mask, 0b0000_0010);
    }

    #[test]
    fn create_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities.create_entity();
        entities.create_entity();
        entities.create_entity();
        let health_components = entities.components.get(&TypeId::of::<Health>()).unwrap();
        let speed_components = entities.components.get(&TypeId::of::<Speed>()).unwrap();

        assert!(health_components.len() == speed_components.len() && health_components.len() == 1);
        assert!(health_components[0].is_none() && speed_components[0].is_none());
    }

    #[test]
    fn with_component() -> Result<(), &'static str> {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Speed(50))?;

        let first_health = &entities.components.get(&TypeId::of::<Health>()).unwrap()[0];
        let wrapped_health = first_health.as_ref().unwrap();
        let borrowed_health = wrapped_health.borrow();
        let health = borrowed_health.downcast_ref::<Health>().unwrap();
        assert_eq!(health.0, 100);

        let first_speed = &entities.components.get(&TypeId::of::<Speed>()).unwrap()[0];
        let wrapped_speed = first_speed.as_ref().unwrap();
        let borrowed_speed = wrapped_speed.borrow();
        let speed = borrowed_speed.downcast_ref::<Speed>().unwrap();
        assert_eq!(speed.0, 50);

        Ok(())
    }

    #[test]
    fn map_is_updated_when_creating_entities() -> Result<(), &'static str> {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        entities.register_component::<Speed>();
        entities
            .create_entity()
            .with_component(Health(100))?
            .with_component(Speed(15))?;
        let entity_map = entities.map[0];
        assert_eq!(entity_map, 3);

        entities.create_entity().with_component(Speed(15))?;
        let entity_map = entities.map[1];
        assert_eq!(entity_map, 2);

        entities.create_entity().with_component(Health(40))?;
        let entity_map = entities.map[2];
        assert_eq!(entity_map, 1);

        Ok(())
    }
}

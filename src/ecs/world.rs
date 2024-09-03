use std::any::Any;

use super::entity::query::Query;
use super::entity::Entities;
use super::resource::Resources;

#[derive(Debug, Default)]
pub struct World {
    entities: Entities,
    resources: Resources,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    pub fn register_component<T: Any + 'static>(&mut self) {
        self.entities.register_component::<T>();
    }

    pub fn create_entity(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }

    pub fn query(&self) -> Query {
        Query::new(&self.entities)
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn create_and_get_resource_immutably() {
        let world = initialize_world();
        let fps = world.get_resource::<FpsResource>().unwrap();
        assert_eq!(fps.0, 60)
    }

    #[test]
    fn get_resources_mutably() {
        let mut world = initialize_world();
        {
            let fps: &mut FpsResource = world.get_resource_mut::<FpsResource>().unwrap();
            fps.0 += 1;
        }
        let fps = world.get_resource::<FpsResource>().unwrap();
        assert_eq!(fps.0, 61);
    }

    #[test]
    fn create_entity() -> Result<(), &'static str> {
        let mut world = World::new();
        world.register_component::<Location>();
        world.register_component::<Size>();

        world
            .create_entity()
            .with_component(Location(42.0, 24.0))?
            .with_component(Size(10.0))?;
        Ok(())
    }

    fn initialize_world() -> World {
        let mut world = World::new();
        world.add_resource(FpsResource(60));
        world
    }

    #[test]
    fn query_for_entities() -> Result<(), &'static str> {
        let mut world = World::new();
        world.register_component::<Location>();
        world.register_component::<Size>();

        world
            .create_entity()
            .with_component(Location(42.0, 24.0))?
            .with_component(Size(10.0))?;

        world.create_entity().with_component(Size(11.0))?;

        world.create_entity().with_component(Location(43.0, 25.0))?;

        world
            .create_entity()
            .with_component(Location(44.0, 26.0))?
            .with_component(Size(12.0))?;

        let query = world
            .query()
            .with_component::<Location>()?
            .with_component::<Size>()?
            .run();

        let locations: &Vec<Rc<RefCell<dyn Any>>> = &query.1[0];
        let sizes: &Vec<Rc<RefCell<dyn Any>>> = &query.1[1];

        assert_eq!(locations.len(), sizes.len());
        assert_eq!(locations.len(), 2);

        let borrowed_first_location = locations[0].borrow();
        let first_location = borrowed_first_location.downcast_ref::<Location>().unwrap();
        assert_eq!(first_location.0, 42.0);
        assert_eq!(first_location.1, 24.0);
        let borrowed_first_size = sizes[0].borrow();
        let first_size = borrowed_first_size.downcast_ref::<Size>().unwrap();
        assert_eq!(first_size.0, 10.0);

        let borrowed_second_location = locations[1].borrow();
        let second_location = borrowed_second_location.downcast_ref::<Location>().unwrap();
        assert_eq!(second_location.0, 44.0);
        assert_eq!(second_location.1, 26.0);
        let mut borrowed_second_size = sizes[1].borrow_mut();
        let second_size = borrowed_second_size.downcast_mut::<Size>().unwrap();
        second_size.0 += 1.0;
        assert_eq!(second_size.0, 13.0);

        Ok(())
    }

    #[derive(Debug)]
    struct FpsResource(pub u32);

    struct Location(pub f32, pub f32);
    struct Size(pub f32);

    impl std::ops::Deref for FpsResource {
        type Target = u32;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

use std::{
    any::{Any, TypeId},
    cell::{Ref, RefMut},
};

use super::{Component, Entities};

pub type QueryIndexes = Vec<usize>;
pub type QueryComponents = Vec<Vec<Component>>;

#[derive(Debug)]
pub struct QueryEntity<'a> {
    id: usize,
    entities: &'a Entities,
}

impl<'a> QueryEntity<'a> {
    pub fn new(id: usize, entities: &'a Entities) -> Self {
        QueryEntity { id, entities }
    }

    pub fn get_component<T: Any>(&self) -> Result<Ref<T>, &'static str> {
        let type_id = TypeId::of::<T>();
        let components = self
            .entities
            .components
            .get(&type_id)
            .ok_or("Component not registered")?;
        let borrow_component = components[self.id]
            .as_ref()
            .ok_or("Component not found")?
            .borrow();

        Ok(Ref::map(borrow_component, |any| {
            any.downcast_ref::<T>().unwrap()
        }))
    }

    pub fn get_component_mut<T: Any>(&self) -> Result<RefMut<T>, &'static str> {
        let type_id = TypeId::of::<T>();
        let components = self
            .entities
            .components
            .get(&type_id)
            .ok_or("Component not registered")?;
        let borrow_component = components[self.id]
            .as_ref()
            .ok_or("Component not found")?
            .borrow_mut();

        Ok(RefMut::map(borrow_component, |any| {
            any.downcast_mut::<T>().unwrap()
        }))
    }
}

#[derive(Debug)]
pub struct Query<'a> {
    map: u32,
    entities: &'a Entities,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {
    pub fn new(entities: &'a Entities) -> Self {
        Self {
            entities,
            map: 0,
            type_ids: vec![],
        }
    }

    pub fn with_component<T: Any>(&mut self) -> Result<&mut Self, &'static str> {
        let type_id = TypeId::of::<T>();
        if let Some(bit_mask) = self.entities.get_bitmask(&type_id) {
            self.map |= bit_mask;
            self.type_ids.push(type_id);
        } else {
            return Err("Component not registered");
        }
        Ok(self)
    }

    pub fn run(&self) -> (QueryIndexes, QueryComponents) {
        let indexes: Vec<usize> = self
            .entities
            .map
            .iter()
            .enumerate()
            .filter_map(|(index, entity_map)| {
                if entity_map & self.map == self.map {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();

        let mut result = vec![];

        for type_id in &self.type_ids {
            let entity_components = self.entities.components.get(type_id).unwrap();
            let mut components_to_keep = vec![];
            for index in &indexes {
                // components_to_keep.push(entity_components[*index].as_ref().unwrap().clone());
                components_to_keep.push(entity_components[*index].as_ref().unwrap().clone());
            }
            result.push(components_to_keep);
        }

        (indexes, result)
    }

    pub fn run_query(&self) -> Vec<QueryEntity> {
        self.entities
            .map
            .iter()
            .enumerate()
            .filter_map(|(index, entity_map)| {
                if entity_map & self.map == self.map {
                    Some(QueryEntity::new(index, self.entities))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn query_mask_updating_with_component() -> Result<(), &'static str> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();
        let mut query = Query::new(&entities);
        query.with_component::<u32>()?.with_component::<f32>()?;

        assert_eq!(query.map, 3);
        assert_eq!(TypeId::of::<u32>(), query.type_ids[0]);
        assert_eq!(TypeId::of::<f32>(), query.type_ids[1]);

        Ok(())
    }

    #[test]
    fn run_query() -> Result<(), &'static str> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();
        entities
            .create_entity()
            .with_component(10_u32)?
            .with_component(20.0_f32)?;
        entities.create_entity().with_component(5_u32)?;
        entities.create_entity().with_component(50.0_f32)?;
        entities
            .create_entity()
            .with_component(15_u32)?
            .with_component(25.0_f32)?;
        let mut query = Query::new(&entities);
        query.with_component::<u32>()?.with_component::<f32>()?;

        let query_result = query.run();
        let u32s = &query_result.1[0];
        let f32s = &query_result.1[1];
        let indexes = &query_result.0;

        assert!(u32s.len() == f32s.len() && u32s.len() == indexes.len());
        assert_eq!(u32s.len(), 2);

        let borrowed_first_u32 = u32s[0].borrow();
        let first_u32 = borrowed_first_u32.downcast_ref::<u32>().unwrap();
        assert_eq!(*first_u32, 10);

        let borrowed_first_f32 = f32s[0].borrow();
        let first_f32 = borrowed_first_f32.downcast_ref::<f32>().unwrap();
        assert_eq!(*first_f32, 20.0);

        let borrowed_second_u32 = u32s[1].borrow();
        let second_u32 = borrowed_second_u32.downcast_ref::<u32>().unwrap();
        assert_eq!(*second_u32, 15);

        let borrowed_second_f32 = f32s[1].borrow();
        let second_f32 = borrowed_second_f32.downcast_ref::<f32>().unwrap();
        assert_eq!(*second_f32, 25.0);

        assert_eq!(indexes[0], 0);
        assert_eq!(indexes[1], 3);

        Ok(())
    }

    #[test]
    fn run_entity_query_ref() -> Result<(), &'static str> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();

        entities.create_entity().with_component(5_u32)?;
        entities.create_entity().with_component(50.0_f32)?;

        let mut query = Query::new(&entities);

        let query_entities = query.with_component::<u32>()?.run_query();

        assert_eq!(query_entities.len(), 1);

        for entity in query_entities {
            assert_eq!(entity.id, 0);
            let health = entity.get_component::<u32>()?;
            assert_eq!(*health, 5);
        }

        Ok(())
    }

    #[test]
    fn run_entity_query_mut() -> Result<(), &'static str> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();

        entities.create_entity().with_component(5_u32)?;
        entities.create_entity().with_component(50.0_f32)?;

        let mut query = Query::new(&entities);

        let query_entities = query.with_component::<u32>()?.run_query();

        assert_eq!(query_entities.len(), 1);

        for entity in query_entities {
            assert_eq!(entity.id, 0);
            let mut health = entity.get_component_mut::<u32>()?;
            assert_eq!(*health, 5);
            *health *= 10;
        }

        let query_entities = query.with_component::<u32>()?.run_query();

        for entity in query_entities {
            assert_eq!(entity.id, 0);
            let health = entity.get_component::<u32>()?;
            assert_eq!(*health, 50);
        }

        Ok(())
    }
}

use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::traits::Resource;

/// A map of resources.
///
/// This map is used to store resources that can be shared among systems.
#[derive(Debug)]
pub(crate) struct ResMap {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl ResMap {
    /// Creates a new `ResMap`.
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Inserts a resource into the map.
    pub(crate) fn insert<T: Resource>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }

    /// Gets a resource from the map.
    pub(crate) fn get<T: Resource>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Gets a mutable reference to a resource from the map.
    pub(crate) fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    /// Removes a resource from the map.
    pub(crate) fn remove<T: Resource>(&mut self) {
        self.map.remove(&TypeId::of::<T>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug)]
    struct TestResource {
        value: i32,
    }

    #[derive(PartialEq, Debug)]
    struct TestResource2 {
        value: i32,
    }

    impl Resource for TestResource {}
    impl Resource for TestResource2 {}

    #[test]
    fn test_res_map() {
        let mut res_map = ResMap::new();

        res_map.insert(TestResource { value: 1 });
        res_map.insert(TestResource2 { value: 2 });

        assert_eq!(
            res_map.get::<TestResource>(),
            Some(&TestResource { value: 1 })
        );
        assert_eq!(
            res_map.get_mut::<TestResource>(),
            Some(&mut TestResource { value: 1 })
        );

        assert_eq!(
            res_map.get::<TestResource2>(),
            Some(&TestResource2 { value: 2 })
        );
        assert_eq!(
            res_map.get_mut::<TestResource2>(),
            Some(&mut TestResource2 { value: 2 })
        );

        res_map.insert(TestResource { value: 3 });

        assert_eq!(
            res_map.get::<TestResource>(),
            Some(&TestResource { value: 3 })
        );
        assert_eq!(
            res_map.get_mut::<TestResource>(),
            Some(&mut TestResource { value: 3 })
        );

        res_map.remove::<TestResource>();
        assert_eq!(res_map.get::<TestResource>(), None);
        assert_eq!(res_map.get_mut::<TestResource>(), None);

        let res = res_map.get_mut::<TestResource2>().unwrap();
        res.value = 3;

        assert_eq!(
            res_map.get::<TestResource2>(),
            Some(&TestResource2 { value: 3 })
        );
    }
}

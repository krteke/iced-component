//! Typed runtime animation overrides shared by themed component contexts.

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    rc::Rc,
};

/// Sparse component-animation overrides indexed by their concrete provider type.
///
/// Component crates keep their defaults locally and only consult this store for
/// an application-level override. Adding a new component provider therefore
/// does not add another field or accessor to the shared context.
#[derive(Clone, Default)]
pub struct AnimationOverrides {
    entries: HashMap<TypeId, Rc<dyn Any>>,
}

impl AnimationOverrides {
    /// Creates an empty override store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces one concrete animation provider.
    pub fn set<T: Any>(&mut self, value: T) {
        self.entries.insert(TypeId::of::<T>(), Rc::new(value));
    }

    /// Returns an override by its concrete provider type.
    #[must_use]
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.entries
            .get(&TypeId::of::<T>())
            .and_then(|value| value.downcast_ref())
    }

    /// Removes an override and reports whether it existed.
    pub fn remove<T: Any>(&mut self) -> bool {
        self.entries.remove(&TypeId::of::<T>()).is_some()
    }

    /// Returns whether an override of type `T` is installed.
    #[must_use]
    pub fn contains<T: Any>(&self) -> bool {
        self.entries.contains_key(&TypeId::of::<T>())
    }

    /// Removes every installed override.
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Returns whether no animation overrides are installed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::AnimationOverrides;

    #[derive(Debug, Eq, PartialEq)]
    struct ButtonAnimations(u8);

    #[derive(Debug, Eq, PartialEq)]
    struct SurfaceAnimations(u8);

    #[test]
    fn providers_are_resolved_and_removed_by_concrete_type() {
        let mut overrides = AnimationOverrides::new();

        overrides.set(ButtonAnimations(1));
        overrides.set(SurfaceAnimations(2));

        assert_eq!(
            overrides.get::<ButtonAnimations>(),
            Some(&ButtonAnimations(1))
        );
        assert_eq!(
            overrides.get::<SurfaceAnimations>(),
            Some(&SurfaceAnimations(2))
        );
        assert!(overrides.remove::<ButtonAnimations>());
        assert!(!overrides.contains::<ButtonAnimations>());
        assert!(overrides.contains::<SurfaceAnimations>());
    }
}

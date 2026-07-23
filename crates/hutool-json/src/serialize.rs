use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use serde_json::Value;

use crate::{JsonError, Result};

/// Typed custom serializer contract.
pub trait JSONSerializer<T>: Send + Sync {
    /// Serializes one value.
    fn serialize(&self, value: &T) -> Result<Value>;
}

impl<T, F> JSONSerializer<T> for F
where
    F: Fn(&T) -> Result<Value> + Send + Sync,
{
    fn serialize(&self, value: &T) -> Result<Value> {
        self(value)
    }
}

/// Typed custom deserializer contract.
pub trait JSONDeserializer<T>: Send + Sync {
    /// Deserializes one value.
    fn deserialize(&self, value: &Value) -> Result<T>;
}

impl<T, F> JSONDeserializer<T> for F
where
    F: Fn(&Value) -> Result<T> + Send + Sync,
{
    fn deserialize(&self, value: &Value) -> Result<T> {
        self(value)
    }
}

type ErasedSerializer = Arc<dyn Fn(&dyn Any) -> Result<Value> + Send + Sync>;
type ErasedDeserializer = Arc<dyn Fn(&Value) -> Result<Box<dyn Any>> + Send + Sync>;

/// Explicitly owned custom serialization mapping.
#[derive(Clone, Default)]
pub struct SerializeRegistry {
    serializers: HashMap<TypeId, ErasedSerializer>,
    deserializers: HashMap<TypeId, ErasedDeserializer>,
}

impl std::fmt::Debug for SerializeRegistry {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SerializeRegistry")
            .field("serializers", &self.serializers.len())
            .field("deserializers", &self.deserializers.len())
            .finish()
    }
}

impl SerializeRegistry {
    /// Creates an empty mapping.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers or replaces a typed serializer.
    pub fn put_serializer<T: Any + Send + Sync>(
        &mut self,
        serializer: impl JSONSerializer<T> + 'static,
    ) -> &mut Self {
        self.serializers.insert(
            TypeId::of::<T>(),
            Arc::new(move |value| {
                serializer.serialize(
                    value
                        .downcast_ref::<T>()
                        .ok_or(JsonError::Mapping("serializer type mismatch"))?,
                )
            }),
        );
        self
    }

    /// Registers or replaces a typed deserializer.
    pub fn put_deserializer<T: Any + Send + Sync>(
        &mut self,
        deserializer: impl JSONDeserializer<T> + 'static,
    ) -> &mut Self {
        self.deserializers.insert(
            TypeId::of::<T>(),
            Arc::new(move |value| Ok(Box::new(deserializer.deserialize(value)?) as Box<dyn Any>)),
        );
        self
    }

    /// Serializes a value using its registered mapping.
    pub fn serialize<T: Any + Send + Sync>(&self, value: &T) -> Result<Value> {
        self.serializers
            .get(&TypeId::of::<T>())
            .ok_or(JsonError::Mapping("serializer not registered"))?(value)
    }

    /// Deserializes a value using its registered mapping.
    pub fn deserialize<T: Any + Send + Sync>(&self, value: &Value) -> Result<T> {
        Ok(*self
            .deserializers
            .get(&TypeId::of::<T>())
            .ok_or(JsonError::Mapping("deserializer not registered"))?(value)?
        .downcast::<T>()
        .map_err(|_| JsonError::Mapping("deserializer type mismatch"))?)
    }

    /// Returns whether no mappings are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.serializers.is_empty() && self.deserializers.is_empty()
    }

    /// Removes all mappings.
    pub fn clear(&mut self) {
        self.serializers.clear();
        self.deserializers.clear();
    }
}

/// Explicit compatibility access to Hutool's global serialization mapping.
pub struct GlobalSerializeMapping;

fn global_slot() -> &'static RwLock<SerializeRegistry> {
    static REGISTRY: OnceLock<RwLock<SerializeRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(SerializeRegistry::new()))
}

impl GlobalSerializeMapping {
    /// Returns a snapshot sharing registered closures.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the compatibility-global lock.
    #[must_use]
    pub fn get() -> SerializeRegistry {
        global_slot()
            .read()
            .expect("global JSON mapping read lock poisoned")
            .clone()
    }

    /// Replaces the compatibility global and returns its previous mapping.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the compatibility-global lock.
    pub fn set(registry: SerializeRegistry) -> SerializeRegistry {
        std::mem::replace(
            &mut *global_slot()
                .write()
                .expect("global JSON mapping write lock poisoned"),
            registry,
        )
    }

    /// Restores an empty global mapping.
    pub fn reset() -> SerializeRegistry {
        Self::set(SerializeRegistry::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Debug, PartialEq, Eq)]
    struct Identifier(u64);

    #[test]
    fn owned_registry_maps_both_directions_and_reports_missing_types() {
        let mut registry = SerializeRegistry::new();
        assert!(registry.is_empty());
        registry
            .put_serializer(|value: &Identifier| Ok(Value::String(value.0.to_string())))
            .put_deserializer(|value: &Value| {
                value
                    .as_str()
                    .ok_or(JsonError::Mapping("identifier must be a string"))?
                    .parse::<u64>()
                    .map(Identifier)
                    .map_err(|_| JsonError::Mapping("invalid identifier"))
            });
        assert_eq!(registry.serialize(&Identifier(7)).unwrap(), "7");
        assert_eq!(
            registry.deserialize::<Identifier>(&json!("8")).unwrap(),
            Identifier(8)
        );
        assert!(registry.serialize(&9_u8).is_err());
        assert!(
            SerializeRegistry::new()
                .deserialize::<Identifier>(&json!(9))
                .is_err()
        );
        assert!(registry.deserialize::<Identifier>(&json!(false)).is_err());
        let mut corrupt_serializer = SerializeRegistry::new();
        corrupt_serializer.put_serializer(|value: &u8| Ok(json!(value)));
        assert_eq!(corrupt_serializer.serialize(&1_u8).unwrap(), json!(1));
        let serializer = corrupt_serializer
            .serializers
            .remove(&TypeId::of::<u8>())
            .unwrap();
        corrupt_serializer
            .serializers
            .insert(TypeId::of::<Identifier>(), serializer);
        assert!(corrupt_serializer.serialize(&Identifier(1)).is_err());
        let mut corrupt = SerializeRegistry::new();
        corrupt.deserializers.insert(
            TypeId::of::<Identifier>(),
            Arc::new(|_| Ok(Box::new(1_u8) as Box<dyn Any>)),
        );
        assert!(corrupt.deserialize::<Identifier>(&json!(1)).is_err());
        assert!(format!("{registry:?}").contains("serializers: 1"));
        registry.clear();
        assert!(registry.is_empty());
        assert!(SerializeRegistry::default().is_empty());
    }

    #[test]
    fn compatibility_global_is_replaceable_and_resettable() {
        let mut registry = SerializeRegistry::new();
        registry.put_serializer(|value: &Identifier| Ok(json!(value.0)));
        let previous = GlobalSerializeMapping::set(registry);
        assert_eq!(
            GlobalSerializeMapping::get()
                .serialize(&Identifier(3))
                .unwrap(),
            3
        );
        let installed = GlobalSerializeMapping::set(previous);
        assert_eq!(installed.serialize(&Identifier(4)).unwrap(), 4);
        let _ = GlobalSerializeMapping::reset();
        assert!(GlobalSerializeMapping::get().is_empty());
    }
}

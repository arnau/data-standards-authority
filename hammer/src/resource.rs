//! This module defines a generic resource to be implemented to interface with the cache storage.

use anyhow::Result;

pub trait Resource<Lens: ?Sized, Item> {
    /// Composes a single resource given its id.
    fn get(&mut self, id: &str) -> Result<Option<Item>>;

    /// Inserts the given resource to the store.
    fn add(&mut self, item: &Item) -> Result<()>;

    /// Cleans a single resource and potentially any dependency given its id.
    fn prune(&mut self, id: &str) -> Result<()>;

    // /// Inserts the given collection of resources to the store.
    // fn bulk(&mut self, collection: &[Item]) -> Result<()>;

    // /// Composes the full collection of resources.
    // fn mass(&mut self) -> Result<Vec<Item>>;
}

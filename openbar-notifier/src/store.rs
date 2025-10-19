//! Item stores for OpenBar Notifier
//!
//! This module contains the item store implementation, which enables
//! tracking the state of items across multiple checks.

use openbar_api::models::Item;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Item store to track item states
///
/// Internally, the store is simply an ordered list of items.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemStore {
    items: Vec<Item>,
}

impl ItemStore {
    /// Create a new, empty ItemStore
    pub fn new() -> Self {
        ItemStore { items: Vec::new() }
    }

    /// Get a reference to the internal list of items
    pub fn items(&self) -> &Vec<Item> {
        &self.items
    }

    /// Clear the item store
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Append an item to the store
    pub fn append(&mut self, item: Item) -> bool {
        // Insert while maintaining order by item ID
        match self.items.binary_search_by_key(&item.id, |i| i.id) {
            Ok(_) => {
                // Item already exists, do not insert
                false
            }
            Err(index) => {
                // Item does not exist, insert it
                self.items.insert(index, item);
                true
            }
        }
    }

    /// Find an item by its ID
    pub fn find(&self, item_id: Uuid) -> Option<&Item> {
        // Since items are ordered, we can use binary search for efficiency
        match self.items.binary_search_by_key(&item_id, |item| item.id) {
            Ok(index) => Some(&self.items[index]),
            Err(_) => None,
        }
    }

    /// Find a mutable reference to an item by its ID
    pub fn find_mut(&mut self, item_id: Uuid) -> Option<&mut Item> {
        match self.items.binary_search_by_key(&item_id, |item| item.id) {
            Ok(index) => Some(&mut self.items[index]),
            Err(_) => None,
        }
    }

    /// Replace an item in the store by its ID
    pub fn replace(&mut self, new_item: Item) -> Result<Item, ()> {
        match self
            .items
            .binary_search_by_key(&new_item.id, |item| item.id)
        {
            Ok(index) => {
                let old_item = self.items[index].clone();
                self.items[index] = new_item;
                Ok(old_item)
            }
            Err(_) => Err(()),
        }
    }
}

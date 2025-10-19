/// ItemEvent types for OpenBar Notifier
///
/// Represents the different types of events that can occur for an item.
pub enum ItemEvent {
    /// The item has been added to the store
    Added,
    /// The item has become buyable
    BecomeBuyable,
    /// The item has become unbuyable
    BecomeUnbuyable,
    /// The item is out of stock
    OutOfStock,
}

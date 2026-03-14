#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryType {
    Equipment,
    Accessory,
    Consumable,
    Material,
    QuestItem,
    Special,
}

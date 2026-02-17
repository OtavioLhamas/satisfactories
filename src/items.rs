use std::fmt;
use strum::{AsRefStr, Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
pub enum ItemName {
    #[strum(to_string = "Iron Ore")]
    IronOre,
    #[strum(to_string = "Iron Ingot")]
    IronIngot,
    #[strum(to_string = "Water")]
    Water,
    #[strum(to_string = "Crude Oil")]
    CrudeOil,
    #[strum(to_string = "Fuel")]
    Fuel,
    #[strum(to_string = "Polymer Resin")]
    PolymerResin,
    #[strum(to_string = "Plastic")]
    Plastic,
    #[strum(to_string = "Heavy Oil Residue")]
    HeavyOilResidue,
    #[strum(to_string = "Rubber")]
    Rubber,
    #[strum(to_string = "Empty Canister")]
    EmptyCanister,
    #[strum(to_string = "Empty Tank")]
    EmptyTank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, AsRefStr)]
pub enum ItemType {
    #[strum(to_string = "Solid")]
    Solid,
    #[strum(to_string = "Fluid")]
    Fluid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item {
    pub name: ItemName,
    pub item_type: ItemType,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub const ITEM_IRON_ORE: Item = Item {
    name: ItemName::IronOre,
    item_type: ItemType::Solid,
};
pub const ITEM_IRON_INGOT: Item = Item {
    name: ItemName::IronIngot,
    item_type: ItemType::Solid,
};
pub const ITEM_WATER: Item = Item {
    name: ItemName::Water,
    item_type: ItemType::Fluid,
};
pub const ITEM_CRUDE_OIL: Item = Item {
    name: ItemName::CrudeOil,
    item_type: ItemType::Fluid,
};
pub const ITEM_FUEL: Item = Item {
    name: ItemName::Fuel,
    item_type: ItemType::Fluid,
};
pub const ITEM_POLYMER_RESIN: Item = Item {
    name: ItemName::PolymerResin,
    item_type: ItemType::Solid,
};
pub const ITEM_PLASTIC: Item = Item {
    name: ItemName::Plastic,
    item_type: ItemType::Solid,
};
pub const ITEM_HEAVY_OIL_RESIDUE: Item = Item {
    name: ItemName::HeavyOilResidue,
    item_type: ItemType::Fluid,
};
pub const ITEM_RUBBER: Item = Item {
    name: ItemName::Rubber,
    item_type: ItemType::Solid,
};

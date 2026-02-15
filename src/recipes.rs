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
pub enum RecipeName {
    #[strum(to_string = "Iron Ingot")]
    IronIngot,
    #[strum(to_string = "Pure Iron Ingot")]
    PureIronIngot,
    #[strum(to_string = "Fuel")]
    Fuel,
    #[strum(to_string = "Residual Fuel")]
    ResidualFuel,
    #[strum(to_string = "Diluted Fuel")]
    DilutedFuel,
    #[strum(to_string = "Plastic")]
    Plastic,
    #[strum(to_string = "Rubber")]
    Rubber,
}

macro_rules! define_machine {
    ($($name:ident => $display:expr, $power:expr),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Machine {
            $($name),*
        }

        impl Machine {
            pub fn base_power(&self) -> f32 {
                match self {
                    $(Machine::$name => $power),*
                }
            }

            /// Calculates power usage based on clock speed (1-250%)
            pub fn clocked_power(&self, clock: f32) -> f32 {
                self.base_power() * (clock / 100.0).powf(1.321928)
            }
        }

        impl fmt::Display for Machine {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Machine::$name => write!(f, "{}", $display)),*
                }
            }
        }
    }
}

define_machine! {
    Smelter => "Smelter", 4.0,
    Foundry => "Foundry", 16.0,
    Constructor => "Constructor", 4.0,
    Assembler => "Assembler", 15.0,
    Manufacturer => "Manufacturer", 55.0,
    Refinery => "Refinery", 30.0,
    Blender => "Blender", 75.0,
}

#[derive(Debug, Clone, Copy)]
pub struct RecipeItem {
    pub item: ItemName,
    pub amount: f32,
    pub duration: f32,
}

impl RecipeItem {
    /// Items per minute at 100% clock speed
    pub fn base_rate(&self) -> f32 {
        (60.0 * self.amount) / self.duration
    }

    /// Items per minute at the given clock speed (1-250)
    pub fn clocked_rate(&self, clock: f32) -> f32 {
        self.base_rate() * (clock / 100.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Recipe {
    pub name: RecipeName,
    pub inputs: &'static [RecipeItem],
    pub outputs: &'static [RecipeItem],
    pub duration: f32,
    pub machine: Machine,
}

macro_rules! recipe {
    ($name:ident, $recipe_name:ident, $duration:expr, $machine:ident, 
     inputs: [ $($in_item:ident: $in_amt:expr),* ], 
     outputs: [ $($out_item:ident: $out_amt:expr),* ] $(,)?) => {
        pub const $name: Recipe = Recipe {
            name: RecipeName::$recipe_name,
            duration: $duration,
            machine: Machine::$machine,
            inputs: &[
                $(RecipeItem { item: ItemName::$in_item, amount: $in_amt, duration: $duration }),*
            ],
            outputs: &[
                $(RecipeItem { item: ItemName::$out_item, amount: $out_amt, duration: $duration }),*
            ],
        };
    };
}

// Recipes defined as constants using the macro
recipe!(
    RECIPE_IRON_INGOT,
    IronIngot,
    2.0,
    Smelter,
    inputs: [IronOre: 1.0],
    outputs: [IronIngot: 1.0],
);

recipe!(
    RECIPE_PURE_IRON_INGOT,
    PureIronIngot,
    12.0,
    Refinery,
    inputs: [IronOre: 7.0, Water: 4.0],
    outputs: [IronIngot: 13.0],
);

recipe!(
    RECIPE_FUEL,
    Fuel,
    6.0,
    Refinery,
    inputs: [CrudeOil: 6.0],
    outputs: [Fuel: 4.0, PolymerResin: 3.0],
);

recipe!(
    RECIPE_RESIDUAL_FUEL,
    ResidualFuel,
    6.0,
    Refinery,
    inputs: [HeavyOilResidue: 6.0],
    outputs: [Fuel: 4.0],
);

recipe!(
    RECIPE_DILUTED_FUEL,
    DilutedFuel,
    6.0,
    Blender,
    inputs: [HeavyOilResidue: 5.0, Water: 10.0],
    outputs: [Fuel: 10.0],
);

recipe!(
    RECIPE_PLASTIC,
    Plastic,
    6.0,
    Refinery,
    inputs: [CrudeOil: 3.0],
    outputs: [Plastic: 2.0, HeavyOilResidue: 1.0],
);

recipe!(
    RECIPE_RUBBER,
    Rubber,
    6.0,
    Refinery,
    inputs: [CrudeOil: 3.0],
    outputs: [Rubber: 2.0, HeavyOilResidue: 2.0],
);

pub const ALL_RECIPES: &[Recipe] = &[
    RECIPE_IRON_INGOT,
    RECIPE_PURE_IRON_INGOT,
    RECIPE_FUEL,
    RECIPE_RESIDUAL_FUEL,
    RECIPE_DILUTED_FUEL,
    RECIPE_PLASTIC,
    RECIPE_RUBBER,
];

#[derive(Default)]
pub struct RecipeFilter {
    pub machine: Option<Machine>,
    pub input_item: Option<ItemName>,
    pub output_item: Option<ItemName>,
}

pub fn get_recipes(filter: RecipeFilter) -> Vec<Recipe> {
    ALL_RECIPES
        .iter()
        .filter(|r| {
            if let Some(m) = filter.machine {
                if r.machine != m {
                    return false;
                }
            }
            if let Some(item) = filter.input_item {
                if !r.inputs.iter().any(|i| i.item == item) {
                    return false;
                }
            }
            if let Some(item) = filter.output_item {
                if !r.outputs.iter().any(|i| i.item == item) {
                    return false;
                }
            }
            true
        })
        .copied()
        .collect()
}

use crate::items;
use crate::machines;
use strum::{AsRefStr, Display, EnumString};

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

#[derive(Debug, Clone, Copy)]
pub struct RecipeItem {
    pub item: items::Item,
    pub amount: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Recipe {
    pub name: RecipeName,
    pub inputs: &'static [RecipeItem],
    pub outputs: &'static [RecipeItem],
    pub duration: f32,
    pub machine: machines::Machine,
    pub alternate: bool,
}

impl Recipe {
    /// Items per minute at 100% clock speed
    pub fn base_rate(&self, item: RecipeItem) -> f32 {
        (60.0 * item.amount) / self.duration
    }

    /// Items per minute at the given clock speed (1 to 250)
    pub fn clocked_rate(&self, item: RecipeItem, clock: f32) -> f32 {
        self.base_rate(item) * (clock / 100.0)
    }
}

macro_rules! recipe {
    ($name:ident, $recipe_name:ident, $duration:expr, $machine:ident,
     inputs: [ $($in_item:expr, $in_amt:expr),* ],
     outputs: [ $($out_item:expr, $out_amt:expr),* ],
     alternate: $alternate:expr, $(,)?) => {
        pub const $name: Recipe = Recipe {
            name: RecipeName::$recipe_name,
            duration: $duration,
            machine: machines::Machine::$machine,
            inputs: &[
                $(RecipeItem { item: $in_item, amount: $in_amt }),*
            ],
            outputs: &[
                $(RecipeItem { item: $out_item, amount: $out_amt }),*
            ],
            alternate: $alternate,
        };
    };
}

// Recipes defined as constants using the macro
recipe!(
    RECIPE_IRON_INGOT,
    IronIngot,
    2.0,
    Smelter,
    inputs: [items::ITEM_IRON_ORE, 1.0],
    outputs: [items::ITEM_IRON_INGOT, 1.0],
    alternate: false,
);

recipe!(
    RECIPE_PURE_IRON_INGOT,
    PureIronIngot,
    12.0,
    Refinery,
    inputs: [items::ITEM_IRON_ORE, 7.0, items::ITEM_WATER, 4.0],
    outputs: [items::ITEM_IRON_INGOT, 13.0],
    alternate: true,
);

recipe!(
    RECIPE_FUEL,
    Fuel,
    6.0,
    Refinery,
    inputs: [items::ITEM_CRUDE_OIL, 6.0],
    outputs: [items::ITEM_FUEL, 4.0, items::ITEM_POLYMER_RESIN, 3.0],
    alternate: true,
);

recipe!(
    RECIPE_RESIDUAL_FUEL,
    ResidualFuel,
    6.0,
    Refinery,
    inputs: [items::ITEM_HEAVY_OIL_RESIDUE, 6.0],
    outputs: [items::ITEM_FUEL, 4.0],
    alternate: true,
);

recipe!(
    RECIPE_DILUTED_FUEL,
    DilutedFuel,
    6.0,
    Blender,
    inputs: [items::ITEM_HEAVY_OIL_RESIDUE, 5.0, items::ITEM_WATER, 10.0],
    outputs: [items::ITEM_FUEL, 10.0],
    alternate: true,
);

recipe!(
    RECIPE_PLASTIC,
    Plastic,
    6.0,
    Refinery,
    inputs: [items::ITEM_CRUDE_OIL, 3.0],
    outputs: [items::ITEM_PLASTIC, 2.0, items::ITEM_HEAVY_OIL_RESIDUE, 1.0],
    alternate: true,
);

recipe!(
    RECIPE_RUBBER,
    Rubber,
    6.0,
    Refinery,
    inputs: [items::ITEM_CRUDE_OIL, 3.0],
    outputs: [items::ITEM_RUBBER, 2.0, items::ITEM_HEAVY_OIL_RESIDUE, 2.0],
    alternate: true,
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
    pub machine: Option<machines::Machine>,
    pub input_item: Option<items::ItemName>,
    pub output_item: Option<items::ItemName>,
}

pub fn get_recipes(filter: RecipeFilter) -> Vec<Recipe> {
    ALL_RECIPES
        .iter()
        .filter(|r| {
            if let Some(m) = filter.machine
                && r.machine != m
            {
                return false;
            }
            if let Some(item) = filter.input_item
                && !r.inputs.iter().any(|i| i.item.name == item)
            {
                return false;
            }
            if let Some(item) = filter.output_item
                && !r.outputs.iter().any(|i| i.item.name == item)
            {
                return false;
            }
            true
        })
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_rate_formula_scales_with_amount() {
        let recipe = RECIPE_IRON_INGOT;
        let item = RecipeItem {
            item: items::ITEM_IRON_ORE,
            amount: 1.0,
        };
        let rate = recipe.base_rate(item);

        let item2 = RecipeItem {
            item: items::ITEM_IRON_ORE,
            amount: 2.0,
        };
        let rate2 = recipe.base_rate(item2);

        assert_eq!(rate2, rate * 2.0);
    }

    #[test]
    fn base_rate_formula_scales_inversely_with_duration() {
        let recipe = RECIPE_IRON_INGOT;
        let item = RecipeItem {
            item: items::ITEM_IRON_ORE,
            amount: 1.0,
        };
        let rate = recipe.base_rate(item);

        let recipe2 = RECIPE_PURE_IRON_INGOT;
        let item2 = RecipeItem {
            item: items::ITEM_IRON_ORE,
            amount: 1.0,
        };
        let rate2 = recipe2.base_rate(item2);

        assert_eq!(rate, rate2 * (12.0 / 2.0));
    }

    #[test]
    fn clocked_rate_scales_linearly_with_clock_speed() {
        let recipe = RECIPE_IRON_INGOT;
        let item = RecipeItem {
            item: items::ITEM_IRON_ORE,
            amount: 1.0,
        };
        let base = recipe.base_rate(item);

        assert_eq!(recipe.clocked_rate(item, 50.0), base * 0.5);
        assert_eq!(recipe.clocked_rate(item, 100.0), base * 1.0);
        assert_eq!(recipe.clocked_rate(item, 150.0), base * 1.5);
        assert_eq!(recipe.clocked_rate(item, 200.0), base * 2.0);
        assert_eq!(recipe.clocked_rate(item, 250.0), base * 2.5);
    }

    #[test]
    fn clocked_rate_at_100_percent_equals_base_rate() {
        let recipe = RECIPE_IRON_INGOT;
        let input = recipe.inputs[0];
        assert_eq!(recipe.clocked_rate(input, 100.0), recipe.base_rate(input));
    }

    #[test]
    fn get_recipes_returns_all_when_no_filter() {
        let recipes = get_recipes(RecipeFilter::default());
        assert_eq!(recipes.len(), ALL_RECIPES.len());
    }

    #[test]
    fn get_recipes_filters_by_machine() {
        let recipes = get_recipes(RecipeFilter {
            machine: Some(machines::Machine::Smelter),
            ..Default::default()
        });
        assert!(
            recipes
                .iter()
                .all(|r| r.machine == machines::Machine::Smelter)
        );
    }

    #[test]
    fn get_recipes_filters_by_output_item() {
        let recipes = get_recipes(RecipeFilter {
            output_item: Some(items::ItemName::IronIngot),
            ..Default::default()
        });
        assert!(recipes.iter().all(|r| {
            r.outputs
                .iter()
                .any(|o| o.item.name == items::ItemName::IronIngot)
        }));
    }

    #[test]
    fn get_recipes_filters_by_input_item() {
        let recipes = get_recipes(RecipeFilter {
            input_item: Some(items::ItemName::IronOre),
            ..Default::default()
        });
        assert!(recipes.iter().all(|r| {
            r.inputs
                .iter()
                .any(|i| i.item.name == items::ItemName::IronOre)
        }));
    }
}

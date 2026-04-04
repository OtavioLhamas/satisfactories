mod fuel_power;
mod iron_refinement;
mod items;
mod machines;
mod recipes;
mod recycling;
mod tui;

use clap::{Parser, Subcommand};
use inquire::{CustomType, Select};
use items::ItemName;
use recipes::{Recipe, get_recipes};

#[derive(Parser)]
#[command(name = "satisfactories")]
#[command(about = "A simple calculator for Satisfactory factory planning", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Run in CLI mode instead of TUI
    #[arg(long, default_value_t = false)]
    cli: bool,

    /// Minimum clock speed (e.g. 50 for 50%)
    #[arg(long, default_value_t = 50.0, global = true)]
    min_clock: f32,

    /// Maximum clock speed (e.g. 250 for 250%)
    #[arg(long, default_value_t = 250.0, global = true)]
    max_clock: f32,

    /// Maximum belt capacity (items/min)
    #[arg(short, long, default_value_t = 1200.0, global = true)]
    belt_limit: f32,

    /// Maximum pipe capacity (m³/min)
    #[arg(short, long, default_value_t = 600.0, global = true)]
    pipe_limit: f32,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Iron Refinement Center
    #[command(visible_alias = "irc")]
    IronRefinementCenter {
        /// Production goal (Iron Ingots/min)
        #[arg(short = 'g', long)]
        production_goal: Option<f32>,

        /// Machines per group (list of u32)
        #[arg(short = 'm', long, value_delimiter = ',', num_args = 1..)]
        machines_per_group: Option<Vec<u32>>,

        /// Maximum amount of groups
        #[arg(long)]
        max_groups: Option<u32>,

        /// Recipe name
        #[arg(short, long)]
        recipe: Option<String>,
    },

    /// Recycling Facility
    #[command(visible_alias = "rec")]
    RecyclingFacility {
        /// Target input rate in items per minute
        #[arg(short, long)]
        input_rate: Option<f32>,
    },

    /// Fuel Power Plant
    #[command(visible_alias = "fuel")]
    FuelPowerPlant {
        /// Target input rate in items per minute
        #[arg(short, long)]
        input_rate: Option<f32>,
    },
}

impl std::fmt::Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::IronRefinementCenter { .. } => write!(f, "Iron Refinement Center"),
            Commands::RecyclingFacility { .. } => write!(f, "Recycling Facility"),
            Commands::FuelPowerPlant { .. } => write!(f, "Fuel Power Plant"),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.cli {
        run_cli(cli);
    } else {
        tui::run_tui();
    }
}

fn run_cli(cli: Cli) {
    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            let options = vec![
                Commands::IronRefinementCenter {
                    production_goal: None,
                    machines_per_group: None,
                    max_groups: None,
                    recipe: None,
                },
                Commands::RecyclingFacility { input_rate: None },
                Commands::FuelPowerPlant { input_rate: None },
            ];

            let answer = Select::new("Select a facility to plan:", options).prompt();

            match answer {
                Ok(cmd) => cmd,
                _ => return,
            }
        }
    };

    match command {
        Commands::IronRefinementCenter {
            production_goal,
            machines_per_group,
            max_groups,
            recipe,
        } => {
            let goal = match production_goal {
                Some(g) => g,
                None => CustomType::<f32>::new("Production Goal (Iron Ingots/min):")
                    .with_default(10_000.0)
                    .prompt()
                    .unwrap_or(10_000.0),
            };

            let group = match machines_per_group {
                Some(m) => m,
                None => {
                    let input = CustomType::<String>::new("Machines per group (comma separated):")
                        .with_default("4, 6, 8, 9, 10, 12".to_string())
                        .prompt()
                        .unwrap_or("4, 6, 8, 9, 10, 12".to_string());
                    input
                        .split(',')
                        .filter_map(|s| s.trim().parse::<u32>().ok())
                        .collect()
                }
            };

            let max = match max_groups {
                Some(g) => g,
                None => CustomType::<u32>::new("Max amount of groups:")
                    .with_default(50)
                    .prompt()
                    .unwrap_or(50),
            };

            let available_recipes = get_recipes(recipes::RecipeFilter {
                output_item: Some(ItemName::IronIngot),
                ..Default::default()
            });

            let selected_recipe = match recipe {
                Some(name) => {
                    let name_lower = name.to_lowercase();
                    let matched = available_recipes.iter().find(|r| {
                        let r_name = r.name.to_string().to_lowercase();
                        if name_lower == "pure" {
                            r_name.contains("pure")
                        } else if name_lower == "default" {
                            r_name == "iron ingot"
                        } else {
                            r_name.contains(&name_lower)
                        }
                    });

                    match matched {
                        Some(r) => *r,
                        None => {
                            println!(
                                "Recipe matching '{}' not found. Please select from list.",
                                name
                            );
                            select_recipe(&available_recipes)
                        }
                    }
                }
                None => select_recipe(&available_recipes),
            };

            iron_refinement::run(iron_refinement::Config {
                production_goal: goal,
                machines_per_group: group,
                max_groups: max,
                min_clock: cli.min_clock,
                max_clock: cli.max_clock,
                belt_limit: cli.belt_limit,
                pipe_limit: cli.pipe_limit,
                recipe: selected_recipe,
            });
        }
        Commands::RecyclingFacility { input_rate } => {
            let rate = match input_rate {
                Some(r) => r,
                None => CustomType::<f32>::new("Target Input Rate (items/min):")
                    .with_default(60.0)
                    .with_help_message("Enter the amount of items to recycle")
                    .prompt()
                    .unwrap_or(60.0),
            };
            recycling::run(rate);
        }
        Commands::FuelPowerPlant { input_rate } => {
            let rate = match input_rate {
                Some(r) => r,
                None => CustomType::<f32>::new("Target Input Rate (items/min):")
                    .with_default(60.0)
                    .with_help_message("Enter the amount of fuel/crude oil")
                    .prompt()
                    .unwrap_or(60.0),
            };
            fuel_power::run(rate);
        }
    }
}

fn select_recipe(recipes: &[Recipe]) -> Recipe {
    let options: Vec<String> = recipes.iter().map(|r| r.name.to_string()).collect();
    let answer = Select::new("Select a recipe:", options).prompt().unwrap();
    *recipes
        .iter()
        .find(|r| r.name.to_string() == answer)
        .unwrap()
}

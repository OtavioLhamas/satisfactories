mod fuel_power;
mod iron_refinement;
mod recipes;
mod recycling;

use clap::{Parser, Subcommand};
use inquire::{CustomType, Select};

#[derive(Parser)]
#[command(name = "satisfactories")]
#[command(about = "A simple calculator for Satisfactory factory planning", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Iron Refinement Center
    IronRefinementCenter {
        /// Target input rate in items per minute
        #[arg(short, long)]
        input_rate: Option<f32>,
    },
    /// Recycling Facility
    RecyclingFacility {
        /// Target input rate in items per minute
        #[arg(short, long)]
        input_rate: Option<f32>,
    },
    /// Fuel Power Plant
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

    let command = match cli.command {
        Some(cmd) => cmd,
        None => {
            let options = vec![
                Commands::IronRefinementCenter { input_rate: None },
                Commands::RecyclingFacility { input_rate: None },
                Commands::FuelPowerPlant { input_rate: None },
            ];
            let ans = Select::new("Select a facility to plan:", options).prompt();

            match ans {
                Ok(cmd) => cmd,
                _ => return,
            }
        }
    };

    match command {
        Commands::IronRefinementCenter { input_rate } => {
            let rate = match input_rate {
                Some(r) => r,
                None => CustomType::<f32>::new("Target Input Rate (items/min):")
                    .with_default(60.0)
                    .with_help_message("Enter the amount of iron ore you want to process")
                    .prompt()
                    .unwrap_or(60.0),
            };
            iron_refinement::run(rate);
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

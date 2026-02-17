use crate::items::ItemType;
use crate::recipes::Recipe;
use tabled::{settings::Style, Table, Tabled};

pub struct Config {
    pub production_goal: f32,
    pub machines_per_group: Vec<u32>,
    pub max_groups: u32,
    pub min_clock: f32,
    pub max_clock: f32,
    pub belt_limit: f32,
    pub pipe_limit: f32,
    pub recipe: Recipe,
}

/// Raw calculation results
pub struct Setup {
    pub group_size: u32,
    pub group_count: u32,
    pub total_machines: u32,
    pub clock: f32,
    pub inputs: Vec<(f32, String)>,
    pub outputs: Vec<(f32, String)>,
    pub total_power: f32,
    pub total_shards: u32,
    pub stacks: Vec<u32>,
    pub space_efficiency: f32,
}

/// Tabled representation for display
#[derive(Tabled)]
struct Row {
    #[tabled(rename = "Group Count")]
    group_count: u32,
    #[tabled(rename = "Total Machines")]
    total_machines: String,
    #[tabled(rename = "Clock (%)")]
    clock: String,
    #[tabled(rename = "Input/Group")]
    input_per_group: String,
    #[tabled(rename = "Output/Group")]
    output_per_group: String,
    #[tabled(rename = "Power (MW)")]
    total_power: String,
    #[tabled(rename = "Shards")]
    total_shards: u32,
    #[tabled(rename = "Stacks")]
    stacks: String,
}

pub fn run(config: Config) {
    let setups = calculate(&config);
    display(&config, setups);
}

pub fn calculate(config: &Config) -> Vec<Setup> {
    let mut setups = Vec::new();

    let base_output_rate = config
        .recipe
        .outputs
        .iter()
        .find(|o| o.item.name.to_string().contains("Iron Ingot"))
        .map(|o| o.base_rate())
        .expect("Recipe must produce Iron Ingot");

    for &group_size in &config.machines_per_group {
        if group_size == 0 {
            continue;
        }

        let max_output_per_machine = base_output_rate * (config.max_clock / 100.0);
        let min_total_machines = (config.production_goal / max_output_per_machine).ceil() as u32;
        let min_groups = (min_total_machines as f32 / group_size as f32).ceil() as u32;

        for group_count in min_groups..=config.max_groups {
            let total_machines = group_size * group_count;
            if total_machines == 0 {
                continue;
            }

            let clock =
                (config.production_goal / (total_machines as f32 * base_output_rate)) * 100.0;

            if clock >= config.min_clock && clock <= config.max_clock {
                let mut inputs = Vec::new();
                let mut max_stack = u32::MAX;

                for input in config.recipe.inputs {
                    let rate = input.clocked_rate(clock) * group_size as f32;
                    inputs.push((rate, input.item.name.to_string()));

                    let limit = if input.item.item_type == ItemType::Fluid {
                        config.pipe_limit
                    } else {
                        config.belt_limit
                    };

                    let stack = (limit / rate).floor() as u32;
                    if stack < max_stack {
                        max_stack = stack;
                    }
                }

                let mut outputs = Vec::new();
                for output in config.recipe.outputs {
                    let rate = output.clocked_rate(clock) * group_size as f32;
                    outputs.push((rate, output.item.name.to_string()));

                    let limit = if output.item.item_type == ItemType::Fluid {
                        config.pipe_limit
                    } else {
                        config.belt_limit
                    };

                    let stack = (limit / rate).floor() as u32;
                    if stack < max_stack {
                        max_stack = stack;
                    }
                }

                let valid_stacks: Vec<u32> = (2..=max_stack).collect();

                let power_per_machine = config.recipe.machine.clocked_power(clock);
                let total_power = power_per_machine * total_machines as f32;
                let shards_per_machine = if clock > 100.0 {
                    ((clock - 100.0) / 50.0).ceil() as u32
                } else {
                    0
                };
                let total_shards = shards_per_machine * total_machines;

                let max_s = *valid_stacks.last().unwrap_or(&1);
                let space_efficiency = total_machines as f32 / max_s as f32;

                setups.push(Setup {
                    group_size,
                    group_count,
                    total_machines,
                    clock,
                    inputs,
                    outputs,
                    total_power,
                    total_shards,
                    stacks: valid_stacks,
                    space_efficiency,
                });
            }
        }
    }
    setups
}

pub fn display(config: &Config, setups: Vec<Setup>) {
    if setups.is_empty() {
        println!("No valid configurations found for the given parameters.");
        return;
    }

    // 1. Calculate Global Bests
    let min_power = setups
        .iter()
        .map(|s| s.total_power)
        .fold(f32::INFINITY, f32::min);
    let min_space_efficiency = setups
        .iter()
        .map(|s| s.space_efficiency)
        .fold(f32::INFINITY, f32::min);
    let max_stack_val = setups
        .iter()
        .map(|s| *s.stacks.last().unwrap_or(&1))
        .max()
        .unwrap_or(1);

    println!(
        "\nPlan for {} Iron Ingots/min using {}",
        config.production_goal, config.recipe.name
    );

    // Legend
    println!("Legend:");
    println!("  🟢 : Best Power Efficiency ({:.2} MW)", min_power);
    println!("  🔵 : Best Space Efficiency ({:.2} machines/stack)", min_space_efficiency);
    println!("  🟣 : Best Stackability (x{})", max_stack_val);

    for &size in &config.machines_per_group {
        let group_setups: Vec<&Setup> = setups.iter().filter(|s| s.group_size == size).collect();
        if group_setups.is_empty() {
            continue;
        }

        println!("\n## Group Size: {}", size);

        let mut rows = Vec::new();

        for setup in group_setups {
            // Power Efficiency Highlight
            let total_power_str = if (setup.total_power - min_power).abs() < 0.001 {
                format!("🟢 {:.1}", setup.total_power)
            } else {
                format!("{:.1}", setup.total_power)
            };

            // Space Efficiency Highlight (on Total Machines)
            let total_machines_str =
                if (setup.space_efficiency - min_space_efficiency).abs() < 0.001 {
                    format!("🔵 {}", setup.total_machines)
                } else {
                    setup.total_machines.to_string()
                };

            // Stackability Highlight
            let stacks_raw = if setup.stacks.is_empty() {
                "-".to_string()
            } else {
                format!("{:?}", setup.stacks)
            };

            let stacks_str = if *setup.stacks.last().unwrap_or(&1) == max_stack_val {
                format!("🟣 {}", stacks_raw)
            } else {
                stacks_raw
            };

            rows.push(Row {
                group_count: setup.group_count,
                total_machines: total_machines_str,
                clock: format!("{:.1}", setup.clock),
                input_per_group: setup
                    .inputs
                    .iter()
                    .map(|(r, name)| format!("{} {}", r, name))
                    .collect::<Vec<_>>()
                    .join(", "),
                output_per_group: setup
                    .outputs
                    .iter()
                    .map(|(r, name)| format!("{} {}", r, name))
                    .collect::<Vec<_>>()
                    .join(", "),
                total_power: total_power_str,
                total_shards: setup.total_shards,
                stacks: stacks_str,
            });
        }

        let mut table = Table::new(&rows);
        table.with(Style::modern());

        println!("{}", table);
    }
}

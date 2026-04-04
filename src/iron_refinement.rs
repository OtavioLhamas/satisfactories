use crate::items::{ItemName, ItemType};
use crate::recipes::Recipe;
use tabled::{Table, Tabled, settings::Style};

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

    let output_iron = config
        .recipe
        .outputs
        .iter()
        .find(|o| o.item.name == ItemName::IronIngot)
        .expect("Recipe must produce Iron Ingot");

    let base_output_rate = config.recipe.base_rate(*output_iron);

    let max_output_per_machine = base_output_rate * (config.max_clock / 100.0);
    let min_total_machines = (config.production_goal / max_output_per_machine).ceil() as u32;

    for &group_size in &config.machines_per_group {
        if group_size == 0 {
            continue;
        }

        let min_groups = (min_total_machines as f32 / group_size as f32).ceil() as u32;

        for group_count in min_groups..=config.max_groups {
            let total_machines = group_size * group_count;

            if total_machines == 0 {
                continue;
            }

            let clock =
                (config.production_goal / (total_machines as f32 * base_output_rate)) * 100.0;

            // skip if the machine count cannot support the clock speed limits
            if !(clock >= config.min_clock && clock <= config.max_clock) {
                continue;
            }

            let mut group_inputs = Vec::new();
            let mut max_stack = u32::MAX;

            for input in config.recipe.inputs {
                let group_rate = config.recipe.clocked_rate(*input, clock) * group_size as f32;
                group_inputs.push((group_rate, input.item.name.to_string()));

                let limit = if input.item.item_type == ItemType::Fluid {
                    config.pipe_limit
                } else {
                    config.belt_limit
                };

                let stack = (limit / group_rate).floor() as u32;
                if stack < max_stack {
                    max_stack = stack;
                }
            }

            let mut group_outputs = Vec::new();
            for output in config.recipe.outputs {
                let group_rate = config.recipe.clocked_rate(*output, clock) * group_size as f32;
                group_outputs.push((group_rate, output.item.name.to_string()));

                let limit = if output.item.item_type == ItemType::Fluid {
                    config.pipe_limit
                } else {
                    config.belt_limit
                };

                let stack = (limit / group_rate).floor() as u32;
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

            let last_stack = *valid_stacks.last().unwrap_or(&1);
            let space_efficiency = total_machines as f32 / last_stack as f32;

            setups.push(Setup {
                group_size,
                group_count,
                total_machines,
                clock,
                inputs: group_inputs,
                outputs: group_outputs,
                total_power,
                total_shards,
                stacks: valid_stacks,
                space_efficiency,
            });
        }
    }
    setups
}

pub fn display(config: &Config, setups: Vec<Setup>) {
    if setups.is_empty() {
        println!("No valid configurations found for the given parameters.");
        return;
    }

    // calculate Global Bests
    let min_power = setups
        .iter()
        .map(|s| s.total_power)
        .reduce(f32::min)
        .unwrap_or(0.0);
    let min_space_efficiency = setups
        .iter()
        .map(|s| s.space_efficiency)
        .reduce(f32::min)
        .unwrap_or(0.0);
    let max_stack_val = setups
        .iter()
        .map(|s| *s.stacks.last().unwrap_or(&1))
        .max()
        .unwrap_or(1);

    println!(
        "\n# Plan for {} Iron Ingots/min using {}",
        config.production_goal, config.recipe.name
    );

    let best_power = '🟢';
    let best_space = '🔵';
    let best_stack = '🟣';

    println!("Legend:");
    println!(
        "\t {}: Best Power Efficiency ({} MW)",
        best_power, min_power
    );
    println!(
        "\t {}: Best Space Efficiency ({} machines/stack)",
        best_space, min_space_efficiency
    );
    println!("\t {}: Best Stackability (x{})", best_stack, max_stack_val);

    for &size in &config.machines_per_group {
        let group_setups: Vec<&Setup> = setups.iter().filter(|s| s.group_size == size).collect();
        if group_setups.is_empty() {
            continue;
        }

        println!("\n## Group Size: {}", size);

        let mut rows = Vec::new();

        for setup in group_setups {
            let mut total_power_str = format!("{}", setup.total_power);
            // Highlight Power Efficiency within 1% of the lowest
            if (setup.total_power - min_power).abs() < (setup.total_power * 0.01) {
                total_power_str.insert_str(0, &format!("{} ", best_power));
            }

            let mut total_machines_str = setup.total_machines.to_string();
            // Highlight Space Efficiency within 5% of lowest
            if (setup.space_efficiency - min_space_efficiency).abs()
                < (setup.space_efficiency * 0.05)
            {
                total_machines_str.insert_str(0, &format!("{} ", best_space));
            }

            let mut stacks_str = if setup.stacks.is_empty() {
                "-".to_string()
            } else {
                format!("{:?}", setup.stacks)
            };

            // Highlight Stackability
            if *setup.stacks.last().unwrap_or(&1) == max_stack_val {
                stacks_str.insert_str(0, &format!("{} ", best_stack));
            }

            rows.push(Row {
                group_count: setup.group_count,
                total_machines: total_machines_str,
                clock: format!("{}", setup.clock),
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

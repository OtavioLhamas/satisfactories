# Satisfactories CLI

A simple CLI tool designed to help me plan factories in the game Satisfactory. It calculates machine configurations, clock speeds, and logistical limits to optimize production lines. It is not a general purpose planning tool, and is tailored specifically for the setups I have in mind for each factory.

## Installation

Ensure you have Rust installed, then clone the repository and build:

```bash
cargo build --release
```

## Usage

You can run the tool interactively or by passing arguments directly.

### Global Options

These options apply to all commands:

-   `--min-clock <FLOAT>`: Minimum clock speed percentage (default: 50.0).
-   `--max-clock <FLOAT>`: Maximum clock speed percentage (default: 250.0).
-   `--belt-limit <FLOAT>`: Maximum items per minute for belts (default: 1200.0).
-   `--pipe-limit <FLOAT>`: Maximum cubic meters per minute for pipes (default: 600.0).

### Iron Refinement Center

Plan your iron ingot production.

```bash
satisfactories iron-refinement-center --production-goal <FLOAT> [OPTIONS]
```

**Arguments:**

-   `-g, --production-goal <FLOAT>`: Target Iron Ingots per minute.
-   `-m, --machines-per-group <UINT>...`: List of group sizes to test (e.g., `-m 4,8,12`).
-   `--max-groups <UINT>`: Maximum number of groups to calculate.
-   `-r, --recipe <STRING>`: Specific recipe to use (e.g., "Pure Iron Ingot").

**Example:**

Plan for 4000 Iron Ingots/min using Pure Iron Ingots, checking groups of 8, 10, and 12 machines:

```bash
satisfactories iron-refinement-center -p 4000 -m 8,10,12 -r "Pure Iron Ingot"
```

#### Output Explained

The tool outputs a series of tables, one for each requested **Group Size**:

-   **Group Count**: Total number of these groups needed.
-   **Total Machines**: Total count of machines across all groups.
-   **Clock (%)**: Required clock speed.
-   **Input/Output**: Total flow rates per group.
-   **Stacks**: Valid stack configurations (how many times the group can be manifolded).

**Highlights:**
Rows are color-coded to indicate the most efficient configurations:
-   **Green**: Best Power Efficiency.
-   **Blue**: Best Space Efficiency.
-   **Magenta**: Best Stackability.

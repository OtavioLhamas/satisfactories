use std::fmt;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clocked_power_at_100_percent_equals_base() {
        let machine = Machine::Smelter;
        assert_eq!(machine.clocked_power(100.0), machine.base_power());
    }

    #[test]
    fn clocked_power_increases_with_clock_speed() {
        let machine = Machine::Refinery;
        let power_100 = machine.clocked_power(100.0);
        let power_150 = machine.clocked_power(150.0);
        assert!(power_150 > power_100);
    }
}

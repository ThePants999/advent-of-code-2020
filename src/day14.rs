use std::collections::HashMap;

pub fn day14(input_lines: &[String]) -> (u64, u64) {
    let mut mem1 = Part1Memory::new();
    let mut mem2 = Part2Memory::new();
    for line in input_lines {
        if line.starts_with("mask") {
            let mask = &line[7..];
            mem1.set_mask(mask);
            mem2.set_mask(mask);
        } else {
            let addr = line[4..line.find(']').expect("Invalid input")].parse::<u64>().expect("Invalid input");
            let value = line[line.find('=').expect("Invalid input") + 2..].parse::<u64>().expect("Invalid input");
            mem1.set_memory(addr, value);
            mem2.set_memory(addr, value);
        }
    }
    let part1 = mem1.values_sum();
    let part2 = mem2.values_sum();
    (part1,part2)
}

struct Part2Memory {
    floating_bits: Vec<u64>,
    ones_mask: u64,
    memory: HashMap<u64, u64>,
}

impl Part2Memory {
    fn new() -> Self {
        Self {
            floating_bits: Vec::new(),
            ones_mask: 0,
            memory: HashMap::new(),
        }
    }    

    fn set_mask(&mut self, mask: &str) {
        self.floating_bits.clear();
        self.ones_mask = u64::from_str_radix(&mask.replace('X', "0"), 2).expect("Invalid mask");
        for (bit, c) in mask.chars().rev().enumerate() {
            if c == 'X' {
                self.floating_bits.push(1u64 << bit);
            }
        }
    }

    fn set_memory(&mut self, address: u64, value: u64) {
        self.recursive_set_memory(address | self.ones_mask, value, 0);
    }

    fn recursive_set_memory(&mut self, address: u64, value: u64, bit_index: usize) {
        // Called twice for every entry in floating_bits, once with each bit value.
        if bit_index < self.floating_bits.len() {
            // There's another bit to toggle.
            self.recursive_set_memory(address | self.floating_bits[bit_index], value, bit_index + 1);
            self.recursive_set_memory(address & !self.floating_bits[bit_index], value, bit_index + 1);
        } else {
            // We're at the bottom of the recursion!
            self.memory.insert(address, value);
        }
    }

    fn values_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

struct Part1Memory {
    zeroes_mask: u64,
    ones_mask: u64,
    memory: HashMap<u64, u64>,
}

impl Part1Memory {
    fn new() -> Self {
        Self {
            zeroes_mask: u64::MAX,
            ones_mask: 0,
            memory: HashMap::new(),
        }
    }

    fn set_mask(&mut self, mask: &str) {
        self.zeroes_mask = u64::from_str_radix(&mask.replace('X', "1"), 2).expect("Invalid mask");
        self.ones_mask = u64::from_str_radix(&mask.replace('X', "0"), 2).expect("Invalid mask");
    }

    fn set_memory(&mut self, address: u64, value: u64) {
        let masked_value = (value | self.ones_mask) & self.zeroes_mask;
        self.memory.insert(address, masked_value);
    }

    fn values_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}
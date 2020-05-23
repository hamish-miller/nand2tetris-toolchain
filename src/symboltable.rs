/// SymbolTable: Keep a correspondence between symbolic labels and numeric addresses.

use std::collections::HashMap;

static PREDEFINED_SYMBOLS: &'static [(&'static str, u32)] = &[
    ("SP",   0x0000),
    ("LCL",  0x0001),
    ("ARG",  0x0002),
    ("THIS", 0x0003),
    ("THAT", 0x0004),
    ("R0",   0x0000),
    ("R1",   0x0001),
    ("R2",   0x0002),
    ("R3",   0x0003),
    ("R4",   0x0004),
    ("R5",   0x0005),
    ("R6",   0x0006),
    ("R7",   0x0007),
    ("R8",   0x0008),
    ("R9",   0x0009),
    ("R10",  0x000a),
    ("R11",  0x000b),
    ("R12",  0x000c),
    ("R13",  0x000d),
    ("R14",  0x000e),
    ("R15",  0x000f),
    ("SCREEN", 0x4000),
    ("KBD", 0x6000),
];

pub struct SymbolTable {
    pub hash: HashMap<String, u32>,
}

impl SymbolTable {
    pub const NEXT_AVAILABLE_RAM_ADDRESS: u32 = 16;

    pub fn new() -> Self {
        let mut symboltable = SymbolTable { hash: HashMap::new() };

        for (k, v) in PREDEFINED_SYMBOLS.iter() {
            symboltable.addEntry(k, *v);
        }

        symboltable
    }

    #[allow(non_snake_case)]
    pub fn addEntry(&mut self, symbol: &str, address: u32) {
        self.hash.insert(symbol.to_string(), address);
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.hash.contains_key(symbol)
    }

    #[allow(non_snake_case)]
    pub fn GetAddress(&self, symbol: &str) -> u32 {
        *self.hash.get(symbol)
            .expect(&format!("KeyError: {}", symbol))
    }
}


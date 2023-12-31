use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
        Symbol {
            name,
            symbol_type,
            offset,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SymbolType {
    Label,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn symbol_value(&self, s: &str) -> Option<u32> {
        self.symbols
            .iter()
            .find(|symbol| symbol.name == s)
            .map(|symbol| symbol.offset)
    }

    pub fn has_symbol(&self, s: &Symbol) -> bool {
        self.symbols.contains(s)
    }

    pub fn set_symbol_offset(&mut self, name: &str, offset: u32) {
        self.symbols
            .iter_mut()
            .find(|item| item.name == name)
            .map(|item| {
                item.offset = offset;
                return item;
            });
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut sym = SymbolTable::new();
        let new_symbol = Symbol::new("test".to_string(), SymbolType::Label, 12);
        sym.add_symbol(new_symbol);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.symbol_value("test");
        assert_eq!(true, v.is_some());
        let v = v.unwrap();
        assert_eq!(v, 12);
        let v = sym.symbol_value("does_not_exist");
        assert_eq!(v.is_some(), false);
    }
}

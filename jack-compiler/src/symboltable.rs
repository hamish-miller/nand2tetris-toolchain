/// SymbolTable: Associate identifiers with properties required for compilation

use std::collections::HashMap;
use std::fmt;

type Name = String;
type Type = String;
type Index = usize;

type Entry = (Type, Kind, Index);
type Table = HashMap<Name, Entry>;

pub struct SymbolTable {
    class: Table,
    subroutine: Table,
}

impl SymbolTable {
    #![allow(non_snake_case)]  // Contract pre-specified

    pub fn new() -> Self {
        SymbolTable {
            class: HashMap::new(),
            subroutine: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, _type: &str, kind: Kind) {
        let k = Name::from(name);
        let v = (Type::from(_type), kind, self.varCount(kind));

        self.insert(k, v);
    }

    pub fn startSubroutine(&mut self) {
        self.subroutine.clear();
    }

    pub fn varCount(&self, kind: Kind) -> usize {
        self.iter()
            .filter(|(_k, v)| v.1 == kind)
            .count()
    }

    pub fn kindOf(&self, name: &str) -> Option<Kind> {
        self.get(name).map(|v| v.1)
    }

    pub fn typeOf(&self, name: &str) -> Type {
        self.get(name).map(|v| v.0.clone()).unwrap()
    }

    pub fn indexOf(&self, name: &str) -> Index {
        self.get(name).map(|v| v.2).unwrap()
    }

    // Reduced HashMap API
    #[inline]
    fn insert(&mut self, k: Name, v: Entry) {
        match Scope::from(v.1) {
            Scope::Class => self.class.insert(k, v),
            Scope::Subroutine => self.subroutine.insert(k, v),
        };
    }

    #[inline]
    fn get(&self, k: &str) -> Option<&Entry> {
        self.subroutine.get(k).or_else(|| self.class.get(k))
    }

    #[cfg(test)]
    #[inline]
    fn is_empty(&self) -> bool {
        self.class.is_empty() && self.subroutine.is_empty()
    }

    // Psuedo-Iterator
    fn iter(&self) -> impl Iterator<Item = (&Name, &Entry)> {
        self.class.iter().chain(self.subroutine.iter())
    }
}



#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Kind {
    Static,
    Field,
    Arg,
    Var,
}

enum Scope {
    Class,
    Subroutine,
}

impl From<Kind> for Scope {
    fn from(kind: Kind) -> Self {
        use Kind::*;
        use Scope::*;
        match kind {
            Static | Field => Class,
            Arg    | Var   => Subroutine,
        }
    }
}


// For debugging
impl fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut results = vec!(writeln!(f, "| name | type | kind | # |"));

        for (k, v) in self.iter() {
            results.push(writeln!(f, "| {} | {} | {} | {} |", k, v.0, v.1, v.2))
        }

        if let Some(&err) = results.iter().find(|r| r.is_err()) {
            return err
        }

        Ok(())
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self {
            Kind::Static => "Static",
            Kind::Field  => "Field",
            Kind::Arg    => "Arg",
            Kind::Var    => "Var",
        };
        write!(f, "{}", kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Kind::*;

    #[test]
    fn test_new_symboltable_is_empty() {
        let s = SymbolTable::new();

        assert!(s.is_empty())
    }

    #[test]
    fn test_define_initial_index_zero() {
        let mut s = SymbolTable::new();

        s.define("foo", "bool", Static);

        assert!(matches!(s.get("foo"), Some((_, _, 0))));
    }

    #[test]
    fn test_define_two_of_a_kind_index_incremented() {
        let mut s = SymbolTable::new();

        s.define("foo", "bool", Static);
        s.define("bar", "bool", Static);

        assert!(matches!(s.get("bar"), Some((_, _, 1))));
    }

    #[test]
    fn test_define_two_kinds_index_zeroed() {
        let mut s = SymbolTable::new();

        s.define("foo", "bool", Static);
        s.define("bar", "bool", Field);

        assert!(matches!(s.get("bar"), Some((_, _, 0))));
        assert!(matches!(s.get("bar"), Some((_, _, 0))));
    }

    #[test]
    fn test_start_subroutine_resets_subroutine_table() {
        let mut s = SymbolTable::new();

        s.define("foo", "bool", Static);
        s.define("bar", "bool", Arg);

        assert!(s.get("foo").is_some());
        assert!(s.get("bar").is_some());

        s.startSubroutine();

        assert!(s.get("foo").is_some());
        assert!(s.get("bar").is_none());
    }

    #[test]
    fn test_var_count() {
        let mut s = SymbolTable::new();

        s.define("foo", "bool", Static);
        s.define("bar", "bool", Static);

        assert_eq!(s.varCount(Static), 2);
    }

    mod query_methods {
        use super::*;

        fn symbol_table() -> SymbolTable {
            let mut s = SymbolTable::new();
            s.define("foo", "bool", Static);
            s
        }

        #[test]
        fn test_kind_of() {
            assert_eq!(symbol_table().kindOf("foo"), Some(Static));
        }

        #[test]
        fn test_type_of() {
            assert_eq!(symbol_table().typeOf("foo"), "bool");
        }

        #[test]
        fn test_index_of() {
            assert_eq!(symbol_table().indexOf("foo"), 0);
        }

        mod unknown_identifier_behaviour {
            use super::*;

            fn empty_symbol_table() -> SymbolTable {
                SymbolTable::new()
            }

            #[test]
            fn test_kind_of_returns_none() {
                assert_eq!(empty_symbol_table().kindOf("foo"), None);
            }

            #[test]
            #[should_panic]
            fn test_type_of_panics() {
                empty_symbol_table().typeOf("foo");
            }

            #[test]
            #[should_panic]
            fn test_index_of_panics() {
                empty_symbol_table().indexOf("foo");
            }
        }
    }

    #[test]
    fn test_subroutine_scope_shadows_class_scope() {
        let mut s = SymbolTable::new();

        s.define("foo", "bool", Static);
        s.define("foo", "bool", Arg);

        assert_eq!(s.kindOf("foo"), Some(Arg));
    }
}

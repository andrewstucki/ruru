use std::convert::From;

use binding::symbol;
use binding::util;
use types::{Value, ValueType};

use {Object, VerifiedObject};

/// `Symbol`
#[derive(Debug, PartialEq)]
pub struct Symbol {
    value: Value,
}

impl Symbol {
    /// Creates a new instance of Ruby `Symbol`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Symbol, VM};
    /// # VM::init();
    ///
    /// let symbol = Symbol::new("hello");
    ///
    /// assert_eq!(symbol.to_string(), "hello");
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// sym = :hello
    ///
    /// sym.to_s == 'hello'
    /// ```
    pub fn new(string: &str) -> Self {
        let id = util::internal_id(string);

        Self::from(symbol::id_to_sym(id))
    }

    /// Retrieves the Rust `String` corresponding to `Symbol` object (Ruby `Symbol#to_s`).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ruru::{Symbol, VM};
    /// # VM::init();
    ///
    /// let symbol = Symbol::new("hello");
    ///
    /// assert_eq!(symbol.to_string(), "hello");
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// sym = :hello
    ///
    /// sym.to_s == 'hello'
    /// ```
    pub fn to_string(&self) -> String {
        let id = symbol::sym_to_id(self.value());

        symbol::id_to_name(id)
    }
}

impl From<Value> for Symbol {
    fn from(value: Value) -> Self {
        Symbol { value: value }
    }
}

impl Object for Symbol {
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Symbol {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.value().ty() == ValueType::Symbol
    }

    fn error_message() -> String {
        "Error converting to Symbol".to_string()
    }
}

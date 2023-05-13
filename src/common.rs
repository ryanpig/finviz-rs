use std::collections::BTreeMap;

/// Convert enum types to more readable string 
pub trait DisplayString {

    /// Implemnet this function to output readable string or description of a variant of enum type
    fn to_display_string(&self) -> &str;
}

/// Alias to represent two-dimension String data
pub type TableData = Vec<Vec<String>>;

/// Alias to represent key-value String data
pub type DictData = BTreeMap<String, String>;

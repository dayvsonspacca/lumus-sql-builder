mod select;
pub use select::*;

mod columns;
pub use columns::*;

mod create_table;
pub use create_table::*;

mod insert;
pub use insert::*;

mod condition;
pub use condition::*;

mod update;
pub use update::*;

mod delete;
pub use delete::*;

mod join;
pub use join::*;

fn escape_value(value: &str) -> String {
    let mut escaped_value = String::with_capacity(value.len() + 2);
    escaped_value.push('\'');
    escaped_value.push_str(value);
    escaped_value.push('\'');
    escaped_value
}

trait BuildableStatement {
    fn build(&self) -> String;
}

impl core::fmt::Display for dyn BuildableStatement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.build())
    }
}

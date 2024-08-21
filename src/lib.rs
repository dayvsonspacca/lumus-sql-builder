pub mod errors;
pub mod sqlite;

fn escape_value(value: &str) -> String {
    let mut escaped_value = String::with_capacity(value.len() + 2);
    escaped_value.push('\'');
    escaped_value.push_str(value);
    escaped_value.push('\'');
    escaped_value
}

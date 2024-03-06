use lumus_sql_builder::sqlite::create::{Column, CreateTable};

#[test]
fn test_create_table() {
    let create = CreateTable::new(
        "example_table",
        vec![
            Column::new("id").literal("INT"),
            Column::new("name").literal("VARCHAR(255)"),
            Column::new("age").literal("INT"),
        ],
    )
    .build();

    let expected_sql = "CREATE TABLE example_table (id INT, name VARCHAR(255), age INT);";

    assert_eq!(create, expected_sql);
}

#[test]
fn test_create_table_if_not_exists() {
    let create = CreateTable::new(
        "example_table",
        vec![
            Column::new("id").literal("INT"),
            Column::new("name").literal("VARCHAR(255)"),
            Column::new("age").literal("INT"),
        ],
    )
    .if_not_exists()
    .build();

    let expected_sql =
        "CREATE TABLE IF NOT EXISTS example_table (id INT, name VARCHAR(255), age INT);";

    assert_eq!(create, expected_sql);
}

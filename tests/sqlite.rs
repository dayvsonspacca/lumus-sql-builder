use lumus_sql_builder::sqlite::create::{Column, CreateTable};

#[test]
fn test_column_integer() {
    let column = Column::new("age").integer().build();
    assert_eq!(column, "age INTEGER");
}

#[test]
fn test_column_text() {
    let column = Column::new("name").text().build();
    assert_eq!(column, "name TEXT");
}

#[test]
fn test_column_real() {
    let column = Column::new("price").real().build();
    assert_eq!(column, "price REAL");
}

#[test]
fn test_column_boolean() {
    let column = Column::new("is_active").boolean().build();
    assert_eq!(column, "is_active BOOLEAN");
}

#[test]
fn test_column_primary_key() {
    let column = Column::new("id").integer().primary_key().build();
    assert_eq!(column, "id INTEGER PRIMARY KEY");
}

#[test]
fn test_column_not_null() {
    let column = Column::new("name").text().not_null().build();
    assert_eq!(column, "name TEXT NOT NULL");
}

#[test]
fn test_column_unique() {
    let column = Column::new("email").text().unique().build();
    assert_eq!(column, "email TEXT UNIQUE");
}

#[test]
fn test_column_default() {
    let column = Column::new("age").integer().default("0").build();
    assert_eq!(column, "age INTEGER DEFAULT 0");
}

#[test]
fn test_column_auto_increment() {
    let column = Column::new("id")
        .integer()
        .primary_key()
        .auto_increment()
        .build();
    assert_eq!(column, "id INTEGER PRIMARY KEY AUTOINCREMENT");
}

#[test]
fn test_create_table() {
    let create = CreateTable::new(
        "example_table",
        vec![
            Column::new("id").integer().primary_key().auto_increment(),
            Column::new("name").text().not_null(),
            Column::new("age").integer().default("0"),
            Column::new("price").real().default("0.0"),
            Column::new("is_active").boolean().default("true"),
        ],
    )
    .build();

    let expected_sql = "CREATE TABLE example_table (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, age INTEGER DEFAULT 0, price REAL DEFAULT 0.0, is_active BOOLEAN DEFAULT true);";

    assert_eq!(create, expected_sql);
}

#[test]
fn test_create_table_if_not_exists() {
    let create = CreateTable::new(
        "example_table",
        vec![
            Column::new("id").integer().primary_key().auto_increment(),
            Column::new("name").text().not_null(),
            Column::new("age").integer().default("0"),
            Column::new("price").real().default("0.0"),
            Column::new("is_active").boolean().default("true"),
        ],
    )
    .if_not_exists()
    .build();

    let expected_sql = "CREATE TABLE IF NOT EXISTS example_table (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, age INTEGER DEFAULT 0, price REAL DEFAULT 0.0, is_active BOOLEAN DEFAULT true);";

    assert_eq!(create, expected_sql);
}

use lumus_sql_builder::sqlite::{Column, CreateTable, Insert, Select};
use sqlite::Connection;
use std::fs;

const TEST_DB: &str = "test.sqlite";

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

#[test]
fn test_complex_create_table() {
    let create = CreateTable::new(
        "complex_table",
        vec![
            Column::new("id").integer().primary_key().auto_increment(),
            Column::new("name").text().not_null().unique(),
            Column::new("age").integer().default("0").not_null(),
            Column::new("price").real().default("0.0"),
            Column::new("is_active").boolean().default("true"),
            Column::new("blob_data").blob(),
            Column::new("timestamp_data").datetime(),
            Column::new("check_column").integer(),
        ],
    )
    .if_not_exists()
    .build();

    let expected_sql =
        "CREATE TABLE IF NOT EXISTS complex_table (id INTEGER PRIMARY KEY AUTOINCREMENT, \
                        name TEXT NOT NULL UNIQUE, age INTEGER DEFAULT 0 NOT NULL, \
                        price REAL DEFAULT 0.0, is_active BOOLEAN DEFAULT true, \
                        blob_data BLOB, timestamp_data DATETIME, \
                        check_column INTEGER);";

    assert_eq!(create, expected_sql);
}

#[test]
fn test_select_without_options() {
    let query = Select::new("users").build();
    assert_eq!(query, "SELECT * FROM users;");
}

#[test]
fn test_select_with_distinct() {
    let query = Select::new("orders").distinct().build();
    assert_eq!(query, "SELECT DISTINCT * FROM orders;");
}

#[test]
fn test_select_with_columns_and_order() {
    let query = Select::new("products")
        .columns("name, price")
        .order("price DESC")
        .build();
    assert_eq!(
        query,
        "SELECT name, price FROM products ORDER BY price DESC;"
    );
}

#[test]
fn test_select_with_group_and_limit() {
    let query = Select::new("transactions")
        .group("category")
        .limit(10)
        .build();
    assert_eq!(
        query,
        "SELECT * FROM transactions GROUP BY category LIMIT 10;"
    );
}

#[test]
fn test_select_with_offset() {
    let query = Select::new("logs").offset(20).build();
    assert_eq!(query, "SELECT * FROM logs OFFSET 20;");
}

#[test]
fn test_column_literal() {
    let column_with_literal = Column::new("age").literal("INTEGER NOT NULL").build();

    assert_eq!(column_with_literal, "age INTEGER NOT NULL");
}

/// # `Insert` tests zone

#[test]
fn test_new_insert_instance() {
    let insert_query = Insert::new("test_table");
    assert_eq!(insert_query.table, "test_table");
    assert_eq!(insert_query.values.len(), 0);
}

#[test]
fn test_set_values_for_insert() {
    let insert_query = Insert::new("test_table").values(vec![("name", "John"), ("age", "30")]);
    assert_eq!(insert_query.values.len(), 2);
    assert_eq!(
        insert_query.values[0],
        ("name".to_string(), "John".to_string())
    );
    assert_eq!(
        insert_query.values[1],
        ("age".to_string(), "30".to_string())
    );
}

#[test]
fn test_build_insert_query() {
    let insert_query = Insert::new("test_table").values(vec![("name", "John"), ("age", "30")]);
    assert_eq!(
        insert_query.build(),
        "INSERT INTO test_table (name, age) VALUES ('John', '30');"
    );
}

#[test]
fn test_insert_query_with_empty_values() {
    let insert_query = Insert::new("test_table").values(vec![("name", ""), ("age", "30")]);
    assert_eq!(
        insert_query.build(),
        "INSERT INTO test_table (name, age) VALUES ('', '30');"
    );
}

#[test]
fn test_insert_query_with_medium_values() {
    let insert_query = Insert::new("test_table").values(vec![
        ("name", "John"),
        ("age", "30"),
        ("city", "New York"),
        ("occupation", "Engineer"),
    ]);
    assert_eq!(
        insert_query.build(),
        "INSERT INTO test_table (name, age, city, occupation) VALUES ('John', '30', 'New York', 'Engineer');"
    );
}

#[test]
fn test_insert_query_with_large_values() {
    let insert_query = Insert::new("test_table").values(vec![
        ("name", "John"),
        ("age", "30"),
        ("city", "New York"),
        ("occupation", "Engineer"),
        ("salary", "100000"),
        ("email", "john@example.com"),
        ("phone", "123-456-7890"),
    ]);
    assert_eq!(
        insert_query.build(),
        "INSERT INTO test_table (name, age, city, occupation, salary, email, phone) VALUES ('John', '30', 'New York', 'Engineer', '100000', 'john@example.com', '123-456-7890');"
    );
}

#[test]
fn test_insert_query_with_many_values() {
    let insert_query = Insert::new("test_table").values(vec![
        ("name", "John"),
        ("age", "30"),
        ("city", "New York"),
        ("occupation", "Engineer"),
        ("salary", "100000"),
        ("email", "john@example.com"),
        ("phone", "123-456-7890"),
        ("address", "123 Main St"),
        ("department", "Engineering"),
        ("company", "TechCorp"),
    ]);
    assert_eq!(
        insert_query.build(),
        "INSERT INTO test_table (name, age, city, occupation, salary, email, phone, address, department, company) VALUES ('John', '30', 'New York', 'Engineer', '100000', 'john@example.com', '123-456-7890', '123 Main St', 'Engineering', 'TechCorp');"
    );
}

#[test]
fn test_insert_query_with_repeated_values() {
    let insert_query = Insert::new("test_table").values(vec![
        ("name", "John"),
        ("age", "30"),
        ("name", "Jane"),
        ("age", "25"),
    ]);
    assert_eq!(
        insert_query.build(),
        "INSERT INTO test_table (name, age, name, age) VALUES ('John', '30', 'Jane', '25');"
    );
}

#[test]
fn test_insert_query_with_null_values() {
    let insert_query = Insert::new("test_table").values(vec![
        ("name", "John"),
        ("age", ""),
        ("city", "New York"),
        ("occupation", ""),
        ("salary", ""),
    ]);
    assert_eq!(
        insert_query.build(),
        "INSERT INTO test_table (name, age, city, occupation, salary) VALUES ('John', '', 'New York', '', '');"
    );
}

#[test]
fn test_insert_query_with_special_characters() {
    let insert_query = Insert::new("test_table").values(vec![
        ("name", "John O'Connor"),
        ("age", "30"),
        ("city", "New York"),
        ("occupation", "Software Engineer"),
    ]);
    assert_eq!(
        insert_query.build(),
        "INSERT INTO test_table (name, age, city, occupation) VALUES ('John O'Connor', '30', 'New York', 'Software Engineer');"
    );
}

#[test]
fn teste_create_tables_in_db() {
    let connection = sqlite::open(TEST_DB).expect("Failed to open database connection");

    test_create_products_table(&connection);
    test_create_users_table(&connection);
    test_insert_sample_data_products(&connection);
    test_select_sample_data_products(&connection);

    match fs::remove_file(TEST_DB) {
        Ok(()) => println!("Wiping database..."),
        Err(e) => println!("Failde to wipe DB. {}", e),
    }
}

fn test_create_products_table(connection: &Connection) {
    let create_table = CreateTable::new(
        "products",
        vec![
            Column::new("id")
                .integer()
                .not_null()
                .primary_key()
                .auto_increment(),
            Column::new("name").text().not_null().unique(),
            Column::new("description").text(),
            Column::new("price").real().not_null(),
            Column::new("stock").integer().not_null().default("0"),
            Column::new("created_at")
                .datetime()
                .default("CURRENT_TIMESTAMP"),
            Column::new("updated_at")
                .datetime()
                .default("CURRENT_TIMESTAMP"),
        ],
    )
    .if_not_exists();
    let expected_sql = "CREATE TABLE IF NOT EXISTS products (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL UNIQUE, description TEXT, price REAL NOT NULL, stock INTEGER NOT NULL DEFAULT 0, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP);";

    assert_eq!(create_table.build(), expected_sql);
    connection.execute(create_table.build()).unwrap();
}

fn test_create_users_table(connection: &Connection) {
    let create_table = CreateTable::new(
        "users",
        vec![
            Column::new("id")
                .integer()
                .not_null()
                .primary_key()
                .auto_increment(),
            Column::new("username").text().not_null().unique(),
            Column::new("email").text().not_null().unique(),
            Column::new("password").text().not_null(),
            Column::new("created_at")
                .datetime()
                .default("CURRENT_TIMESTAMP"),
            Column::new("updated_at")
                .datetime()
                .default("CURRENT_TIMESTAMP"),
        ],
    )
    .if_not_exists();

    let expected_sql = "CREATE TABLE IF NOT EXISTS users (id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL UNIQUE, email TEXT NOT NULL UNIQUE, password TEXT NOT NULL, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, updated_at DATETIME DEFAULT CURRENT_TIMESTAMP);";

    assert_eq!(create_table.build(), expected_sql);
    connection.execute(create_table.build()).unwrap();
}

fn test_insert_sample_data_products(connection: &Connection) {
    let insert_query =
        Insert::new("products").values(vec![("name", "Product A"), ("price", "100.0")]);
    connection
        .execute(insert_query.build())
        .expect("Failed to insert data into database");
}

fn test_select_sample_data_products(connection: &Connection) {
    let query = Select::new("products").columns("name, price").build();
    connection
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                println!("{} = {}", name, value.unwrap());
            }
            true
        })
        .expect("Failed to iterate");
}

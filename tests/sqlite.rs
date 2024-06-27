use lumus_sql_builder::sqlite::{Column, CreateTable, Insert, Select, Where};
use sqlite::{Connection, State};

#[test]
fn create_test() {
    let create = create_test_table_schema();
    assert_eq!(
        "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, email TEXT UNIQUE NOT NULL, age INTEGER NOT NULL, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, update_at DATETIME );", 
        create
    );

    let conn = Connection::open(":memory:").unwrap();
    conn.execute(create).unwrap()
}

#[test]
fn insert_and_select_test() {
    let create = create_test_table_schema();
    let conn = Connection::open(":memory:").unwrap();
    conn.execute(create).unwrap();

    let insert = Insert::new("test_table")
        .values(vec![
            ("name", "Dayvson Spacca"),
            ("email", "spacca.dayvson@gmail.com"),
            ("age", "21"),
        ])
        .build()
        .unwrap();

    assert_eq!("INSERT INTO test_table (name, email, age) VALUES ('Dayvson Spacca', 'spacca.dayvson@gmail.com', '21');", insert);
    conn.execute(insert).unwrap();

    let select = Select::new("test_table")
        .columns("id, name, email, age, created_at")
        .build()
        .unwrap();

    let mut statement = conn.prepare(select).unwrap();

    while let Ok(State::Row) = statement.next() {
        assert_eq!(statement.read::<i64, _>("id").unwrap(), 1);
        assert_eq!(
            statement.read::<String, _>("name").unwrap(),
            "Dayvson Spacca"
        );
        assert_eq!(
            statement.read::<String, _>("email").unwrap(),
            "spacca.dayvson@gmail.com"
        );
        assert_eq!(statement.read::<i64, _>("age").unwrap(), 21);
    }
}

#[test]
fn test_select_where() {
    let create = create_test_table_schema();
    let conn = Connection::open(":memory:").unwrap();
    conn.execute(create).unwrap();

    conn.execute(
        Insert::new("test_table")
            .values(vec![
                ("name", "Dayvson Spacca"),
                ("email", "spacca.dayvson@gmail.com"),
                ("age", "21"),
            ])
            .build()
            .unwrap(),
    )
    .unwrap();

    conn.execute(
        Insert::new("test_table")
            .values(vec![
                ("name", "John doe"),
                ("email", "Jhondoe@doe.com"),
                ("age", "42"),
            ])
            .build()
            .unwrap(),
    )
    .unwrap();

    let condition = Where::new()
        .equal_to("name", "John doe")
        .and()
        .not_equal_to("age", "21")
        .or()
        .is_not_null("email")
        .build();

    let select = Select::new("test_table")
        .condition(condition)
        .build()
        .unwrap();

    assert_eq!(
        "SELECT * FROM test_table WHERE name = 'John doe' AND age != '21' OR email IS NOT NULL;",
        select
    );

    let statement = conn.prepare(select).unwrap().iter().count();

    assert_eq!(statement, 2);
}

fn create_test_table_schema() -> String {
    CreateTable::new(
        "test_table",
        vec![
            Column::new("id").integer().primary_key().auto_increment(),
            Column::new("name").text().not_null(),
            Column::new("email").text().unique().not_null(),
            Column::new("age").integer().not_null(),
            Column::new("created_at")
                .datetime()
                .default("CURRENT_TIMESTAMP"),
            Column::new("update_at").datetime(),
        ],
    )
    .if_not_exists()
    .build()
    .unwrap()
}

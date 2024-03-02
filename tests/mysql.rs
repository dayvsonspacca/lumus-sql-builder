use lua_sql_builder::mysql::{
    create::Create,
    delete::Delete,
    insert::Insert,
    join::{Join, JoinType},
    select::Select,
    update::Update,
    where_::{Combiner, Where},
};

#[test]
fn test_create_table() {
    let create = Create::new("example_table")
        .columns("id INT, name VARCHAR(255), age INT")
        .build();

    let expected_sql =
        "CREATE TABLE IF NOT EXISTS example_table (id INT, name VARCHAR(255), age INT);";

    assert_eq!(create, expected_sql);
}

#[test]
fn test_create_table_without_columns() {
    let create = Create::new("empty_table").build();

    let expected_sql = "CREATE TABLE IF NOT EXISTS empty_table;";

    assert_eq!(create, expected_sql);
}

#[test]
fn test_delete_with_where_clause() {
    let mut where_ = Where::new(Combiner::And);

    where_.equal_to("id", "42").not_equal_to("name", "John");

    let delete = Delete::new("example_table").where_(where_).build();

    let expected_sql = "DELETE FROM example_table WHERE id = 42 AND name != 'John';";

    assert_eq!(delete, expected_sql);
}

#[test]
fn test_delete_without_where_clause() {
    let delete = Delete::new("empty_table").build();

    let expected_sql = "DELETE FROM empty_table;";

    assert_eq!(delete, expected_sql);
}

#[test]
fn test_select_query() {
    let mut where_ = Where::new(Combiner::And);

    where_
        .equal_to("age", "25")
        .not_equal_to("status", "inactive");

    let select_query = Select::new()
        .distinct()
        .columns("name, age")
        .from("users")
        .join(
            "emails",
            "users.id = emails.user_id",
            lua_sql_builder::mysql::join::JoinType::Inner,
        )
        .where_(where_)
        .group("department")
        .order("age DESC")
        .limit(10)
        .offset(5)
        .build();

    let expected_sql = "SELECT DISTINCT name, age FROM users INNER JOIN emails ON users.id = emails.user_id WHERE age = 25 AND status != 'inactive' GROUP BY department ORDER BY age DESC LIMIT 10 OFFSET 5;";

    assert_eq!(select_query, expected_sql);
}

#[test]
fn test_select_query_without_where_clause() {
    let mut where_ = Where::new(Combiner::Or);
    where_.greater_than_equal("price", "1000");

    let select_query = Select::new().from("products").where_(where_).build();

    let expected_sql = "SELECT * FROM products WHERE price >= 1000;";

    assert_eq!(select_query, expected_sql);
}

#[test]
fn test_insert_query() {
    let mut insert = Insert::new("users");
    insert.values(vec![
        ["name", "John Doe"],
        ["age", "30"],
        ["email", "john@example.com"],
    ]);

    let insert_query = insert.build();

    let expected_sql =
        "INSERT INTO users (name,age,email) VALUES ('John Doe',30,'john@example.com');";

    assert_eq!(insert_query, expected_sql);
}

#[test]
fn test_where_clause_equal_to() {
    let mut where_clause = Where::new(Combiner::And);
    where_clause.equal_to("age", "25");

    let result = where_clause.build();
    let expected_sql = "WHERE age = 25";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_where_clause_not_equal_to() {
    let mut where_clause = Where::new(Combiner::Or);
    where_clause.not_equal_to("status", "inactive");

    let result = where_clause.build();
    let expected_sql = "WHERE status != 'inactive'";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_where_clause_greater_than() {
    let mut where_clause = Where::new(Combiner::And);
    where_clause.greater_than("price", "1000");

    let result = where_clause.build();
    let expected_sql = "WHERE price > 1000";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_where_clause_greater_than_equal() {
    let mut where_clause = Where::new(Combiner::Or);
    where_clause.greater_than_equal("quantity", "10");

    let result = where_clause.build();
    let expected_sql = "WHERE quantity >= 10";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_where_clause_less_than() {
    let mut where_clause = Where::new(Combiner::And);
    where_clause.less_than("rating", "3");

    let result = where_clause.build();
    let expected_sql = "WHERE rating < 3";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_where_clause_less_than_equal() {
    let mut where_clause = Where::new(Combiner::And);
    where_clause.less_than_equal("score", "80");

    let result = where_clause.build();
    let expected_sql = "WHERE score <= 80";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_where_clause_is_null() {
    let mut where_clause = Where::new(Combiner::And);
    where_clause.is_null("description");

    let result = where_clause.build();
    let expected_sql = "WHERE description ISNULL";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_where_clause_is_not_null() {
    let mut where_clause = Where::new(Combiner::And);
    where_clause.is_not_null("last_login");

    let result = where_clause.build();
    let expected_sql = "WHERE last_login IS NOT NULL";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_where_clause_in() {
    let mut where_clause = Where::new(Combiner::Or);
    where_clause.in_("category", vec!["Electronics", "Clothing", "Books"]);

    let result = where_clause.build();
    let expected_sql = "WHERE category IN ('Electronics', 'Clothing', 'Books')";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_where_clause_not_in() {
    let mut where_clause = Where::new(Combiner::And);
    where_clause.not_in("color", vec!["Red", "Green", "Blue"]);

    let result = where_clause.build();
    let expected_sql = "WHERE color NOT IN ('Red', 'Green', 'Blue')";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_inner_join() {
    let join = Join::new(
        "orders".to_string(),
        "users.id = orders.user_id".to_string(),
        JoinType::Inner,
    );
    let result = join.build();
    let expected_sql = "INNER JOIN orders ON users.id = orders.user_id ";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_left_join() {
    let join = Join::new(
        "products".to_string(),
        "categories.id = products.category_id".to_string(),
        JoinType::Left,
    );
    let result = join.build();
    let expected_sql = "LEFT JOIN products ON categories.id = products.category_id ";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_right_join() {
    let join = Join::new(
        "customers".to_string(),
        "orders.customer_id = customers.id".to_string(),
        JoinType::Right,
    );
    let result = join.build();
    let expected_sql = "RIGHT JOIN customers ON orders.customer_id = customers.id ";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_full_join() {
    let join = Join::new(
        "suppliers".to_string(),
        "products.supplier_id = suppliers.id".to_string(),
        JoinType::Full,
    );
    let result = join.build();
    let expected_sql = "FULL JOIN suppliers ON products.supplier_id = suppliers.id ";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_left_outer_join() {
    let join = Join::new(
        "departments".to_string(),
        "employees.department_id = departments.id".to_string(),
        JoinType::LeftOuter,
    );
    let result = join.build();
    let expected_sql = "LEFT OUTER JOIN departments ON employees.department_id = departments.id ";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_right_outer_join() {
    let join = Join::new(
        "countries".to_string(),
        "cities.country_id = countries.id".to_string(),
        JoinType::RightOuter,
    );
    let result = join.build();
    let expected_sql = "RIGHT OUTER JOIN countries ON cities.country_id = countries.id ";

    assert_eq!(result, expected_sql);
}

#[test]
fn test_update_query() {
    let mut update = Update::new("users");
    let mut where_ = Where::new(Combiner::And);
    where_.equal_to("id", "1");

    update
        .set(vec![
            ["name", "John Doe"],
            ["age", "30"],
            ["email", "john@example.com"],
        ])
        .where_(where_);

    let update_query = update.build();

    let expected_sql =
        "UPDATE users SET name = 'John Doe', age = 30, email = 'john@example.com' WHERE id = 1;";

    assert_eq!(update_query, expected_sql);
}

#[test]
fn test_update_query_without_where_clause() {
    let mut update = Update::new("products");
    update.set(vec![["price", "25.50"], ["quantity", "100"]]);

    let update_query = update.build();

    let expected_sql = "UPDATE products SET price = 25.50, quantity = 100;";

    assert_eq!(update_query, expected_sql);
}

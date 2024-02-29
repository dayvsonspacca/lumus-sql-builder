# Lua sql builder

Este repositório contém um projeto escrito em Rust com o objetivo de construir consultas SQL. Foi criado como parte de um esforço para aprender mais sobre a linguagem de programação Rust e suas aplicações.

## Sobre o Projeto

O projeto é um construtor de consultas SQL que permite criar consultas SQL complexas de maneira programática e intuitiva. Ele suporta uma variedade de operações SQL, incluindo seleção de colunas, junções, cláusulas WHERE, agrupamento, ordenação e muito mais.

Aqui está um exemplo de como o código pode ser usado para construir uma consulta SQL:

```rust
mod builder;

use builder::mysql::join::JoinType;
use builder::mysql::select::Select;
use builder::mysql::where_::{Combiner, Where};

fn main() {
    let mut select = Select::new();
    let mut where_ = Where::new(Combiner::And);

    where_
        .not_equal_to("email", "spacca.dayvson@gmail.com")
        .greater_than_equal("age", "2")
        .less_than("salary", "230.00")
        .less_than_equal("age", "25")
        .is_null("genre");

    select
        .columns("name, age, email, salary")
        .from("users_tb u")
        .join("emails_tb e", "e.user_id = u.user_id", "", JoinType::Left)
        .join("phones_tb p", "p.user_id = u.user_id", "", JoinType::Left)
        .where_(where_);

    println!("{}", select.build());
}
```
# Output: 
```sql
SELECT name, age, email, salary FROM users_tb u LEFT JOIN emails_tb e ON e.user_id = u.user_id LEFT JOIN phones_tb p ON p.user_id = u.user_id WHERE email != 'spacca.dayvson@gmail.com' AND age >= 2 AND salary < 230.00 AND age <= 25 AND genre ISNULL;
```

mod builder;

use builder::mysql::select::Select;
use builder::mysql::where_::{Operator, Where};
fn main() {
    let mut select = Select::new();
    let mut where_ = Where::new(Operator::AND);

    where_.equal_to("name", "2");
    select
        .columns("name, age, email")
        .from("users_tb")
        .where_(where_)
        .group("name")
        .order("name DESC, age ASC");

    println!("{}", select.build());
}

mod builder;

use builder::mysql::select::Select;
use builder::mysql::where_::{Combiner, Where};
fn main() {
    let mut select = Select::new();
    let mut where_ = Where::new(Combiner::AND);

    where_.equal_to("name", "2")
          .not_equal_to("email", "spacca.dayvson@gmail.com")
          .greater_than("age", "2")
          .greater_than_equal("age", "2")
          .less_than("salary", "230.00")
          .less_than_equal("age", "25");

    select
        .columns("name, age, email, salary")
        .from("users_tb")
        .where_(where_)
        .group("name")
        .order("name DESC, age ASC");

    println!("{}", select.build());
}

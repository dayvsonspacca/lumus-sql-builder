mod builder;

use crate::builder::mysql::join::JoinType;
use builder::mysql::select::Select;
use builder::mysql::where_::{Combiner, Where};

fn main() {
    let mut select = Select::new();
    let mut where_ = Where::new(Combiner::And);

    where_
        .equal_to("name", "2")
        .not_equal_to("email", "spacca.dayvson@gmail.com")
        .greater_than("age", "2")
        .greater_than_equal("age", "2")
        .less_than("salary", "230.00")
        .less_than_equal("age", "25")
        .is_null("genre")
        .is_not_null("name")
        .in_("name", vec!["dayvson", "iago", "oaao", "ivalber"])
        .not_in("age", vec!["20", "23", "19"]);

    select
        .columns("name, age, email, salary")
        .from("users_tb u")
        .join("emails_tb e", "e.user_id = u.user_id", "", JoinType::Left)
        .join("phones_tb p", "p.user_id = u.user_id", "", JoinType::Left)
        .where_(where_)
        .group("name")
        .order("name DESC, age ASC")
        .offset(4);

    println!("{}", select.build());
}

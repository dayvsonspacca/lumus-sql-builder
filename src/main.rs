mod builder;

use builder::mysql::select::Select;

fn main() {
    let mut select = Select::new();

    select
        .from("users_tb")
        .columns("name, age, email")
        .group("name")
        .order("name DESC, age ASC");

    println!("{}", select.build());
}

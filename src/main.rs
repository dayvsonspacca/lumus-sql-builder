mod builder;

use builder::mysql::select::Select;

fn main() {
    let mut select = Select::new();

    select
        .from("users_tb")
        .columns(vec!["name", "age", "email"]);

    println!("{}", select.build());
}

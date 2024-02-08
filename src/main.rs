mod builder;

use builder::mysql::insert::Insert;

fn main() {
    let mut insert = Insert::new("users_tb");

    insert.values(vec![
        ["name", "Dayvson Spacca"],
        ["age", "20"],
    ]);

    println!("{}", insert.build());
}

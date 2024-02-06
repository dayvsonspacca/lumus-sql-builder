mod builder;

use builder::mysql::insert::Insert;

fn main() {
    let mut insert = Insert::new("users_tb");

    insert.values(vec![
        ["name".to_string(), "Dayvson Spacca".to_string()],
        ["age".to_string(), "20".to_string()],
    ]);

    println!("{}", insert.build());
}

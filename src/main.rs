use crate::builder::mysql::{update::Update, where_::{self, Where}};

mod builder;

fn main() {
    let mut update = Update::new("users_tb");
    let mut where_ = Where::new(where_::Combiner::And);

    where_.equal_to("user_id", "2");

    update.set(vec![
        ["nome", "Luiz Gonzaga"],
        ["age", "2"]
    ]);
    update.where_(where_);

    println!("{}", update.build());
}

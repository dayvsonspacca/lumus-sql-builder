use crate::builder::mysql::{
    delete::Delete,
    where_::{self, Where},
};

mod builder;

fn main() {
    let mut delete = Delete::new("users_tb");
    let mut where_ = Where::new(where_::Combiner::And);

    where_.equal_to("user_id", "2");

    delete.where_(where_);

    println!("{}", delete.build());
}

// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

mod delicious_snacks {

    pub mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

// TODO: Add the following two `use` statements after fixing them.
pub use crate::delicious_snacks::fruits::PEAR as fruit;
pub use crate::delicious_snacks::veggies::CUCUMBER as veg;

fn main() {
    println!("favorite snacks: {} and {}", fruit, veg,);
}

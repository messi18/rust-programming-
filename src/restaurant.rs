#[cfg(test)]
mod tests {
    use crate::restaurant::eat_at_restaurant;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn test_eat() {
        let meal = eat_at_restaurant();
        assert_eq!("Wheat" ,meal.toast)
    }
}

mod back_of_house {
    use std::fmt::{Display, Formatter};

    pub struct Breakfast {
        pub toast: String,
        pub(crate) seasonal_fruit: String,
    }

    impl Display for Breakfast {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "toast:{}, fruit:{}", self.toast, self.seasonal_fruit)
        }
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub use back_of_house::Breakfast;
pub fn eat_at_restaurant() -> Breakfast {
    let mut breakfast = Breakfast::summer("Roy");
    breakfast.toast = String::from("Wheat");
    println!("I would like {} toast please", breakfast.toast);

    breakfast.seasonal_fruit = String::from("blueberries");
    println!("I would like {} seasonal_fruit please", breakfast.seasonal_fruit);
    return breakfast;
}
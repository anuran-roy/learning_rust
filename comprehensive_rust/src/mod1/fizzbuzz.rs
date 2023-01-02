pub mod fizzbuzz {
    // Function definition
    pub fn is_divisible_by(lhs: u128, rhs: u128) -> bool {
        if rhs == 0 {
            return false;
        }

        return lhs % rhs == 0;
    }

    pub fn fizz_buzz(num: u128) -> () {
        match (is_divisible_by(num, 3), is_divisible_by(num, 5)) {
            (true, true) => println!("fizz_buzz!"),
            (false, true) => println!("Buzz!"),
            (true, false) => println!("Fizz!"),
            (false, false) => println!("{num} is neither Fizz nor Buzz"),
        }
    }

    pub fn list_fizz_buzz(llim: u128, ulim: u128) {
        for i in llim..=ulim {
            fizz_buzz(i);
        }
    }
}

pub mod struct_impl {
    // Struct definition
    pub struct Rectangle {
        pub length: f64,
        pub breadth: f64,
    }

    // Method implementation for Rectangle struct
    impl Rectangle {
        pub fn area(&self) -> f64 {
            return self.length * self.breadth;
        }

        pub fn add_length(&mut self, delta: f64) {
            self.length += delta;
        }

        pub fn add_breadth(&mut self, delta: f64) {
            self.breadth += delta;
        }
    }

    pub fn sample() {
        let mut rect = Rectangle {
            length: 10.0,
            breadth: 20.0,
        };

        println!("{:?}", rect.area());
        rect.add_breadth(5.5);
        rect.add_length(5.5);
        println!("{:?}", rect.area());
    }
}

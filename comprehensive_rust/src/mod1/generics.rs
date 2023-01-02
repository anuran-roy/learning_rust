pub mod generics {
    // Type Generics are supported,but not function overloading.
    // Default values are also not supported.
    pub fn pick_one<T>(a: T, b: T) -> T {
        if std::process::id() % 2 == 0 {
            return a;
        } else {
            return b;
        }
    }

    pub fn sample() {
        // Overloading - No overloading support

        println!("{:}, {:}", pick_one("hello", "bye"), pick_one(1, 2));
    }
}

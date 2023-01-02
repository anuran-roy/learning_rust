pub mod ownership {
    struct Point(i32, i32);

    pub fn func1() {
        let s1: String = String::from("Hello!");
        let s2: String = s1;
        println!("s2: {s2}");
        // println!("s1: {s1}");
    }
    pub fn func2() {
        {
            let p = Point(3, 4);
            println!("x: {}", p.0);
        }
        // println!("y: {}", p.1); // Scope not found
    }

    pub fn sample() {
        func1();
        func2();
    }
}

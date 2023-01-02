// A borrowed value has a lifetime:

// 1. The lifetime can be elided: add(p1: &Point, p2: &Point) -> Point.
// 2. Lifetimes can also be explicit: &'a Point, &'document str.
// 3. Read &'a Point as “a borrowed Point which is valid for at least the lifetime a”.
// 4. Lifetimes are always inferred by the compiler: you cannot assign a lifetime yourself.
// 5. Lifetime annotations create constraints; the compiler verifies that there is a valid solution.

pub mod lifetimes {
    #[derive(Debug)]
    pub struct Point(i32, i32);

    pub fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
        if p1.0 < p2.0 {
            p1
        } else {
            p2
        }
    }

    pub fn function_call_sample() {
        let p1: Point = Point(10, 10);
        let p2: Point = Point(20, 20); // Put into different scope
        let p3: &Point = left_most(&p1, &p2);
        println!("left-most point: {:?}", p3);
    }

    // Data Structures Lifetimes
    #[derive(Debug)]
    pub struct Highlight<'doc>(&'doc str);

    pub fn erase(text: String) {
        println!("Bye {text}!");
    }

    pub fn data_structures_sample() {
        let text = String::from("The quick brown fox jumps over the lazy dog.");
        let fox = Highlight(&text[4..19]);
        let dog = Highlight(&text[35..43]);
        // erase(text);
        println!("{fox:?}");
        println!("{dog:?}");
    }
}

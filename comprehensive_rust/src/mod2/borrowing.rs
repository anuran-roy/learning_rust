pub mod borrowing {
    #[derive(Debug)]
    pub struct Point(i32, i32);

    pub fn borrowing_add(a: &Point, b: &Point) -> Point {
        return Point(a.0 + b.0, a.1 + b.1);
    }

    pub fn sample() -> () {
        let p1 = Point(3, 4);
        let p2 = Point(10, 20);
        let p3 = borrowing_add(&p1, &p2);
        println!("{p1:?} + {p2:?} = {p3:?}");
    }
}

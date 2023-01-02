pub mod iterators {
    use std::clone;

    #[derive(Debug)]
    pub struct Item {
        name: String,
        price: f64,
    }

    #[derive(Debug)]
    pub struct ShoppingList {
        list: Vec<Item>,
        name: Option<String>,
    }

    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    // The trait IntoIterator tells you how to create the iterator.
    // The syntax here means that every implementation of IntoIterator must declare two types:
    // Item: the type we iterate over, such as i8,
    // IntoIter: the Iterator type returned by the into_iter method.
    // Note that IntoIter and Item are linked: the iterator must have the same Item type, which means that it returns Option<Item>

    pub trait IntoIterator {
        type Item;
        type IntoIter: Iterator<Item = Self::Item>;
        fn into_iter(self);
    }

    pub fn sample() {
        let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
        let mut iter = v.clone().into_iter(); // Clone and create an iterator on top of the clone, otherwise ownership would be transferred, rendering v unable to be used later.

        let v0: Option<String> = iter.next();
        println!("v0: {v0:?}");

        for i in iter {
            println!("element: {i:?}");
        }

        for item in v {
            println!("element: {item:?}");
        }
    }
}

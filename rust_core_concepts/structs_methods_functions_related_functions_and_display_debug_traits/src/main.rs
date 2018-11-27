use std::fmt::{Display, Formatter, Result};

struct PlainObject {
    width: u32,
    height: u32,
}

fn plain_area(obj: &PlainObject) -> u32 {
    obj.width * obj.height
}

#[derive(Debug)]
struct FancyObject {
    width: u32,
    height: u32,
}

// Methods
impl FancyObject {
    fn fancy_area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!(
            "{}x{} with area: {}\n",
            self.width,
            self.height,
            self.fancy_area()
        );
    }
}

// Related Functions
impl FancyObject {
    fn new(width: u32, height: u32) -> FancyObject {
        FancyObject {
            // NOTE: Just as with ES6 and its object literal
            //       property value shorthand,
            //       we can use a shorthand for struct properties
            //       if their name and value are identical
            width,
            height,
        }
    }
}

impl Display for FancyObject {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "({}, {}) and Area: {}",
            self.width,
            self.height,
            self.fancy_area()
        )
    }
}

fn main() {
    let plain_o = PlainObject {
        width: 35,
        height: 55,
    };

    let fancy_o = FancyObject::new(45, 65);

    println!(
        "Plain Object:\n{}x{} with area: {}\n",
        plain_o.width,
        plain_o.height,
        plain_area(&plain_o)
    );

    println!(
        "Fancy Object (made with new constructor):\n{}x{} with area: {}\n",
        fancy_o.width,
        fancy_o.height,
        fancy_o.fancy_area()
    );

    println!("Using Fancy Object's `show` method:");
    fancy_o.show();

    println!(
        "Debugging the Fancy Object with Debug trait:\n{:?}\n",
        fancy_o
    );
    println!(
        "Pretty Printing the Fancy Object with Debug trait:\n{:#?}\n",
        fancy_o
    );
    println!(
        "Pretty Printing the Fancy Object with Debug trait:\n{}\n",
        fancy_o
    );
}

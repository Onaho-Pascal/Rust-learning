// Define traits (blueprints for behavior)
trait Fly {
    fn fly(&self);
}

trait Swim {
    fn swim(&self);
}

// Define concrete types
struct Bird {
    name: String,
}

struct Fish {
    name: String,
}

// Implement methods directly on Bird
impl Bird {
    fn speak(&self) {
        println!("{} says: karo karo!", self.name);
    }
}

// Implement traits for types
impl Fly for Bird {
    fn fly(&self) {
        println!("{} is flying high! 🕊️", self.name);
    }
}

impl Swim for Fish {
    fn swim(&self) {
        println!("{} is swimming fast! 🐟", self.name);
    }
}

fn main() {
    let parrot = Bird { name: "Polly".to_string() };
    let goldfish = Fish { name: "Nemo".to_string() };

    // Call method directly on Bird
    parrot.speak();

    // Call trait methods
    parrot.fly();
    goldfish.swim();
}

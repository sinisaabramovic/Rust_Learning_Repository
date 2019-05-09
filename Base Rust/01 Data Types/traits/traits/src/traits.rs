
// its a class :D
trait Mamel {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn to_string(&self) -> String;
    // does not need to be implemented
    fn talk(&self) {
        println!("Base mamel cannot talk");
    }
}

struct Animal {
    name: &'static str
}

struct Human {
    name: &'static str,
    sur_name: &'static str
}

impl Mamel for Human {
    
    fn create(name: &'static str) -> Human {
        Human{name: name, sur_name: "Doe"}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn to_string(&self) -> String {
        self.name.to_string() + ", " + &self.sur_name.to_string()
    }
    
    fn talk(&self) {
        println!("Hi I am {} {}", self.name, self.sur_name);
    }
}

impl Mamel for Animal {

    fn create(name: &'static str) -> Animal {
        Animal{name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn to_string(&self) -> String {
        self.name.to_string()
    }
    
    fn talk(&self) {
        println!("I am {} animal", self.name);
    }
}

// lets create trait or better say interface
trait Summable<T> {
    fn sum(&self) -> T;
}

// Extensions of Vec or
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self {
            result += *x;
        }

        return result;
    }
}

pub fn run() {
    //let human = Human{name: "John", sur_name: "Doe"};
    let human = Human::create("John");
    let animal = Animal{name: "Dog"};

    human.talk();
    animal.talk();

    let a = vec![1,2,3];
    println!("Sum = {}", a.sum());
}
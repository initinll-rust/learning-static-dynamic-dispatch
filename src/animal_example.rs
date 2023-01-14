
pub trait Animal {
    fn talk(&self);
}

pub struct Cat {}
pub struct Dog {}

impl Animal for Cat {
    fn talk(&self) {
        println!("meow");
    }
}

impl Animal for Dog {
    fn talk(&self) {
        println!("bark");
    }
}

// static dispatch - function input
pub fn animal_talk_1(a: &impl Animal) {
    a.talk();
}

// static dispatch - function output
pub fn animal_1() -> impl Animal {
    Dog {}
}

//dynamic dispatch - function input
pub fn animal_talk_2(a: &dyn Animal) {
    a.talk();
}

//dynamic dispatch - function output
pub fn animal_2() -> Box<dyn Animal> {
    if 1==1 {
        return Box::new(Cat{});
    }

    return Box::new(Dog{});
}


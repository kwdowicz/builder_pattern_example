use crate::Topping::{Ham, Pepper, Pineapple};

#[derive(Debug)]
enum Topping {
    Peperoni,
    Ham,
    Pineapple,
    Oregano,
    Pepper,
}

#[derive(Debug)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
struct Pizza {
    name: String,
    size: Size,
    topping_1: Option<Topping>,
    topping_2: Option<Topping>,
    topping_3: Option<Topping>,
}

struct PizzaBuilder {
    name: String,
    size: Size,
    topping_1: Option<Topping>,
    topping_2: Option<Topping>,
    topping_3: Option<Topping>,
}

impl PizzaBuilder {
    fn new(name: &str) -> PizzaBuilder {
        PizzaBuilder {
            name: name.to_string(),
            size: Size::Medium,
            topping_1: None,
            topping_2: None,
            topping_3: None,
        }
    }

    fn size(mut self, size: Size) -> PizzaBuilder {
        self.size = size;
        self
    }

    fn topping_1(mut self, topping: Topping) -> PizzaBuilder {
        self.topping_1 = Some(topping);
        self
    }

    fn topping_2(mut self, topping: Topping) -> PizzaBuilder {
        self.topping_2 = Some(topping);
        self
    }

    fn topping_3(mut self, topping: Topping) -> PizzaBuilder {
        self.topping_3 = Some(topping);
        self
    }

    fn build(self) -> Pizza {
        Pizza {
            name: self.name,
            size: self.size,
            topping_1: self.topping_1,
            topping_2: self.topping_2,
            topping_3: self.topping_3,
        }
    }
}

fn main() {
    let pizza1 = PizzaBuilder::new("Nice")
        .topping_2(Topping::Ham)
        .topping_1(Topping::Peperoni)
        .build();
    println!("Pizza 1: {:#?}", pizza1);

    let pizza2 = PizzaBuilder::new("Better")
        .topping_2(Topping::Ham)
        .topping_1(Topping::Peperoni)
        .size(Size::Large)
        .build();
    println!("Pizza 2: {:#?}", pizza2);

    let pizza3 = PizzaBuilder::new("Best")
        .topping_2(Topping::Ham)
        .topping_1(Topping::Peperoni)
        .topping_3(Pineapple)
        .topping_3(Pepper)
        .size(Size::Large)
        .build();
    println!("Pizza 3: {:#?}", pizza3);
}

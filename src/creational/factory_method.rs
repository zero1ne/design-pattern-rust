
pub trait IHotDrink {
    fn consume(&self);
}

pub struct Tea;

impl IHotDrink for Tea {
    fn consume(&self) {
        println!("This tea is nice but I'd prefer it with milk.");
    }
}

pub struct Coffee;

impl IHotDrink for Coffee {
    fn consume(&self) {
        println!("This coffee is delicious!");
    }
}

pub trait IHotDrinkFactory {
    fn prepare(&self, amount: u32) -> Box<dyn IHotDrink>;
}

pub struct TeaFactory;

impl IHotDrinkFactory for TeaFactory {
    fn prepare(&self, amount: u32) -> Box<dyn IHotDrink> {
        println!("Put in tea bag, boil water, pour {}ml, add lemon, enjoy!", amount);
        Box::new(Tea)
    }
}

pub struct CoffeeFactory;

impl IHotDrinkFactory for CoffeeFactory {
    fn prepare(&self, amount: u32) -> Box<dyn IHotDrink> {
        println!("Grind some beans, boil water, pour {}ml, add cream and sugar, enjoy!", amount);
        Box::new(Coffee)
    }
}

#[test]
fn factory_method_test() {
    let tea_factory = TeaFactory;
    let coffee_factory = CoffeeFactory;

    let tea = tea_factory.prepare(500);
    let coffee = coffee_factory.prepare(200);

    tea.consume();
    coffee.consume();
}
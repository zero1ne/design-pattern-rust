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

pub trait IHotDrinkAbstractFactory {
    fn create_hot_drink_factory(&self) -> Box<dyn IHotDrinkFactory>;
}

pub struct HotDrinkAbstractFactory;

impl IHotDrinkAbstractFactory for HotDrinkAbstractFactory {
    fn create_hot_drink_factory(&self) -> Box<dyn IHotDrinkFactory> {
        Box::new(TeaFactory)
    }
}

pub struct HotDrinkWithMilkAbstractFactory;

impl IHotDrinkAbstractFactory for HotDrinkWithMilkAbstractFactory {
    fn create_hot_drink_factory(&self) -> Box<dyn IHotDrinkFactory> {
        Box::new(CoffeeFactory)
    }
}

#[test]
fn abstract_factory_test() {
    let abstract_factory = HotDrinkAbstractFactory;
    let hot_drink_factory = abstract_factory.create_hot_drink_factory();
    let hot_drink = hot_drink_factory.prepare(500);
    hot_drink.consume();

    let abstract_factory = HotDrinkWithMilkAbstractFactory;
    let hot_drink_factory = abstract_factory.create_hot_drink_factory();
    let hot_drink = hot_drink_factory.prepare(200);
    hot_drink.consume();
}

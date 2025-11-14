use std::collections::HashMap;

use crate::helper::{enum_type::Manager, input::get_input};

#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
}
struct Products {
    inventory: HashMap<String, Product>,
}
impl Products {
    fn new() -> Self {
        Self {
            inventory: HashMap::new(),
        }
    }
    fn add(&mut self, product: Product) {
        self.inventory.insert(product.name.to_string(), product);
    }
    fn view(&self) -> Vec<&Product> {
        self.inventory.values().collect()
    }
    fn edit(&mut self, name: &str, price: f64) {
        match self.inventory.get_mut(name) {
            Some(product) => {
                product.price = price;
            }
            None => (),
        }
    }
    fn delete(&mut self, name: &str) {
        self.inventory.remove(name);
    }
}
mod manager {
    use crate::helper::input::get_input;
    use crate::product::{Product, Products};

    pub fn add_product(products: &mut Products) {
        println!("Enter name of product");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter price of product");
        let price_input = match get_input() {
            Some(input) => input,
            None => return,
        };
        let price: f64 = match price_input.parse() {
            Ok(p) => p,
            Err(_) => return,
        };
        let product = Product { name, price };
        products.add(product);
    }
    pub fn view_product(products: &Products) {
        let products = products.view();
        for product in products {
            println!("{:?}", product);
        }
    }
    pub fn edit_product(products: &mut Products) {
        for product in products.view() {
            println!("{:?}", product);
        }
        println!("Enter name of product to edit");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter new price of product");
        let price_input = match get_input() {
            Some(input) => input,
            None => return,
        };
        let price: f64 = match price_input.parse() {
            Ok(p) => p,
            Err(_) => return,
        };
        products.edit(&name, price);
    }
    pub fn delete_product(products: &mut Products) {
        for product in products.view() {
            println!("{:?}", product);
        }
        println!("Enter name of product to delete");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        products.delete(&name);
    }
}
pub fn main_product() {
    let mut students = Products::new();
    loop {
        Manager::show();
        let input = get_input().expect("Please input your option");
        match Manager::choice(input.as_str()) {
            Some(Manager::Add) => manager::add_product(&mut students),
            Some(Manager::View) => manager::view_product(&students),
            Some(Manager::Edit) => manager::edit_product(&mut students),
            Some(Manager::Delete) => manager::edit_product(&mut students),
            None => break,
        }
    }
}

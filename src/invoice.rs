use crate::helper::{enum_type::Manager, input::get_input};
#[derive(Debug)]
pub struct Item {
    name: String,
    quantity: u32,
    price: f64,
}
pub struct Invoice {
    items: Vec<Item>,
}
impl Invoice {
    fn new() -> Self {
        Self { items: Vec::new() }
    }
    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
    fn view_items(&self) -> &Vec<Item> {
        &self.items
    }
    fn edit_item(&mut self, name: &str, quantity: u32, price: f64) {
        for item in &mut self.items {
            if item.name == name {
                item.quantity = quantity;
                item.price = price;
            }
        }
    }
    fn delete_item(&mut self, name: &str) {
        self.items.retain(|item| item.name != name);
    }
    fn total_amount(&self) -> f64 {
        self.items
            .iter()
            .map(|item| item.quantity as f64 * item.price)
            .sum()
    }
}
trait Printable {
    fn print(&self);
}
impl Printable for Invoice {
    fn print(&self) {
        println!("{:=^50}", " INVOICE "); // Tiêu đề hóa đơn giữa
        println!("{:<20} | {:<10} | {:<10}", "Item", "Quantity", "Price");
        println!("{}", "-".repeat(50));

        for item in &self.items {
            println!(
                "{:<20} | {:<10} | {:<10.2}",
                item.name,
                item.quantity,
                item.price
            );
        }

        println!("{}", "-".repeat(50));
        println!("{:<20} | {:<10} | {:<10.2}", "Total", "", self.total_amount());
        println!("{:=^50}", "");
    }
}

mod manager {
    use crate::{
        helper::input::get_input,
        invoice::{Invoice, Item},
    };

    pub fn add_product(invoice: &mut Invoice) {
        println!("Enter item name:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter quantity:");
        let quantity_input = match get_input() {
            Some(input) => input,
            None => return,
        };
        let quantity: u32 = match quantity_input.parse() {
            Ok(q) => q,
            Err(_) => return,
        };
        println!("Enter price:");
        let price_input = match get_input() {
            Some(input) => input,
            None => return,
        };
        let price: f64 = match price_input.parse() {
            Ok(p) => p,
            Err(_) => return,
        };
        let item = Item {
            name,
            quantity,
            price,
        };
        invoice.add_item(item);
    }
    pub fn view_product(invoice: &Invoice) {
        for product in invoice.view_items() {
            println!("{:?}", product);
        }
    }
    pub fn edit_product(invoice: &mut Invoice) {
        println!("Enter item name to edit:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        println!("Enter new quantity:");
        let quantity_input = match get_input() {
            Some(input) => input,
            None => return,
        };
        let quantity: u32 = match quantity_input.parse() {
            Ok(q) => q,
            Err(_) => return,
        };
        println!("Enter new price:");
        let price_input = match get_input() {
            Some(input) => input,
            None => return,
        };
        let price: f64 = match price_input.parse() {
            Ok(p) => p,
            Err(_) => return,
        };
        invoice.edit_item(&name, quantity, price);
    }
    pub fn delete_product(invoice: &mut Invoice) {
        println!("Enter item name to delete:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        invoice.delete_item(&name);
    }
    pub fn total_amount(invoice: &Invoice) {
        let total = invoice.total_amount();
        println!("Total Amount: {}", total);
    }
}
pub fn main_invoice() {
    let mut invoice = Invoice::new();
    loop {
        Manager::show();
        let input = get_input().expect("Please input your option");
        match Manager::choice(input.as_str()) {
            Some(Manager::Add) => manager::add_product(&mut invoice),
            Some(Manager::View) => manager::view_product(&invoice),
            Some(Manager::Edit) => manager::edit_product(&mut invoice),
            Some(Manager::Delete) => manager::delete_product(&mut invoice),
            Some(Manager::Total) => manager::total_amount(&invoice),
            Some(Manager::Print) => invoice.print(),
            None => break,
        }
    }
}

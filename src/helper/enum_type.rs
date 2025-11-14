// enum_type.rs
#[derive(Debug)]
pub enum Manager {
    Add,
    View,
    Edit,
    Delete,
    Total,
    Print
}
impl Manager {
    /// Hiển thị menu
    pub fn show() {
        println!("{}", "=".repeat(40));
        println!("{:^40}", "MANAGER PANEL"); // Tiêu đề giữa
        println!("{}", "=".repeat(40));
        println!("{:<3} - {:<30}", "1", "Add Item");
        println!("{:<3} - {:<30}", "2", "View Items");
        println!("{:<3} - {:<30}", "3", "Edit Item");
        println!("{:<3} - {:<30}", "4", "Delete Item");
        println!("{:<3} - {:<30}", "5", "Total Amount");
        println!("{:<3} - {:<30}", "6", "Print Invoice");
        println!("{:<3} - {:<30}", "0", "Exit");
        println!("{}", "=".repeat(40));
        print!("Enter your choice (0-6): ");
    }

    /// Chuyển input string thành Option<Manager>
    pub fn choice(input: &str) -> Option<Manager> {
        match input.trim() {
            "1" => Some(Manager::Add),
            "2" => Some(Manager::View),
            "3" => Some(Manager::Edit),
            "4" => Some(Manager::Delete),
            "5" => Some(Manager::Total),
            "6" => Some(Manager::Print),
            "0" => None, // Thoát
            _ => {
                println!("Invalid choice! Please enter 0-6.");
                None
            }
        }
    }
}

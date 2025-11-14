// enum_type.rs
#[derive(Debug)]
pub enum Manager {
    Add,
    View,
    Edit,
    Delete,
}

impl Manager {
    // Hiển thị menu
    pub fn show() {
        println!("");
        println!("== Manager Panel ==");
        println!("");
        println!("1. Add");
        println!("2. View");
        println!("3. Edit");
        println!("4. Delete");
        println!("");
        println!("Please Enter Your Choice:");
        println!("");
    }

    // Chuyển input string thành Option<Manager>
    pub fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::Add),
            "2" => Some(Manager::View),
            "3" => Some(Manager::Edit),
            "4" => Some(Manager::Delete),
            _ => None,
        }
    }
}

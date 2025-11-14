use std::collections::HashMap;
use crate::helper::{enum_type::Manager, input::{get_input, get_int}};

#[derive(Debug)]
pub struct Student {
    name: String,
    age: i32,
}
pub struct Students {
    class: HashMap<String, Student>,
}
impl Students {
    fn new() -> Self {
        Self {
            class: HashMap::new(),
        }
    }
    fn add(&mut self, student: Student) {
        self.class.insert(student.name.to_string(), student);
    }
    fn view(&self) -> Vec<&Student> {
        self.class.values().collect()
    }
    fn edit(&mut self, name: &str, age: i32) -> bool {
        match self.class.get_mut(name) {
            Some(student) => {
                student.age = age;
                true
            }
            None => false,
        }
    }
    fn delete(&mut self, name: &str) -> bool {
        self.class.remove(name).is_some()
    }
}
mod manager {
    use crate::{helper::input::{get_input, get_int}, student::{Student, Students}};



    pub fn add_student(students: &mut Students) {
        println!("Enter name of student");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let age = match get_int() {
            Some(input) => input,
            None => return,
        };
        let student = Student { name, age: age };
        students.add(student);
    }
    pub fn view_student(students: &Students) {
        let students = students.view();
        for student in students {
            println!("{:?}", student);
        }
    }
    pub fn edit_student(students: &mut Students) {
        for student in students.view() {
            println!("{:?}", student);
        }
        println!("Enter name of student");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let age = match get_int() {
            Some(input) => input,
            None => return,
        };
        if students.edit(&name, age) {
            println!("Student has edit");
        } else {
            println!("not found")
        }
    }
    pub fn delete_student(students: &mut Students) {
        for student in students.view() {
            println!("{:?}", student);
        }
        println!("Enter name of student");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        if students.delete(&name) {
            print!("delete student {}", name)
        }
    }
}



fn program() {
    let mut students = Students::new();
    loop {
        Manager::show();
        let input = get_input().expect("Please input your option");
        match Manager::choice(input.as_str()) {
            Some(Manager::Add) => manager::add_student(&mut students),
            Some(Manager::View) => manager::view_student(&students),
            Some(Manager::Edit) => manager::edit_student(&mut students),
            Some(Manager::Delete) => manager::delete_student(&mut students),
            None => break,
        }
    }
}

pub fn student_main() {
    program();
}




/* // https://pastebin.com/WFxh3FnB
use std::io;
use std::collections::HashMap;
 
fn main() {
    let mut database = HashMap::new();
 
    let help = String::from("Usage: 
            Add names using: add <name> to <department> or add <name> <department>
            Get names using: get <department> or 'get' to list all names in all departments.
            Exit by pressing return or type 'exit'");
 
    println!("{}", help);
 
    loop {
 
        let mut raw_data = String::new();
        io::stdin().read_line(&mut raw_data).expect("Err");
 
        let input: Vec<_> = raw_data.trim().split(' ').collect();
 
        match input[0].as_ref() {
            "add" => {
                if input.len() == 3 || input.len() == 4 {
                    println!("Adding {} to {}", input[1], input[input.len() - 1]);
                    let data = database.entry(input[input.len() - 1].to_string()).or_insert(vec![]);
                    data.push(input[1].to_string());
                } else {
                    println!("{}", help);
                }
            }
 
            "get" => {
                if input.len() == 2 {
                    println!("Getting names from {}", input[1]);
 
                    for value in database.get(input[1]) {
                        for i in value {
                            println!("{}", i)
                        }
                    }
                } else {
                    for (key, value) in &database {
                        println!("Department: {}", key);
                        for name in value.iter() {
                            println!("{}", name)
                        }
                    }
                }
            }
            "exit" => {
                break
            }
 
            _ => break
        }
 
    };
 
    println!("Exiting..");
} */

/* // https://github.com/nyambura00/employee_interface/blob/main/src/main.rs
use std::collections::HashMap;
use std::io;

/*struct Employee{
    name: String,
    department: String,
}*/
fn main() {
    //hashmap of employee name, department
    let mut employees = HashMap::new();
    loop {
        println!("Enter a command: ");
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        let command = command.trim().to_string();
        if command == "quit" {
            break;
        }
        let mut split_command = command.split_whitespace();
        let command = split_command.next().unwrap();

        //pattern match add, list and delete employees
        match command {
            "add" => {
                let name = split_command.next().unwrap();
                let department = split_command.next().unwrap();
                employees.insert(name.to_string(), department.to_string());
                //print new employee
                println!("{} added to {}", name, department);
            }
            "list" => {
                let specific_department = split_command.next().unwrap();
                let mut employees_in_department = Vec::new();
                for (name, department) in employees.iter() {
                    if department == specific_department {
                        employees_in_department.push(name.to_string());
                    }
                }
                employees_in_department.sort();
                for name in employees_in_department {
                    println!("{}", name);
                }
            }
            "delete" => {
                let name = split_command.next().unwrap();
                employees.remove(name);
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
} */
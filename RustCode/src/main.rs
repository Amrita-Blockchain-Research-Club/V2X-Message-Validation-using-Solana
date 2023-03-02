#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Voting {
    Yes: i64,
    No: i64,
}

fn main(){
    let vec = inputTovector();
    println!("The vector is: {:?}", vec);

    println!("\n<__________Voting Starts___________>\n");

    let map = voting(&vec);

    println!("\n<__________Voting Ends___________>\n");

    for (key, value) in &map{
        println!("{:?}: {:?}", key, value);
    }
}

fn inputTovector() -> Vec<String> {
    let mut vec = Vec::new();

    println!("How many items you need to Input: ");
    let mut NumbOfItems = String::new();
    io::stdin().read_line(&mut NumbOfItems).expect("Failed to read line");
    let num: i32 = NumbOfItems.trim().parse().expect("Please type a number");

    let mut i: i32 = 0;
    while i < num {
        println!("Enter the item: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        vec.push(input.trim().to_string());
        i += 1;
    }
    vec
}

fn voting(vec: &Vec<String>) -> HashMap<&String, Voting>{

    let mut map = HashMap::new();
    for i in vec.iter(){
        let mut yes: i64 = 0;
        let mut no: i64 = 0;

        println!(":: For item {}\nEnter 1 for yes\n0 for no\nanything else to stop voting: ",i);
        loop{
            let mut input = String::new();
            println!(">>Vote: ");
            io::stdin().read_line(&mut input).expect("Failed to read line");

            if let Ok(choice) = input.trim().parse::<i64>() {
                if choice == 1 {
                    yes += 1;
                } else if choice == 0 {
                    no += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        map.insert(i, Voting{Yes:yes, No:no});

        if yes > no {
            println!("----->> {} is selected",i);
        } 
        else if no == yes {
            println!("----->> {} has equal voting",i);
        }
        else{
            println!("----->> {} is not selected",i);
        }
        println!("\n<--------------------->\n");
    }
    map
}

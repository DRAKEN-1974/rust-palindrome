use std::io;
fn main (){
    println!("Here we will check wether the enter string is palindrome or not!!!");
    println!("Intresting stuff");
    println!("Enter the value lets check it !!!!~~!!");
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Cant get what you have entered");
    let value = value.trim();
    let reverse:String = value.chars().rev().collect();
    if reverse == value {
        println!("They are same ");
    }
    else {
        {
            println!("they are differnet" );
        }
    }
}
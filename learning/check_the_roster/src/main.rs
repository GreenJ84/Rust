use std::env;
use std::fs;

fn main() {
    if env::args().len() < 2{
        print!("Program requires 2 arguments");
        std::process::exit(1);
    }

    let filename = env::args().nth(1).unwrap();
    let search_name = env::args().nth(2).unwrap();

    let contents = fs::read_to_string(format!("src/{}", filename)).unwrap();

    if contents.contains(&search_name){
        print!("\nYes, {} did walk on the moon!", search_name)
    } else {
        print!("\nNo, {} did not walk on the moon.", search_name)
    }
}

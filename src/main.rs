use serde::{Serialize, Deserialize};
use std::fs;
use std::env;


#[derive(Serialize, Deserialize, Debug)]
struct Pattern{
  driver: String,
  path: String,
}

fn main() {
    let content = fs::read_to_string("Dasafile.json").expect("Somthing went wrong reading the file");

    let mut pattern: Pattern = serde_json::from_str(&content).unwrap();
    let _model = fs::read_to_string(&pattern.path).expect("No Value");


    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            info(&mut pattern, _model);
        }
        _ => {
            
        }
    }   

}

fn info(pt: &mut Pattern, model: String){
    println!("Driver: {}", pt.driver);
    println!("Path: {}", pt.path);
    println!("Models: {}", model);
}

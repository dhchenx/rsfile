use std::io::{Error, stdin};
mod rsio;
use rsio::*;

fn main(){
    println!("Hello, rsfile!");
    let args=std::env::args();
    println!("{:?}",args);

}

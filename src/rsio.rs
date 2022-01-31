#[cfg(test)]
mod tests {
    use crate::rsio::*;

    # [test]
    fn test_input_line(){
        let line=input_line();
        println!("input  = {}",line);
    }

    # [test]
    fn test_input_line_with_msg(){
        let line=input_line_with_msg("Please input a line:");
        println!("input  = {}",line);
    }

    # [test]
    fn test_input_binary(){
        let content = read_binary("data/test.txt");
        println!("{:?}", content);
    }

    # [test]
    fn test_write_once(){
        write_text_once("data/test2.txt","Hello, Rust!")
;    }

    # [test]
    fn test_write_lines(){
        let mut lines:Vec<&str>=Vec::new();
        lines.push("a");
        lines.push("b");
        append_text("data/test2.txt",lines);

    }

}

use std::borrow::{Borrow, BorrowMut};
use std::io::stdin;


///
/// obtain a line string from terminal
///
pub fn input_line()->String{
    let mut str_buf = String::new();
    stdin().read_line(&mut str_buf)
        .expect("Failed to read line.");
    return str_buf;
}


///
/// obtain a list of strings from a input line in the terminal
///
pub fn input()->Vec<String>{
    let mut input=String::new();
    stdin().read_line(&mut input).unwrap();
    let mut s = input.split_whitespace();
    let mut values:Vec<String>=Vec::new();
    for v in s{
        let a:&str=v.clone();
        values.push(String::from(a));
    }
    return values;
}

///
/// a simplified print!(s) function like using in Python
///
pub fn println(s:&str){
    println!("{}",s);
}

///
/// a simplified println!(s) function like using in Python
///
pub fn print(s:&str){
    print!("{}",s)
}

///
/// a simplified function for eprintln!("{}",s)
///
pub fn eprintln(s:&str){
    eprintln!("{}",s)
}

///
/// obtain a line from the terminal after a message
///
pub fn input_line_with_msg(msg:&str)->String{
    let mut str_buf = String::new();
    println!("{}",msg);
    stdin().read_line(&mut str_buf)
        .expect("Failed to read line.");
    return str_buf;
}

use std::fs;

///
/// read binary from a file
///
pub fn read_binary(filepath:&str)->Vec<u8>{
    let content = fs::read(filepath).unwrap();
    return content;
}

///
/// write a text file once, which will replace
///
pub fn write_text_once(filepath:&str,content:&str){
    fs::write(filepath, content)
        .unwrap();
}

use std::io::prelude::*;
use std::fs::OpenOptions;

///
/// append a list of lines into a file
/// # Example:
/// ```
///  use rsfile::*;
///  let mut lines:Vec<&str>=Vec::new();
///  lines.push("a");
///  lines.push("b");
///  append_text("data/test2.txt",lines);
/// ```
///
pub fn append_text(filepath:&str,lines:Vec<&str>){
    let mut file = OpenOptions::new()
        .append(true).open(filepath);

    match file {
        Ok(mut file)=>{
            for line in lines{
                file.write(line.as_bytes()).unwrap();
                file.write("\n".as_bytes()).unwrap();
            }
        },
        Err(e)=>{
           eprintln!("Error: {}",e);
        }
    }

}
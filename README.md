# rsfile: Rust File Utility

A Rust library to operate files or web pages easily and quickly

## Functions

The `rsfile` library include simplified operation functions for commonly used I/O, text files, csv files and web crawlers.

## Examples
### File Operations
```rust
use rsfile::*;
fn main(){
    use rsfile;
    // read a csv file and load a list of HashMap models 
    // where you can get value by key. 
    let result=rsfile::read_csv_simple("data/test.csv");
    for model in result{
        println!("RECORD: {:?}",model);
    }
    // save a csv file by using a list of HashMap models 
    // where you can get value by key. 
    let list_model=rsfile::read_csv("data/test.csv");
    let flag=rsfile::write_csv("data/test1.csv",list_model);
    println!("{}",flag);

}
```
### Web Page Operations
```rust

use rsfile::*;

fn main(){
    // 1. get HTML page information (including html string, title, meta, raw text, etc.)
    let page=fetch_html("https://www.rust-lang.org/");
    for k in page.keys(){
        println!("{}\t{:?}",k,page.get(k));
    };
    //2.  get HTML page from a local path and obtain its HTML string
    let page=read_html_file("data/webpage.html");
    let html_opt=page.get("html");
}
```

## Standard IO Operations
```rust
use rsfile::*;
fn main(){
    // read a line
    let line=input_line();
    // read a line after a message
    let line=input_line_with_msg("Please input a line:");
    // read binary
    let content = read_binary("data/test.txt");
    // write a text file for one time
    write_text_once("data/test2.txt","Hello, Rust!");
    // write a list of lines
    let mut lines:Vec<&str>=Vec::new();
    lines.push("a");
    lines.push("b");
    append_text("data/test2.txt",lines);
}
```

Other available functions are:
### text files
- read_text()
- write_text()
- read_lines()
- append_text()
- write_text_once()
- write_lines()

### csv files
- read_csv()
- read_csv_text()
- read_csv_with_error()
- write_csv_with_error()

### HTML pages
- fetch_html()
- read_html_file()
- read_html_string()

### Input/Output
- input()
- input_line()
- input_line_with_msg()
- print()
- read_binary()

More example codes can be found [here](https://github.com/dhchenx/rsfile/blob/main/tests).

## License
MIT
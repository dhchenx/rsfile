# rsfile: Rust File Utility

A Rust library to operate files easily and quickly

## Features

- Write text file
- Read text file
- Write CSV file
- Read CSV file

## Examples

```rust
// read a csv file and load a list of HashMap models where you can get value by key. 
fn test_get_csv_simple(){
    let result=read_csv_simple("data/test.csv");
    for model in result{
        println!("{:?}",model);
    }
}

// save a csv file by using a list of HashMap models where you can get value by key. 
fn test_write_csv(){
    let list_model=read_csv("data/test.csv");
    let flag=write_csv("data/test1.csv",list_model);
    println!("{}",flag);
}
```
Other available functions are:

- read_text()
- write_text()
- read_lines()
- read_csv()
- read_csv_text()
- read_csv_with_error()
- write_csv_with_error()

## License
MIT
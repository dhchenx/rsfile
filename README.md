# rsfile: Rust File Utility

A Rust library to operate files or web pages easily and quickly

## Features

- Write text file
- Read text file
- Write CSV file
- Read CSV file

## Examples
### File Operations
```rust
fn test_rsfile(){
    use rsfile;
    
    // read a csv file and load a list of HashMap models where you can get value by key. 
    let result=rsfile::read_csv_simple("data/test.csv");
    for model in result{
        println!("RECORD: {:?}",model);
    }
    
    // save a csv file by using a list of HashMap models where you can get value by key. 
    let list_model=rsfile::read_csv("data/test.csv");
    let flag=rsfile::write_csv("data/test1.csv",list_model);
    println!("{}",flag);

}
```
### Web Page Operations
```rust
#[cfg(test)]
mod tests {
    use rsfile::*;
    # [test]
    fn test_get_html(){
        // obtain html page
        let page=fetch_html("https://www.rust-lang.org/");
        for k in page.keys(){
            println!("{}\t{:?}",k,page.get(k));
        };
        // get html tag content
        let html_str=match page.get("html"){
            Some(t)=>t.to_string(),
            None=>{String::from("")}
        };
        //write to a html file path
        let hh=&html_str[..];
        let flag=write_text("data/webpage.html",hh);
    }
    #[test]
    fn test_read_html(){
        // read html file from local path
        let page=read_html_file("data/webpage.html");
        let html_opt=page.get("html");
        // get html strings
        let html_str=match html_opt {
            Some(t)=>t.to_string(),
            None=>String::from("").to_string()
        };
        let page1=read_html_string(html_str);
        // let page1=read_html_string(get_ref_string_str(html_opt));
        show_dict(page);
    }
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
- fetch_html()
- read_html_file()
- read_html_string()


More example codes can be found [here](https://github.com/dhchenx/rsfile/blob/main/tests/examples_test.rs).

## License
MIT
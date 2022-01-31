///
///
/// A Rust library to operate files or web pages easily and quickly
///
/// ```
///  use rsfile::*;
/// // Read a file
///  let mytext=read_text("data/poem.txt");
/// // Write a file
///  write_text("data/test.txt","I Like Rust!");
/// // Read a csv file
/// let result=read_csv_simple("data/test.csv");
///  for model in result{
///      println!("{:?}",model);
///   }
/// // Write a csv file
/// let flag=write_csv("data/test1.csv",list_model);
///  println!("{}",flag);
/// // Input from the terminal
/// let line=input_line();
/// let line=input_line_with_msg("Please input a line:");
/// // Fetch a web page
/// let page=fetch_html("https://www.rust-lang.org/");
///  for k in page.keys(){
///      println!("{}\t{:?}",k,page.get(k));
///  };
/// ```

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
mod web;
mod rsio;

pub use web::{read_html_string,read_html_file ,fetch_html,show_dict};

pub use rsio::*;

///
/// Read a text file
///
pub fn read_text(filepath:&str)->String{
    // Create a path to the desired file
    let path = Path::new(filepath);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => {
            eprintln!("couldn't open {}: {}", display, why);
            return String::from("");
        },
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => eprintln!("Couldn't read {}: {}", display, why),
        Ok(_) => print!("Read successfully:\n {}", display),
    };
    s
}

///
/// write text into a file
///
pub fn write_text(filepath:&str,content:&str)->bool{
    let path = Path::new(filepath);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => {
            eprintln!("couldn't create {}: {}", display, why);
            return false
        },
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => {
            eprintln!("couldn't write to {}: {}", display, why);
            return false;
        },
        Ok(_) => {
            println!("successfully wrote to {}", display);
            return true;
        },
    };

}

use std::io::{self, BufRead};

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

///
/// Read a list of lines from a file
///
pub fn read_lines(filepath:&str)->Vec<String>{
    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.

    let mut strings=Vec::new();

    if let Ok(lines) = _read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                // println!("{}", ip);
                strings.push(line);
            }
        }
    }
    strings
}

use csv::Error;

use std::collections::HashMap;

///
/// read a csv file with Result<,>
///
pub fn read_csv_text_with_error(text:&str)->Result<Vec<HashMap<String,String>>, Error>{

    let mut reader = csv::Reader::from_reader(text.as_bytes());

    let mut list_result:Vec<HashMap<String,String>>=Vec::new();

    let headers=reader.headers()?.clone();
    println!("Headers: {:?}",headers);

    let headers_num=headers.len();


    for record in reader.records() {
        let record = record?;
        // println!("{:?}",headers);

        let mut header_names=Vec::new();
        for h in &headers{
            header_names.push(h);
        }
        // let header_names=["year","make","model","description"];

        let mut model:HashMap<String,String> = HashMap::new();
        for i in 0..headers_num{
            let key = String::from(header_names[i]);
            let value=String::from(&record[i].to_string());
            model.insert(key,value);
        }

        // println!("{:?}", record);
        // let s=String::from(&record[0].to_string());
        list_result.push(model);
    };

    Ok(list_result)
}

///
/// Read a csv file's text
///
pub fn read_csv_text(text:&str)->Vec<HashMap<String,String>>{

    let mut reader = csv::Reader::from_reader(text.as_bytes());

    let mut list_result:Vec<HashMap<String,String>>=Vec::new();

    let headers=reader.headers();
    // println!("Fields: {:?}",headers);
    let mut header_names:Vec<String>=Vec::new();
    match headers{
        Ok(hs)=>{
            println!("Header:{:?}",hs);

            let n=hs.len();
            for i in 0..n{
                header_names.push(hs[i].to_string());
            }

        },
        Err(e)=>eprintln!("err: {}",e),
    }

    // let header_names=["year","make","model","description"];

    for item in reader.records() {
        let record = item;
        // println!("{:?}",headers);

        match record{
            Ok(rec)=>{


                let headers_num=header_names.len();


                // let header_names=["year","make","model","description"];

                let mut model:HashMap<String,String> = HashMap::new();
                for i in 0..headers_num{
                    let key = String::from(header_names[i].to_string());
                    let value=String::from(&rec[i].to_string());
                    model.insert(key,value);
                }

                // println!("{:?}", rec);
                // let s=String::from(&record[0].to_string());
                list_result.push(model);
            },
            Err(e)=>{
                eprintln!("{}",e)
            }
        }


    };

    list_result
}

///
/// Read a csv file using Result<,>
///
pub fn read_csv_with_error(filepath:&str)->Result<Vec<HashMap<String,String>>,Error>{
    let all_text=read_text(filepath);
    let all_text_slice:&str=&all_text[..];
    read_csv_text_with_error(all_text_slice)
}

///
/// Read a csv file in a very simple manner and return a Vec<HashMap<String,String>> list
///
pub fn read_csv(filepath:&str)->Vec<HashMap<String,String>>{
    let all_text=read_text(filepath);
    let all_text_slice:&str=&all_text[..];
    read_csv_text(all_text_slice)
}

///
/// Read a simple csv file and return a list of HashMap models
///
pub fn read_csv_simple(filepath:&str)->Vec<HashMap<String,String>>{

    let mut list_result:Vec<HashMap<String,String>>=Vec::new();

    let result=read_csv_with_error(filepath);
    match result{
        Ok(list)=>{
            for model in list{
                println!("RECORD: {:?}",model);
                let mut new_model:HashMap<String,String>=HashMap::new();
                for k in model.keys(){
                    let value=if let Some(v)=model.get(k){
                        // println!("{}:{}",k,v);
                        new_model.insert(k.to_string().clone(),v.to_string().clone());
                    };
                    // println!("{:?}",value);
                }
                list_result.push(new_model);
                // println!();
            }
        },
        Err(e)=>{
            eprintln!("result error: {}",e);
        }
    }
    return list_result;
}

///
/// Write a csv file from a list of HashMap model
///
pub fn write_csv(filepath:&str,list_model:Vec<HashMap<String,String>>)->bool{
    let mut wtr = csv::Writer::from_path(filepath);
    match wtr{
        Ok(mut writer)=>{
            let mut headers:Vec<String>=Vec::new();
            for model in list_model{
                let len=model.keys();
                if headers.len()==0{
                    let ks=model.keys().clone();
                    for k in ks{
                        headers.push(k.to_string());
                    }
                    writer.write_record(&headers);
                }

                let mut values:Vec<String>=Vec::new();

                for k in model.keys(){
                    match model.get(k){
                        Some(t)=>{
                            values.push(t.to_string());
                        }
                        None=>{
                            eprintln!("error in getting the value");
                        }
                    }

                }
                // println!("-{:?}",values);
                writer.write_record(&values);
            }
            writer.flush();
            return true;
        },
        Err(e)=>{
            eprintln!("err:{}",e);
            return false;
        }
    }

}
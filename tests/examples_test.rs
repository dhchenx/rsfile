use rsfile::*;

#[cfg(test)]
mod tests {
    use std::ptr::write;
    use crate::*;
    #[test]
    fn test_read_text(){
        let mytext=read_text("data/poem.txt");
        println!();
        println!("{:?}",mytext);
    }

    #[test]
    fn test_write_text(){
        let r:bool = write_text("data/test.txt","I Like you!\n美国!");
        println!("result = {}",r);
    }

    #[test]
    fn test_read_lines(){
        let lines=read_lines("data/test.txt");
        for line in lines {
            println!("LINE: {}",line);
        }
    }

    #[test]
    fn test_read_csv_text(){
        let text = "year,make,model,description
        1948,Porsche,356,Luxury sports car
        1967,Ford,Mustang fastback 1967,American car";

        let result=read_csv_text_with_error(text);
        match result{
            Ok(list)=>{
                for model in list{
                    println!("RECORD: {:?}",model);
                    for k in model.keys(){
                        let value=if let Some(v)=model.get(k){
                            println!("{}:{}",k,v);
                        };
                        // println!("{:?}",value);
                    }
                    println!();
                }
            },
            Err(e)=>{
                eprintln!("result error: {}",e);
            }
        }

    }

    #[test]
    fn test_read_csv_file(){

        let result=read_csv_with_error("data/test.csv");
        match result{
            Ok(list)=>{
                for model in list{
                    println!("RECORD: {:?}",model);
                    for k in model.keys(){
                        let value=if let Some(v)=model.get(k){
                            println!("{}:{}",k,v);
                        };
                        // println!("{:?}",value);
                    }
                    println!();
                }
            },
            Err(e)=>{
                eprintln!("result error: {}",e);
            }
        }

    }

    #[test]
    fn test_get_csv_simple(){
        let result=read_csv_simple("data/test.csv");
        for model in result{
            println!("{:?}",model);
        }
    }

    #[test]
    fn test_get_csv_simple2(){
        let text = "year,make,model,description
        1948,Porsche,356,Luxury sports car
        1967,Ford,Mustang fastback 1967,American car";
        let result=read_csv_text(text);
        for model in result{
            println!("{:?}",model);
        }
    }

    #[test]
    fn test_write_csv(){
        let list_model=read_csv("data/test.csv");
        let flag=write_csv("data/test1.csv",list_model);
        println!("{}",flag);
    }
}
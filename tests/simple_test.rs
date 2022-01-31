#[cfg(test)]
mod tests {
    use rsfile::*;

    #[test]
    fn test_get_csv_simple(){
        let result=read_csv_simple("data/test.csv");
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


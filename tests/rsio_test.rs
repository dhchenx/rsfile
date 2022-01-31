#[cfg(test)]
mod tests {
    use rsfile::*;

    fn test_all(){
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

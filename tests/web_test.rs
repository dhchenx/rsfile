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


#[cfg(test)]
mod tests {
    use crate::web::{fetch_html,get_string_or_empty, read_html_file, read_html_string, show_dict};
    use crate::write_text;

    # [test]
    fn test_get_html(){
        let page=fetch_html("https://www.rust-lang.org/");
        for k in page.keys(){
            println!("{}\t{:?}",k,page.get(k));
        };
        let html_str=match page.get("html"){
            Some(t)=>t.to_string(),
            None=>{String::from("")}
        };

        let hh=&html_str[..];

        let flag=write_text("data/webpage.html",hh);

        // println!("{:?}",page.get("html").unwrap());
        // println!("html_content:\n{:?}",html_str)
    }

    #[test]
    fn test_read_html(){
        let page=read_html_file("data/webpage.html");
        let html_opt=page.get("html");
        let html_str=match html_opt {
            Some(t)=>t.to_string(),
            None=>String::from("").to_string()
        };
        // let page1=read_html_string(html_str);
        let page1=read_html_string(get_ref_string_str(html_opt));
        show_dict(page);
    }

    fn get_ref_string_str(s:Option<&String>)->String{
        let html_str=match s {
            Some(t)=>t.to_string(),
            None=>String::from("").to_string()
        };
        return html_str;
    }

}

use std::collections::HashMap;
use webpage::{Webpage, WebpageOptions};
use webpage::HTML;
///
/// Convert a Option<String> to &str
///
pub fn get_string_or_empty(s:Option<String>)->String{
    match(s){
        Some(str)=>str,
        None=>String::from(""),
    }
}



/*
pub fn string_to_str(string:String)->&'static str{
    let c=string.clone();
    let hh=&c[..];
    return hh;
}
*/
///
/// print a HashMap model
///
pub fn show_dict(model:HashMap<String,String> ){
    for k in model.keys(){
        println!("{}\t{:?}",k,model.get(k));
    };
}

/*
pub fn opt_string_to_str(opt_string:Option<String>)->&'static str{
    let html_str=match opt_string{
        Some(t)=>t.to_string(),
        None=>{String::from("")}
    };

    let hh=&html_str[..];

    return hh;
}
 */


///
/// Read a html file and return dict
///
pub fn read_html_file(file_path:&str)->HashMap<String,String>{
    let html = HTML::from_file(file_path, None);
    let mut page:HashMap<String,String>=HashMap::new();
    match html{
        Ok(html)=>{

            page.insert("title".to_string(),get_string_or_empty(html.title));
            page.insert("description".to_string(),get_string_or_empty(html.description));
            page.insert("language".to_string(),get_string_or_empty(html.language));
            page.insert("text".to_string(),html.text_content);
            page
        },
        Err(e)=>{
            eprintln!("Error: {}",e);
            page
        }
    }
}

///
/// Read HTML dict from a HTML string
///
pub fn read_html_string(html_string:String)->HashMap<String,String>{
    let html = HTML::from_string(html_string,None);

    let mut page:HashMap<String,String>=HashMap::new();
    match html{
        Ok(html)=>{

            page.insert("title".to_string(),get_string_or_empty(html.title));
            page.insert("description".to_string(),get_string_or_empty(html.description));
            page.insert("language".to_string(),get_string_or_empty(html.language));
            page.insert("text".to_string(),html.text_content);
            page
        },
        Err(e)=>{
            eprintln!("Error: {}",e);
            page
        }
    }
}

///
/// Fetch a web page's HTML codes and return a dict of the HTML page
///
pub fn fetch_html(url:&str)->HashMap<String,String>{
    let info = Webpage::from_url(url, WebpageOptions::default())
        .expect("Could not read from URL");
    // the HTTP transfer info
    let http = info.http;
    let html = info.html;

    let mut page:HashMap<String,String>=HashMap::new();

    let empty=|s:Option<String>|->String{
        match(s){
            Some(ss)=>ss,
            None=>String::from(""),
        }
    };

    page.insert("ip".to_string(),http.ip.to_string());
    page.insert("html".to_string(),http.body);
    page.insert("content_type".to_string(),http.content_type);
    page.insert("title".to_string(),empty(html.title));
    page.insert("description".to_string(),empty(html.description));
    page.insert("language".to_string(),empty(html.language));
    page.insert("text".to_string(),html.text_content);

    page

}
// in chapter-3/main.rs

use chapter_3::{generate_html_template_var, get_content_type, ContentType, TagType};
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

// invokes the parser, initializes the context data & invokes the generator
fn main() {
    // pass context data - creates hashmap to pass values for template variables
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert("name".to_string(), "Bob".to_string());
    context.insert("city".to_string(), "London".to_string());

    /*
    invoke parser and generator - calls get_context_data(), following 3 things can happen:
    01 - contains template variable, invokes generate_html_template_var()
    02 - contains literal string, echoes back the input HTML literal string
    03 - contains 'for' or 'if' tags - feature currently todo!
    */

    // a handle to the standard input of the current process is created & read 1 line at a time
    for line in io::stdin().lock().lines() {
        match get_content_type(&line.unwrap().clone()) {
            ContentType::TemplateVariable(content) => {
                let html = generate_html_template_var(content, context.clone());
                println!("{}", html);
            }
            ContentType::Literal(text) => println!("{}", text),
            ContentType::Tag(TagType::ForTag) => println!("ForTag not yet implemented!"),
            ContentType::Tag(TagType::IfTag) => println!("IfTag not yet implemented!"),
            ContentType::Unrecognised => print!("Unrecognised input!"),
        }
    }
}

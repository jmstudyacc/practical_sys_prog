// in chapter-3/lib.rs

use std::collections::HashMap;

// ExpressionData : Stores the result of the template string tokenization
#[derive(PartialEq, Debug)]
pub struct ExpressionData {
    // head & tag are of Option<String> as a template variable may not contain static literal text before or after it
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>,
}

#[derive(PartialEq, Debug)] // PartialEq allows comparison, Debug allows for Printing
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData), // template string is parsed to extract the head, tail & template var
    Tag(TagType),
    Unrecognised,
}

#[derive(PartialEq, Debug)]
pub enum TagType {
    ForTag, // repetitive loop
    IfTag,  // display control
}

pub fn get_content_type(input_line: &str) -> ContentType {
    // tag expressions are opened & closed by {% ... %}
    let is_tag_expression = check_matching_pairs(&input_line, "{%", "%}");

    // For tags begin with keywords 'for ... in' enclosed in {% ... %}
    // For tags end with 'endfor' enclosed in {% ... %}
    let is_for_tag = (check_symbol_string(&input_line, "for")
        && check_symbol_string(&input_line, "in"))
        || check_symbol_string(&input_line, "endfor");

    // If tags begin with keywords 'if' enclosed in {% ... %}
    // If tags end with 'endif' enclosed in {% ... %}
    let is_if_tag =
        check_symbol_string(&input_line, "if") || check_symbol_string(&input_line, "endif");

    // Template Variables have:
    // 01) An optional head
    // 02) A template variable enclosed bby {{ and }}
    // 03) An optional tail value
    // e.g. the expression "<p> Hello {{name}}, welcome </p> is parsed as:
    // head = 'Hello'   |   variable = '{{name}}    |   tail = ', welcome'

    let is_template_variable = check_matching_pairs(&input_line, "{{", "}}");

    let return_val;

    // case: For Tag
    if is_tag_expression && is_for_tag {
        return_val = ContentType::Tag(TagType::ForTag);
    // case: If Tag
    } else if is_tag_expression && is_if_tag {
        return_val = ContentType::Tag(TagType::IfTag);
    // case: Template Variable
    } else if is_template_variable {
        let content = get_expression_data(&input_line);
        return_val = ContentType::TemplateVariable(content);
    // case: Literal String
    } else if !is_tag_expression && !is_template_variable {
        return_val = ContentType::Literal(input_line.to_string());
    // case: Unrecognised
    } else {
        return_val = ContentType::Unrecognised;
    }

    return_val
}

// check_symbol_string: compares the input & symbol returning a bool if input contains the symbol
pub fn check_symbol_string(input: &str, symbol: &str) -> bool {
    input.contains(symbol)
}

// check_matching_pairs: checks if both the passed symbols are present in the input
pub fn check_matching_pairs(input: &str, symbol1: &str, symbol2: &str) -> bool {
    input.contains(symbol1) && input.contains(symbol2)
}

// helper function that parses template variables
pub fn get_expression_data(input_line: &str) -> ExpressionData {
    let (_h, i) = get_index_for_symbol(input_line, '{');
    let head = input_line[0..i].to_string();
    let (_j, k) = get_index_for_symbol(input_line, '}');
    let variable = input_line[i + 1 + 1..k].to_string();
    let tail = input_line[k + 1 + 1..].to_string();
    ExpressionData {
        head: Some(head),
        variable,
        tail: Some(tail),
    }
}

// get_index_for_symbol: returns bool & usize to verify if the symbol is included & its index
pub fn get_index_for_symbol(input: &str, symbol: char) -> (bool, usize) {
    let mut characters = input.char_indices();
    let mut does_exist = false;
    let mut index = 0;
    while let Some((c, d)) = characters.next() {
        if d == symbol {
            does_exist = true;
            index = c;
            break;
        }
    }
    (does_exist, index)
}

/*
generate_html_template_var : Constructs the output html statement using head, variable, tail of content
Template variables are replaced with the values from the 'context' HashMap variable
*/
pub fn generate_html_template_var(
    content: ExpressionData,
    context: HashMap<String, String>,
) -> String {
    let mut html = String::new();
    if let Some(h) = content.head {
        html.push_str(&h);
    }
    if let Some(val) = context.get(&content.variable) {
        html.push_str(&val);
    }
    if let Some(t) = content.tail {
        html.push_str(&t)
    }
    html
}

// creation of a test module inline
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_literal_test() {
        // 01 - test checks if the static literal string in 's' is tokenized as ContentType::Literal(s)
        let s = "<h1>Hello world</h1>";
        assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s));
    }

    #[test]
    fn check_template_var_test() {
        // 02 - check if the content type is of TemplateVariable type and parsing is correct
        let content = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(", welcome".to_string()),
        };
        assert_eq!(
            ContentType::TemplateVariable(content),
            get_content_type("Hi {{name}}, welcome")
        );
    }

    #[test]
    fn check_for_tag_test() {
        // 03 - Checks if the content is a ForTag - success = ContentType::Tag(TagType::ForTag)
        assert_eq!(
            ContentType::Tag(TagType::ForTag),
            get_content_type("{% for name in names %}, welcome")
        );
    }

    #[test]
    fn check_if_tag_test() {
        // 04 - Checks if the content is an IfTag - success = ContentType::Tag(TagType::IfTag)
        assert_eq!(
            ContentType::Tag(TagType::IfTag),
            get_content_type("{% if name == 'Bob' %}")
        );
    }

    #[test]
    fn check_symbol_string_test() {
        assert_eq!(true, check_symbol_string("{{Hello}}", "{{"));
    }

    #[test]
    fn check_matching_pair_test() {
        // {{ and }} are passed and verified as present/or not in the passed str
        assert_eq!(true, check_matching_pair("{{Hello}}", "{{", "}}"));
    }

    #[test]
    fn check_get_expression_data_test() {
        let expression_data = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(", welcome".to_string()),
        };
        assert_eq!(expression_data, get_expression_date("Hi {{name}}, welcome"));
    }

    #[test]
    fn check_get_index_for_symbol_test() {
        assert_eq!((true, 3), get_index_for_symbol("Hi {name}, welcome", '{'));
    }
}


use std::error::Error;
use std::fmt;

use rustlibxml::xpath::Context;
use rustlibxml::parser::Parser;

#[derive(Debug, PartialEq)]
pub enum XmlError {
    ParseError,
    ContextError,
    XPathError
}

impl Error for XmlError {
    fn description(&self) -> &str { "Error occurred while parsing XML" }
}

impl fmt::Display for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XmlError::ParseError => {
                write!(f, "Error during parsing of XML!")
            }
            XmlError::ContextError => {
                write!(f, "Failed to create a new XPathContext!")
            }
            XmlError::XPathError => {
                write!(f, "Invalid XPath!")
            }
        }
    }
}

// NOTE: Even better was to get rid of map_err and use From<T> but not possible here since 2
// different errors return ()...


pub fn parse(xml: &str, xpath: &str) -> Result<Vec<String>, XmlError> {
    let parser = Parser::default();
    let xml_doc = try!(parser.parse_string(xml).map_err(|_e| XmlError::ParseError));
    let context = try!(Context::new(&xml_doc).map_err(|_e| XmlError::ContextError));
    let result = try!(context.evaluate(xpath).map_err(|_e| XmlError::XPathError));

    Ok(result.get_nodes_as_vec()
             .into_iter()
             .map(|node| node.get_content())
             .collect())
}


#[test]
fn test_parse() {
    let xml = r#"
    <xml>
        <a>
            <b>
                <c>123456789</c>
                <d>Hello</d>
            </b>
            <b>
                <c>abcdefghi</c>
                <d>world!</d>
            </b>
        </a>
    </xml>
    "#;

    let output_c = parse(xml, "//a/b/c/text()").unwrap();
    let output_d = parse(xml, "//a/b/d/text()").unwrap();
    assert_eq!(output_c, vec!["123456789".to_string(), "abcdefghi".to_string()]);
    assert_eq!(output_d, vec!["Hello".to_string(), "world!".to_string()]);
}


mod materialx;

use sxd_xpath::evaluate_xpath;
use sxd_xpath::Factory;
use sxd_document::parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate sxd_document;
extern crate sxd_xpath;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn read_mtlx(filename: &str) {

    let xml = BufReader::new(File::open(filename).unwrap());

    let xml_string :&str = &xml.lines().map(|l| l.unwrap()).collect::<String>();

    let package = parser::parse(xml_string).expect("failed to parse XML");
    let document = package.as_document();
    let factory = Factory::new();
    let xpath_expression = "/materialx/standard_surface/@name";
    let xpath = factory.build(xpath_expression).expect("failed to compile XPath");
    xpath.expect("No XPath expression was compiled");
    let value = evaluate_xpath(&document, xpath_expression).expect("evaluation failed");

    println!("xml_string: {:#?}", xml_string);

    println!("Value: {}", value.string());

    assert_eq!("SR_Aluminum", value.string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let filename = "sample-files/mtlx/Aluminum.mtlx";
        read_mtlx(filename);

        // assert_eq!(result, 4);
    }
}



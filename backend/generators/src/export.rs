/// Auto-generated file. Any changes will be overwritten.
use crate::common::*;
pub fn get(name: &str) -> Option<Box<dyn Generator>> {
    match name {
        "TIAToolbox" => Some(Box::new(crate::tiatoolbox::Module)),
        "Test" => Some(Box::new(crate::test::Module)),
        _ => None,
    }
}
pub fn names() -> Vec<&'static str> {
    vec!["TIAToolbox", "Test"]
}

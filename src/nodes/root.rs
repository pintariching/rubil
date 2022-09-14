use std::collections::HashMap;

use crate::nodes::div::AddDiv;
use crate::traits::element::Element;
use crate::HtmlNode;

#[derive(Debug)]
pub struct Root {
    attributes: HashMap<String, Option<String>>,
    children: Vec<HtmlNode>,
}

impl Root {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }
}

impl Element for Root {
    fn build(&self) -> Self {
        Self {
            attributes: self.attributes.clone(),
            children: self.children.clone(),
        }
    }

    fn attributes(&mut self) -> &mut HashMap<String, Option<String>> {
        &mut self.attributes
    }

    fn children(&mut self) -> &mut Vec<HtmlNode> {
        &mut self.children
    }
}
impl AddDiv for Root {}

use nodes::div::Div;
use traits::{element::Element, ToHtml};

mod nodes;
mod traits;

#[derive(Clone, Debug)]
pub enum HtmlNode {
    Div(Div),
}

impl ToHtml for HtmlNode {
    fn to_html(&self) -> String {
        match self {
            HtmlNode::Div(n) => n.to_html(),
        }
    }
}

impl Element for HtmlNode {
    fn attributes(&mut self) -> &mut std::collections::HashMap<String, Option<String>> {
        match self {
            HtmlNode::Div(n) => n.attributes(),
        }
    }

    fn build(&self) -> Self {
        match self {
            HtmlNode::Div(n) => HtmlNode::Div(n.build()),
        }
    }

    fn children(&mut self) -> &mut Vec<HtmlNode> {
        match self {
            HtmlNode::Div(n) => n.children(),
        }
    }
}

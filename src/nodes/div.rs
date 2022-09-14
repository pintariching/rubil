use crate::traits::element::Element;
use crate::traits::ToHtml;
use crate::HtmlNode;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Div {
    attributes: HashMap<String, Option<String>>,
    children: Vec<HtmlNode>,
}

impl Div {
    fn new() -> Self {
        Div {
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }
}

impl Element for Div {
    fn attributes(&mut self) -> &mut HashMap<String, Option<String>> {
        &mut self.attributes
    }

    fn children(&mut self) -> &mut Vec<HtmlNode> {
        &mut self.children
    }

    fn build(&self) -> Self {
        Self {
            attributes: self.attributes.clone(),
            children: self.children.clone(),
        }
    }
}

impl AddDiv for Div {}

impl ToHtml for Div {
    fn to_html(&self) -> String {
        if self.children.is_empty() {
            if let Some(attrs) = self.clone().get_attr_str() {
                return format!("<div{}></div>", attrs);
            } else {
                return String::from("<div></div>");
            }
        } else {
            let child_content: String = self.children.iter().map(|c| c.to_html()).collect();
            if let Some(attrs) = self.clone().get_attr_str() {
                return format!("<div{}>{}</div>", attrs, child_content);
            } else {
                return format!("<div>{}</div>", child_content);
            }
        }
    }
}

pub trait AddDiv: Element {
    fn div<F: Fn(&mut Self) -> &mut Self>(&mut self, f: F) -> &mut Self {
        f(self)
    }

    fn empty_div(&mut self) -> &mut Self {
        self.children().push(HtmlNode::Div(Div::new()));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::AddDiv;
    use crate::traits::ToHtml;
    use crate::{nodes::div::Div, traits::element::Element};

    #[test]
    fn test_div_add_class() {
        let div = Div::new().class("container p-5").build();

        assert_eq!(
            div.attributes.get("class").unwrap().clone().unwrap(),
            "container p-5".to_string()
        );
    }

    #[test]
    fn test_div_add_class_multiple() {
        let div = Div::new()
            .class("container p-5")
            .class("m-5")
            .class("flex")
            .build();

        assert_eq!(
            div.attributes.get("class").unwrap().clone().unwrap(),
            "container p-5 m-5 flex".to_string()
        );
    }

    #[test]
    fn test_div() {
        let html = Div::new()
            .div(|d| d.class("test").empty_div())
            // .div(|| Div::new().class("container").id("main").build())
            .build();

        println!("{:#?}", html);

        assert!(true);
    }

    #[test]
    fn test_to_html() {
        let html = Div::new().to_html();
        assert_eq!(html, "<div></div>".to_string());

        let html = Div::new().empty_div().to_html();
        assert_eq!(html, "<div><div></div></div>");

        let html = Div::new().empty_div().empty_div().to_html();
        assert_eq!(html, "<div><div></div><div></div></div>");

        let html = Div::new().class("container").to_html();
        assert_eq!(html, r#"<div class="container"></div>"#.to_string());

        let html = Div::new().class("container").id("main").to_html();
        assert_eq!(
            html,
            r#"<div class="container" id="main"></div>"#.to_string()
        );

        let html = Div::new().disabled().to_html();
        assert_eq!(html, r#"<div disabled></div>"#.to_string());
    }
}

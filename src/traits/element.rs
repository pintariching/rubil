use std::collections::HashMap;

use crate::HtmlNode;

pub trait Element {
    fn class(&mut self, class: &str) -> &mut Self {
        if let Some(entry) = self.attributes().get_mut("class") {
            if let Some(classes) = entry {
                classes.push(' ');
                classes.push_str(class);
            } else {
                self.attributes()
                    .insert("class".to_string(), Some(class.to_string()));
            }
        } else {
            self.attributes()
                .insert("class".to_string(), Some(class.to_string()));
        }

        self
    }

    fn id(&mut self, id: &str) -> &mut Self {
        self.attributes()
            .insert("id".to_string(), Some(id.to_string()));

        self
    }

    fn disabled(&mut self) -> &mut Self {
        self.attributes().insert("disabled".to_string(), None);
        self
    }

    fn style(&mut self, style: &str) -> &mut Self {
        if let Some(entry) = self.attributes().get_mut("style") {
            if let Some(classes) = entry {
                classes.push(' ');
                classes.push_str(style);
            } else {
                self.attributes()
                    .insert("style".to_string(), Some(style.to_string()));
            }
        } else {
            self.attributes()
                .insert("style".to_string(), Some(style.to_string()));
        }

        self
    }

    fn build(&self) -> Self;

    fn attributes(&mut self) -> &mut HashMap<String, Option<String>>;

    fn children(&mut self) -> &mut Vec<HtmlNode>;
}

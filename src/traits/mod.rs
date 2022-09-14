use self::element::Element;

pub mod element;
pub trait ToHtml: Element {
    fn to_html(&self) -> String;

    fn get_attr_str(&mut self) -> Option<String> {
        if self.attributes().is_empty() {
            return None;
        } else {
            Some(
                self.attributes()
                    .iter()
                    .map(|(key, value)| {
                        if let Some(v) = value {
                            format!(r#" {}="{}""#, key, v)
                        } else {
                            format!(" {}", key)
                        }
                    })
                    .collect::<String>(),
            )
        }
    }
}

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum HaiSVGError {
    KeyNotFound(String),
}

impl fmt::Display for HaiSVGError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HaiSVGError::KeyNotFound(key) => write!(formatter, "Key '{}' not found in map", key),
        }
    }
}

impl std::error::Error for HaiSVGError {}

struct SVGElement {
    tag: String,
    attributes: HashMap<String, String>,
}

impl SVGElement {
    fn new(tag: &str) -> SVGElement {
        SVGElement {
            tag: tag.to_string(),
            attributes: HashMap::new(),
        }
    }

    fn get_value(&self, key: &str) -> Result<&String, HaiSVGError> {
        self.attributes
            .get(key)
            .ok_or_else(|| HaiSVGError::KeyNotFound(key.to_string()))
    }

    fn attr(&mut self, key: &str, value: &str) -> &mut Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }
}

struct SVG {
    elements: Vec<SVGElement>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_attribute_creation() -> Result<(), HaiSVGError> {
        let mut test_element = SVGElement::new("test_element");
        test_element
            .attr("ten", "10")
            .attr("twenty", "20")
            .attr("thirty", "30");

        let value_a = test_element.get_value("ten")?;
        let value_b = test_element.get_value("twenty")?;
        let value_c = test_element.get_value("thirty")?;

        assert_eq!(value_a, "10");
        assert_eq!(value_b, "20");
        assert_eq!(value_c, "30");
        Ok(())
    }
}

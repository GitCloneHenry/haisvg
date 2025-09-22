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

    fn add_attr<T: ToString>(&mut self, key: &str, value: T) -> &mut Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }

    fn format_keys(&self) -> String {
        let mut items = self.attributes
            .iter()
            .map(|(key, value)| format!("{}=\"{}\"", key, value))
            .collect::<Vec<_>>();
        
        items.sort();
        
        items.join(" ")
    }
}

impl fmt::Display for SVGElement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "<{} {} />", self.tag, self.format_keys())
    }
}

struct SVG {
    attributes: HashMap<String, String>,
    elements: Vec<SVGElement>,
}

impl SVG {
    fn new<T: ToString>(width: Option<T>, height: Option<T>, namespace: Option<T>) -> SVG {
        let width = width
            .map(|w| w.to_string())
            .unwrap_or_else(|| "100".to_string());
        let height = height
            .map(|h| h.to_string())
            .unwrap_or_else(|| "100".to_string());
        let namespace = namespace
            .map(|ns| ns.to_string())
            .unwrap_or_else(|| "http://www.w3.org/2000/svg".to_string());

        let mut svg = SVG {
            attributes: HashMap::new(),
            elements: Vec::new(),
        };

        svg.add_attr("width", width);
        svg.add_attr("height", height);
        svg.add_attr("xmlns", namespace);

        svg
    }

    fn add_attr<T: ToString>(&mut self, key: &str, value: T) -> &mut Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }

    fn add_element(&mut self, element: SVGElement) -> &mut Self {
        self.elements.push(element);
        self
    }

    fn format_keys(&self) -> String {
        let mut items = self.attributes
            .iter()
            .map(|(key, value)| format!("{}=\"{}\"", key, value))
            .collect::<Vec<_>>();
        
        items.sort();
        
        items.join(" ")
    }

    fn format_elements(&self) -> String {
        self.elements
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl fmt::Display for SVG {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "<svg {}>\n{}\n</svg>", self.format_keys(), self.format_elements())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_attribute_creation() -> Result<(), HaiSVGError> {
        let mut test_element = SVGElement::new("test_element");
        test_element
            .add_attr("ten", 10)
            .add_attr("hello", "hi")
            .add_attr("pi", 3.14);

        let value_a = test_element.get_value("ten")?;
        let value_b = test_element.get_value("hello")?;
        let value_c = test_element.get_value("pi")?;

        assert_eq!(value_a, "10");
        assert_eq!(value_b, "hi");
        assert_eq!(value_c, "3.14");
        Ok(())
    }
    
    #[test]
    fn test_element_formatting() {
        let mut test_element = SVGElement::new("test_element");
        test_element.add_attr("test_attr", "foo");

        assert_eq!(
            test_element.to_string(),
            "<test_element test_attr=\"foo\" />"
        );
    }

    #[test]
    fn test_svg_formatting() {
        let mut svg = SVG::new(Some(100), Some(100), None);
        let mut test_element = SVGElement::new("test_element");
        
        test_element.add_attr("test_attr", "foo");      
        svg.add_element(test_element);

        assert_eq!(
            svg.to_string(),
            "<svg height=\"100\" width=\"100\" xmlns=\"http://www.w3.org/2000/svg\">\n<test_element test_attr=\"foo\" />\n</svg>"
        )
    }
}

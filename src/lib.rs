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

pub trait Processable {
    fn process(&self) -> String;
}

impl Processable for String {
    fn process(&self) -> String {
        self.to_string()
    }
}

impl<T: ToString> Processable for Vec<(T, T)> {
    fn process(&self) -> String {
        self.iter()
            .map(|(x, y)| format!("{},{}", x.to_string(), y.to_string()))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

pub struct PathNode {
    tag: String,
    point_data: String,
}

impl PathNode {
    pub fn move_to<T: ToString>(x: T, y: T) -> PathNode {
        PathNode {
            tag: "M".to_string(),
            point_data: format!("{},{}", x.to_string(), y.to_string()),
        }
    }

    pub fn move_by<T: ToString>(dx: T, dy: T) -> PathNode {
        PathNode {
            tag: "m".to_string(),
            point_data: format!("{},{}", dx.to_string(), dy.to_string()),
        }
    }

    pub fn line_to<T: ToString>(dx: T, dy: T) -> PathNode {
        PathNode {
            tag: "L".to_string(),
            point_data: format!("{},{}", dx.to_string(), dy.to_string()),
        }
    }

    pub fn line_by<T: ToString>(dx: T, dy: T) -> PathNode {
        PathNode {
            tag: "l".to_string(),
            point_data: format!("{},{}", dx.to_string(), dy.to_string()),
        }
    }

    pub fn horizontal_to<T: ToString>(x: T) -> PathNode {
        PathNode {
            tag: "H".to_string(),
            point_data: format!("{}", x.to_string()),
        }
    }

    pub fn horizontal_by<T: ToString>(dx: T) -> PathNode {
        PathNode {
            tag: "h".to_string(),
            point_data: format!("{}", dx.to_string()),
        }
    }

    pub fn vertical_to<T: ToString>(y: T) -> PathNode {
        PathNode {
            tag: "V".to_string(),
            point_data: format!("{}", y.to_string()),
        }
    }

    pub fn vertical_by<T: ToString>(dy: T) -> PathNode {
        PathNode {
            tag: "v".to_string(),
            point_data: format!("{}", dy.to_string()),
        }
    }

    pub fn cubic_to<T: ToString>(x1: T, y1: T, x2: T, y2: T, x: T, y: T) -> PathNode {
        PathNode {
            tag: "C".to_string(),
            point_data: format!(
                "{},{} {},{} {},{}",
                x1.to_string(),
                y1.to_string(),
                x2.to_string(),
                y2.to_string(),
                x.to_string(),
                y.to_string(),
            ),
        }
    }

    pub fn cubic_by<T: ToString>(dx1: T, dy1: T, dx2: T, dy2: T, dx: T, dy: T) -> PathNode {
        PathNode {
            tag: "c".to_string(),
            point_data: format!(
                "{},{} {},{} {},{}",
                dx1.to_string(),
                dy1.to_string(),
                dx2.to_string(),
                dy2.to_string(),
                dx.to_string(),
                dy.to_string(),
            ),
        }
    }

    pub fn smooth_cubic_to<T: ToString>(x2: T, y2: T, x: T, y: T) -> PathNode {
        PathNode {
            tag: "S".to_string(),
            point_data: format!(
                "{},{} {},{}",
                x2.to_string(),
                y2.to_string(),
                x.to_string(),
                y.to_string(),
            ),
        }
    }

    pub fn smooth_cubic_by<T: ToString>(dx2: T, dy2: T, dx: T, dy: T) -> PathNode {
        PathNode {
            tag: "s".to_string(),
            point_data: format!(
                "{},{} {},{}",
                dx2.to_string(),
                dy2.to_string(),
                dx.to_string(),
                dy.to_string(),
            ),
        }
    }

    pub fn quadratic_to<T: ToString>(x1: T, y1: T, x: T, y: T) -> PathNode {
        PathNode {
            tag: "Q".to_string(),
            point_data: format!(
                "{},{} {},{}",
                x1.to_string(),
                y1.to_string(),
                x.to_string(),
                y.to_string(),
            ),
        }
    }

    pub fn quadratic_by<T: ToString>(dx1: T, dy1: T, dx: T, dy: T) -> PathNode {
        PathNode {
            tag: "q".to_string(),
            point_data: format!(
                "{},{} {},{}",
                dx1.to_string(),
                dy1.to_string(),
                dx.to_string(),
                dy.to_string(),
            ),
        }
    }

    pub fn smooth_quadratic_to<T: ToString>(x: T, y: T) -> PathNode {
        PathNode {
            tag: "Q".to_string(),
            point_data: format!(
                "{},{}",
                x.to_string(),
                y.to_string(),
            ),
        }
    }

    pub fn quadratic_by<T: ToString>(dx: T, dy: T) -> PathNode {
        PathNode {
            tag: "q".to_string(),
            point_data: format!(
                "{},{}",
                dx.to_string(),
                dy.to_string(),
            ),
        }
    }
}

// impl fmt::Display for PathNode {
//     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(formatter, "{} {}", self.tag, self.point_data)
//     }
// }

pub trait ToPathNode {
    fn to_path_node(&self, tag: &str) -> PathNode;
}

impl ToPathNode for PathNode {
    fn to_path_node(&self, _tag: &str) -> PathNode {
        PathNode {
            tag: self.tag.clone(),
            point_data: self.point_data.clone(),
        }
    }
}

impl<T: ToString> ToPathNode for (T, T) {
    fn to_path_node(&self, tag: &str) -> PathNode {
        PathNode {
            tag: tag.to_string(),
            point_data: format!("{},{}", self.0.to_string(), self.1.to_string()),
        }
    }
}

pub struct SVGElement {
    tag: String,
    attributes: HashMap<String, String>,
}

impl SVGElement {
    pub fn new(tag: &str) -> SVGElement {
        SVGElement {
            tag: tag.to_string(),
            attributes: HashMap::new(),
        }
    }

    pub fn rect<T: ToString>(
        width: T,
        height: T,
        x: T,
        y: T,
        rx: Option<T>,
        ry: Option<T>,
    ) -> Self {
        let rx = rx
            .map(|rx| rx.to_string())
            .unwrap_or_else(|| "0".to_string());
        let ry = ry
            .map(|ry| ry.to_string())
            .unwrap_or_else(|| "0".to_string());

        let mut rect = SVGElement {
            tag: "rect".to_string(),
            attributes: HashMap::new(),
        };

        rect.add_attr("width", width)
            .add_attr("height", height)
            .add_attr("x", x)
            .add_attr("y", y)
            .add_attr("rx", rx)
            .add_attr("ry", ry);

        rect
    }

    pub fn circle<T: ToString>(r: T, cx: T, cy: T) -> Self {
        let mut circle = SVGElement {
            tag: "circle".to_string(),
            attributes: HashMap::new(),
        };

        circle
            .add_attr("r", r)
            .add_attr("cx", cx)
            .add_attr("cy", cy);

        circle
    }

    pub fn ellipse<T: ToString>(rx: T, ry: T, cx: T, cy: T) -> Self {
        let mut ellipse = SVGElement {
            tag: "ellipse".to_string(),
            attributes: HashMap::new(),
        };

        ellipse
            .add_attr("rx", rx)
            .add_attr("ry", ry)
            .add_attr("cx", cx)
            .add_attr("cy", cy);

        ellipse
    }

    pub fn line<T: ToString>(x1: T, y1: T, x2: T, y2: T) -> Self {
        let mut line = SVGElement {
            tag: "line".to_string(),
            attributes: HashMap::new(),
        };

        line.add_attr("x1", x1)
            .add_attr("y1", y1)
            .add_attr("x2", x2)
            .add_attr("y2", y2);

        line
    }

    pub fn polygon<T: Processable>(points: T) -> Self {
        let points = points.process();

        let mut polygon = SVGElement {
            tag: "polygon".to_string(),
            attributes: HashMap::new(),
        };

        polygon.add_attr("points", points);

        polygon
    }

    pub fn polyline<T: Processable>(points: T) -> Self {
        let points = points.process();

        let mut polyline = SVGElement {
            tag: "polyline".to_string(),
            attributes: HashMap::new(),
        };

        polyline.add_attr("points", points);

        polyline
    }

    pub fn get_value(&self, key: &str) -> Result<&String, HaiSVGError> {
        self.attributes
            .get(key)
            .ok_or_else(|| HaiSVGError::KeyNotFound(key.to_string()))
    }

    pub fn add_attr<T: ToString>(&mut self, key: &str, value: T) -> &mut Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }

    pub fn format_keys(&self) -> String {
        let mut items = self
            .attributes
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

pub struct SVG {
    attributes: HashMap<String, String>,
    elements: Vec<SVGElement>,
}

impl SVG {
    pub fn new<T: ToString>(width: T, height: T, namespace: Option<T>) -> SVG {
        let width = width.to_string();
        let height = height.to_string();
        let namespace = namespace
            .map(|ns| ns.to_string())
            .unwrap_or_else(|| "http://www.w3.org/2000/svg".to_string());

        let mut svg = SVG {
            attributes: HashMap::new(),
            elements: Vec::new(),
        };

        svg.add_attr("width", width)
            .add_attr("height", height)
            .add_attr("xmlns", namespace);

        svg
    }

    pub fn add_attr<T: ToString>(&mut self, key: &str, value: T) -> &mut Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }

    pub fn add_element(&mut self, element: SVGElement) -> &mut Self {
        self.elements.push(element);
        self
    }

    pub fn format_keys(&self) -> String {
        let mut items = self
            .attributes
            .iter()
            .map(|(key, value)| format!("{}=\"{}\"", key, value))
            .collect::<Vec<_>>();

        items.sort();

        items.join(" ")
    }

    pub fn format_elements(&self) -> String {
        self.elements
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl fmt::Display for SVG {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "<svg {}>\n{}\n</svg>",
            self.format_keys(),
            self.format_elements()
        )
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
        let mut svg = SVG::new(100, 100, None);
        let mut test_element = SVGElement::new("test_element");

        test_element.add_attr("test_attr", "foo");
        svg.add_element(test_element);

        assert_eq!(
            svg.to_string(),
            "<svg height=\"100\" width=\"100\" xmlns=\"http://www.w3.org/2000/svg\">\n<test_element test_attr=\"foo\" />\n</svg>"
        )
    }
}

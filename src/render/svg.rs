use svg::Document;

use crate::{Drawing, Position};
use crate::render::Renderer;
use crate::canvas::Canvas;
use crate::shape::Shape;

/// Renders the canvas as an SVG
pub struct SvgRenderer {}

impl SvgRenderer {
    pub fn new() -> SvgRenderer {
        SvgRenderer{}
    }
}

impl Renderer for SvgRenderer {
    fn render(&self, canvas: &Canvas) -> Vec<u8> {
        // create a new svg document
        let mut document = Document::new().set("viewBox", (0, 0, canvas.width, canvas.height));
        // first render the background
        if let Some(shape) = &canvas.background {
            let origin = Position::new(0.0, 0.0);
            document = render_shape(shape, &origin, document);
        }
        // render all drawings from the bottom up
        for drawing in canvas.drawings() {
            document = render_drawing(drawing, document)
        }
        // return a byte array
        document.to_string().into_bytes()
    }
}

fn render_drawing(drawing: &Drawing, mut document: Document) -> Document {
    // first, render this drawing's shape
    document = render_shape(&drawing.shape, &drawing.position, document);
    // next, render each drawing from the bottom up
    for drawing in &drawing.display_list.drawings {
        document = render_drawing(drawing, document);
    }
    // finally, return the composed document
    document
}

fn render_shape(shape: &Shape, position: &Position, mut document: Document) -> Document {
    match shape {
        Shape::Rectangle {width, height} => {
            document = document.add(
                svg::node::element::Rectangle::new()
                .set("x", position.x)
                .set("y", position.y)
                .set("width", *width)
                .set("height", *height)
                .set("fill", "black")
                .set("stroke", "gray")
                .set("stroke-width", 1)
            );
        }
    }

    document
}

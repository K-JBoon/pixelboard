use crate::{RGBMatrix, Widget};
use std::collections::HashMap;
use std::time::Duration;
use taffy::error::TaffyError;
use taffy::node::{Node, Taffy};
use taffy::prelude::*;

struct WidgetSet {
    widgets: Vec<Box<dyn Widget>>,
}

pub struct LayoutEngine {
    layout: Taffy,
    root_node: Node,
    size: Size<AvailableSpace>,
    width: usize,
    height: usize,
    widgetsets: HashMap<Node, WidgetSet>,
}

impl LayoutEngine {
    pub fn new(layout: Taffy, root_node: Node, width: usize, height: usize) -> Self {
        let size = Size {
            width: points(width as f32),
            height: points(height as f32),
        };

        Self {
            layout,
            root_node,
            size,
            width,
            height,
            widgetsets: HashMap::new(),
        }
    }

    pub fn add_widget_to_node(&mut self, node: Node, widget: impl Widget + 'static) {
        self.widgetsets.entry(node).or_insert(WidgetSet {
            widgets: vec![],
        });

        let widgetset = self.widgetsets.get_mut(&node).unwrap();
        widgetset.widgets.push(Box::new(widget));
    }

    fn get_widgets_for_node(&mut self, node: Node) -> Option<&mut Vec<Box<dyn Widget>>> {
        if let Some(widget_set) = self.widgetsets.get_mut(&node) {
            return Some(&mut widget_set.widgets);
        } else {
            return None;
        }
    }

    fn compute_layout(&mut self) -> Result<(), TaffyError> {
        self.layout.compute_layout(self.root_node, self.size)
    }

    fn render_node(&mut self, node: Node, mut parent_matrix: RGBMatrix, elapsed_time: Duration) -> RGBMatrix {
        let node_layout = self.layout.layout(node).expect("Failed to get layout");
        let node_width = node_layout.size.width as usize;
        let node_height = node_layout.size.height as usize;

        let mut node_matrix = RGBMatrix::new(node_width, node_height);

        if let Some(widgets) = self.get_widgets_for_node(node) {
            for widget in widgets {
                let widget_matrix = widget.render(node_width, node_height, elapsed_time);
                node_matrix.merge(widget_matrix, 0, 0);
            }
        }

        parent_matrix.merge(node_matrix, 0, 0);

        let children = self
            .layout
            .children(node)
            .expect("Failed to get children for node");

        for child in children {
            let child_layout = self.layout.layout(child).expect("Failed to get layout");

            let child_width = child_layout.size.width as usize;
            let child_height = child_layout.size.height as usize;
            let child_col_offset = child_layout.location.x as usize;
            let child_row_offset = child_layout.location.y as usize;

            let child_base_matrix = RGBMatrix::new(child_width, child_height);
            let child_matrix = self.render_node(child, child_base_matrix, elapsed_time);

            parent_matrix.merge(child_matrix, child_row_offset, child_col_offset);
        }

        parent_matrix
    }

    pub fn render(&mut self, elapsed_time: Duration) -> RGBMatrix {
        self.compute_layout()
            .expect("Failed to compute new layout while rendering");

        let base_matrix = RGBMatrix::new(self.width, self.height);
        self.render_node(self.root_node, base_matrix, elapsed_time)
    }
}

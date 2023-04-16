mod layout_engine;
mod matrix;
mod visualizers;
mod widgets;

use std::time::{Duration, Instant};

use taffy::error::*;
use taffy::prelude::*;

use crate::layout_engine::LayoutEngine;
use crate::matrix::*;
use crate::visualizers::*;
use crate::widgets::*;

const FPS: u32 = 24;

fn default<T: Default>() -> T {
    Default::default()
}

fn main() -> Result<(), TaffyError> {
    let mut taffy = Taffy::new();

    // Setup the grid
    let root_style = Style {
        display: Display::Grid,
        size: Size {
            width: points(96.0),
            height: points(48.0),
        },
        grid_template_columns: vec![points(15.0), fr(1.0), points(15.0)],
        grid_template_rows: vec![points(10.0), fr(1.0), points(10.0)],
        ..default()
    };

    // Define the child nodes
    let header = taffy.new_leaf(Style {
        grid_row: line(1),
        grid_column: span(3),
        ..default()
    })?;
    let left_sidebar = taffy.new_leaf(Style {
        grid_row: line(2),
        grid_column: line(1),
        ..default()
    })?;
    let content_area = taffy.new_leaf(Style {
        grid_row: line(2),
        grid_column: line(2),
        ..default()
    })?;
    let right_sidebar = taffy.new_leaf(Style {
        grid_row: line(2),
        grid_column: line(3),
        ..default()
    })?;
    let footer = taffy.new_leaf(Style {
        grid_row: line(3),
        grid_column: span(3),
        ..default()
    })?;

    // Create the container with the children
    let root = taffy.new_with_children(
        root_style,
        &[header, left_sidebar, content_area, right_sidebar, footer],
    )?;

    // Setup the layout engine
    let mut layout_engine = LayoutEngine::new(taffy, root, 96, 48);

    // Setup the widgets
    let red_block = SolidBlockWidget::new(Some((8, 30, 33)));
    let blue_block = SolidBlockWidget::new(Some((11, 83, 81)));
    let green_block = SolidBlockWidget::new(Some((78, 128, 152)));
    let yellow_block = SolidBlockWidget::new(Some((78, 128, 152)));
    let footer_bg_block = SolidBlockWidget::new(Some((8, 30, 33)));
    let text_block = TextWidget::new("Kanderego? Nani koro!".to_string(), Some((144, 194, 231)));

    // Add widgets to the layout engine
    layout_engine.add_widget_to_node(header, red_block);
    layout_engine.add_widget_to_node(left_sidebar, yellow_block);
    layout_engine.add_widget_to_node(content_area, blue_block);
    layout_engine.add_widget_to_node(right_sidebar, green_block);
    layout_engine.add_widget_to_node(footer, footer_bg_block);
    layout_engine.add_widget_to_node(footer, text_block);

    // Setup visualizer
    let visualizer = TerminalVisualizer::new(96, 48);

    // Start render loop
    let frame_duration = Duration::from_secs(1) / FPS;
    let mut last_frame_time = Instant::now();

    loop {
        let elapsed_time = last_frame_time.elapsed();

        if elapsed_time < frame_duration {
            std::thread::sleep(frame_duration - elapsed_time);
        }

        last_frame_time = Instant::now();

        // Do rendering stuff
        let matrix = layout_engine.render(elapsed_time);
        visualizer.render(matrix);
    }
}

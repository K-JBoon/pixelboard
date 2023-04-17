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

// TODO: make this align with the LED matrix (also defaults to 120)
const FPS: u32 = 120;

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
        grid_template_columns: vec![points(25.0), fr(1.0)],
        grid_template_rows: vec![points(25.0), fr(1.0)],
        ..default()
    };

    // Define the child nodes
    let header = taffy.new_leaf(Style {
        grid_row: line(1),
        grid_column: span(3),
        ..default()
    })?;
    let sidebar = taffy.new_leaf(Style {
        grid_row: line(2),
        grid_column: line(1),
        ..default()
    })?;
    let content_area = taffy.new_leaf(Style {
        grid_row: line(2),
        grid_column: line(2),
        ..default()
    })?;

    // Create the container with the children
    let root = taffy.new_with_children(
        root_style,
        &[header, sidebar, content_area],
    )?;

    // Setup the layout engine
    let mut layout_engine = LayoutEngine::new(taffy, root, 96, 48);

    // Setup the widgets
    let header_block = SolidBlockWidget::new(Some((0, 0, 0)), Some((255, 0, 0)));
    let text_block = TextWidget::new("Pixelboard".to_string(), Some((255, 255, 255)));

    let sidebar_block = SolidBlockWidget::new(Some((0, 0, 0)), Some((0, 255, 0)));
    let clock_block = ClockWidget::new(Some((255, 255, 255)));

    let content_area_block = SolidBlockWidget::new(Some((0, 0, 0)), Some((255, 255, 0)));

    // Add widgets to the layout engine
    layout_engine.add_widget_to_node(header, header_block);
    layout_engine.add_widget_to_node(header, text_block);

    layout_engine.add_widget_to_node(sidebar, sidebar_block);
    layout_engine.add_widget_to_node(sidebar, clock_block);

    layout_engine.add_widget_to_node(content_area, content_area_block);

    // Check passed arguments
    let args: Vec<String> = std::env::args().collect();

    let output_mode: String = match args.len() {
        2 => {
            args[1].parse().expect("Failed to parse argument")
        },
        _ => "terminal".to_owned()
    };

    // Setup visualizer
    if output_mode == "matrix".to_owned() {
        let mut visualizer = LEDMatrixVisualizer::new(96, 48);

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
    } else {
        let mut visualizer = TerminalVisualizer::new(96, 48);

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
}

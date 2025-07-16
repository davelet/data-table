use floem::{
    event::EventListener,
    prelude::*,
    window::{Fullscreen, WindowConfig},
};
use std::sync::Arc;

mod action;

fn main() {
    // Create application with fullscreen window configuration
    let app = floem::Application::new();

    // Configure window to start in fullscreen mode
    let window_config = WindowConfig::default().fullscreen(Fullscreen::Borderless(None));

    app.window(
        move |_window_id| {
            // Create the view
            let view = hundred_table_view();

            // Add a global event listener for window close events
            view.on_event(EventListener::WindowClosed, move |_| {
                // On macOS, we need to explicitly exit
                #[cfg(target_os = "macos")]
                std::process::exit(0);

                // On other platforms, allow normal window close behavior
                #[cfg(not(target_os = "macos"))]
                floem::event::EventPropagation::Continue
            })
        },
        Some(window_config),
    )
    .run();
}

fn hundred_table_view() -> impl IntoView {
    // Create the style closure once and wrap it in an Arc
    let cell_style = Arc::new(cell_style());

    v_stack((
        label(|| "万数表 (1-10,000)").style(|s| {
            s.font_size(20.0)
                .margin_bottom(10.0)
                .color(floem::peniko::Color::rgb8(50, 50, 50))
        }),
        // Add a scrollable container
        scroll(
            v_stack_from_iter((0..100).map(|row| {
                let cell_style = Arc::clone(&cell_style);
                h_stack_from_iter((0..100).map(move |col| {
                    let num = row * 100 + col + 1;
                    let cell_style = Arc::clone(&cell_style);
                    label(move || num.to_string()).style(move |s| cell_style(s))
                }))
            }))
            .style(|s| s.gap(1.0)),
        )
        .style(|s| s.size_full()),
    ))
    .style(|s| {
        s.size_full()
            .justify_center()
            .items_center()
            .padding(20.0)
            .background(floem::peniko::Color::rgb8(250, 250, 250))
    })
}

fn cell_style() -> impl Fn(floem::style::Style) -> floem::style::Style + 'static {
    |s| {
        s.padding(2.0) // Reduced padding
            .border(0.3) // Thinner border
            .border_color(floem::peniko::Color::rgb8(200, 200, 200))
            .border_radius(1.0) // Smaller radius
            .items_center()
            .justify_center()
            .width(30.0) // Smaller width
            .height(25.0) // Smaller height
            .background(floem::peniko::Color::rgb8(245, 245, 245))
            .font_size(10.0) // Smaller font
            .color(floem::peniko::Color::rgb8(60, 60, 60))
            .hover(|s| {
                s.background(floem::peniko::Color::rgb8(220, 240, 255))
                    .border_color(floem::peniko::Color::rgb8(100, 150, 200))
                    .z_index(1) // Ensure hovered cell is above others
            })
    }
}

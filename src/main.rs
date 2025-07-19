use floem::peniko::kurbo::Point;
use floem::{event::EventListener, prelude::*, window::{WindowConfig, Icon}};
use std::fs::File;
use std::io::Read;

// Load icon function
fn load_icon() -> Option<Icon> {
    let icon_path = "assets/icon.png";
    println!("Attempting to load icon: {}", icon_path);
    
    // Try to open the icon file
    match File::open(icon_path) {
        Ok(mut file) => {
            println!("Successfully opened icon file");
            let mut buffer = Vec::new();
            match file.read_to_end(&mut buffer) {
                Ok(size) => {
                    println!("Successfully read icon file, size: {} bytes", size);
                    // Use image library to load the image
                    match image::load_from_memory(&buffer) {
                        Ok(img) => {
                            // Use GenericImageView trait to get image dimensions
                            let width = img.width();
                            let height = img.height();
                            println!("Image loaded successfully, dimensions: {}x{}", width, height);
                            let rgba_img = img.into_rgba8();
                            let rgba_data = rgba_img.into_raw();
                            
                            // Create Icon
                            match Icon::from_rgba(rgba_data, width, height) {
                                Ok(icon) => {
                                    println!("Icon created successfully");
                                    return Some(icon);
                                }
                                Err(e) => {
                                    println!("Failed to create icon: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to load image: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to read icon file: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Unable to open icon file: {}", e);
        }
    }
    
    println!("Icon loading failed, returning None");
    None
}

fn main() {
    // Create application with fullscreen window configuration
    let app = floem::Application::new();
    
    // Load icon
    let icon = load_icon();

    // Configure window with title, size, and position at left-top corner
    let mut window_config = WindowConfig::default()
        .title("Why")
        .size((1280.0, 900.0))
        .position(Point::new(0.0, 0.0))
        .resizable(true);
        
    // Set icon (if loading was successful)
        println!("Icon loading result: {:?}", icon);
    if let Some(icon) = icon {
        window_config = window_config.window_icon(icon);
    }

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
    // Display the large "Why" text
    label(|| "Why").style(|s| {
        s.font_size(120.0) // Use large font
            .font_weight(floem::text::fontdb::Weight(900)) // Bold
            .color(floem::peniko::Color::rgb8(50, 50, 50)) // Dark gray
            .size_full() // Fill entire space
            .justify_center() // Horizontally centered
            .items_center() // Vertically centered
            .padding(20.0) // Padding
            .background(floem::peniko::Color::rgb8(250, 250, 250)) // Light gray background
    })
}

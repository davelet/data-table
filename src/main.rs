use floem::{
    event::EventListener,
    peniko::Color,
    prelude::*,
    window::{Fullscreen, WindowConfig},
};

use crate::action::get_app_config;
use you_my_sql_config::AppConfig;

mod action;

fn main() {
    // Create application with fullscreen window configuration
    let app = floem::Application::new();

    // Configure window to start in fullscreen mode
    let window_config = WindowConfig::default().fullscreen(Fullscreen::Borderless(None));

    app.window(
        move |_window_id| {
            let view = saved_connections_view();
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

fn saved_connections_view() -> impl View {
    let config: RwSignal<Option<AppConfig>> = create_rw_signal(None);

    // Load the config when the view is created
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            if let Ok(cfg) = get_app_config().await.into_result() {
                config.set(Some(cfg));
            }
        });
    });

    let view = container((move || {
        let content = if let Some(cfg) = config.get() {
            if !cfg.saved_connections.is_empty() {
                scroll(
                    dyn_stack(
                        move || cfg.saved_connections.clone(),
                        move |conn| conn.name.clone(),
                        move |conn| {
                            v_stack((
                                label(move || format!("🔗 {}", conn.name))
                                    .style(|s| s.font_size(16.0)),
                                label(move || {
                                    format!("  {}@{}:{}", conn.username, conn.host, conn.port)
                                }),
                            ))
                            .style(|s| {
                                s.padding(15.0)
                                    .border(1.0)
                                    .border_color(Color::rgb8(220, 220, 220))
                                    .border_radius(8.0)
                                    .margin_bottom(10.0)
                                    .width_full()
                            })
                        },
                    )
                    .style(|s| s.width_full().padding(20.0)),
                )
                .style(|s| s.width_full().height_full())
            } else {
                scroll(label(|| "No saved connections found").style(|s| s.padding(20.0)))
                    .style(|s| s.width_full().height_full())
            }
        } else {
            scroll(label(|| "Loading connections...").style(|s| s.padding(20.0)))
                .style(|s| s.width_full().height_full())
        };

        content
    })());

    view.style(|s| {
        s.width_full()
            .height_full()
            .background(floem::peniko::Color::rgb8(250, 250, 250))
    })
}

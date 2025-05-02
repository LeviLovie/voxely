use anyhow::{Context, Result};
use engine::Engine;
use std::sync::Arc;
use tracing::info;
use tracing::{debug, error};
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};
use winit::{event_loop::EventLoop, window::WindowBuilder};

fn run() -> Result<()> {
    let (event_loop, window) = {
        debug!("Creating event loop and window");
        let l_event_loop = EventLoop::new().context("Failed to create event loop")?;
        let l_window = Arc::new(
            WindowBuilder::new()
                .with_title("Game Engine")
                .build(&l_event_loop)
                .context("Failed to create window")?,
        );
        (l_event_loop, l_window)
    };

    let mut engine = Engine::new(window.clone()).context("Failed to create engine")?;

    engine.start().context("Failed to start engine")?;
    info!("Engine started");

    engine
        .run(window, event_loop)
        .context("Failed to run engine")?;
    info!("Engine running");

    engine.destroy().context("Failed to destroy engine")?;
    info!("Engine destroyed");

    Ok(())
}

fn main() {
    {
        let filter = tracing_subscriber::filter::filter_fn(|metadata| {
            let target = metadata.target();
            metadata.level() <= &tracing::Level::WARN
                || target == "assets"
                || target == "engine"
                || target == "mod-api"
                || target == "mod-rhai"
                || target == "render"
                || target == "utils"
        });
        let layer = tracing_subscriber::fmt::layer();
        tracing_subscriber::registry()
            .with(layer.with_filter(filter))
            .init();
    }

    match run() {
        Err(e) => {
            error!("Error: {}", e);
            std::process::exit(1);
        }
        _ => {}
    }
}

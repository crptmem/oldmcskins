use axum::{
    response::IntoResponse, routing::get, Router
};

use axum::extract::Path;
use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use log::info;
use reqwest;

use clap::Parser;

/// Skin obtaining methods
#[derive(clap::ValueEnum, Clone, Debug)]
enum ObtainingMethod {
    /// Get skins and cloaks from Minecraft servers
    License,

    /// Use local skins and cloaks
    Local
}

/// Program arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Skin obtaining method 
    #[arg(short, long)]
    obtaining_method: ObtainingMethod,

    /// HTTP port to listen
    #[arg(short, long, default_value_t = 3000)]
    port: usize,
}

/// Setup Fern logging
fn setup_logging() {
    let colors = ColoredLevelConfig::new()
        .debug(Color::BrightBlack)
        .info(Color::BrightBlue);

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                // This will color the log level only, not the whole line. Just a touch.
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .apply()
        .unwrap();
}

/// Main function
#[tokio::main]
async fn main() {
    setup_logging();
    let args = Args::parse();
    info!("Obtaining method: {:?}", args.obtaining_method);
    info!("Registering routes");
    let app = Router::new()
        .route("/", get(root))
        .route("/MinecraftCloaks/:username", get(cloak))
        .route("/MinecraftSkins/:username", get(skin));

    info!("Listening on 0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Root route handler
async fn root() -> &'static str {
    "Hello, World!"
}

/// Skin route handler
async fn skin(Path(username): Path<String>) -> impl IntoResponse {
    let result = reqwest::get(format!("https://mc-heads.net/download/{}", username)).await.unwrap();
    result.bytes().await.unwrap()
}

/// Cloak route handler
async fn cloak(Path(username): Path<String>) -> impl IntoResponse {
    // NOT IMPLEMENTED
    let result = reqwest::get(format!("https://mc-heads.net/download/{}", username)).await.unwrap();
    result.bytes().await.unwrap()
}

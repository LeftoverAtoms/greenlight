#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

mod asset_browser;
pub use asset_browser::AssetBrowser;

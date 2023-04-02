pub mod api;
pub mod ssh;
pub mod wireguard;

pub mod bastionconfig;
pub mod model;
pub mod persistance;
pub mod startup;
pub mod wgconfigure;
pub mod consts;

pub use model::*;
pub use bastionconfig::BastionConfig;
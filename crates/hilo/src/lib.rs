#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/anton-rs/hilo/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/anton-rs/hilo/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(feature = "derive")]
#[doc(inline)]
pub use hilo_derive as derive;

#[cfg(feature = "registry")]
#[doc(inline)]
pub use hilo_registry as registry;
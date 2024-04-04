#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod client;
pub mod request;
pub mod response;
pub mod utils;

#[cfg(feature = "cli")]
pub mod cli;

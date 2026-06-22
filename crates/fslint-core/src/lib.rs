// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
pub mod cache;
pub mod config;
pub mod plugin_loader;
pub mod scanner;
pub mod safety;

pub use cache::ResultCache;
pub use config::{Config, ScannerConfig};
pub use plugin_loader::PluginLoader;
pub use scanner::{Scanner, ScannedFile};
pub use safety::SafetyChecker;

// Re-export plugin API
pub use fslint_plugin_api::*;

//! A very small bevy wrapper over [`etcetera`](https://docs.rs/etcetera/latest/etcetera/). It allows you to
//! access common directories across MacOS, Windows, and Linux.
//!
//! # Basic usage
//!
//! ```
//! use bevy_etcetera::Directories;
//! use bevy::prelude::*;
//!
//! let mut world = World::new();
//! let directories = Directories::new("com", "doomy", "Cool Bevy Game");
//! world.insert_resource(directories);
//!
//! fn my_system(directories: Res<Directories>) {
//!     // Path dependent on OS
//!     let path = directories.data_dir().join("some_file").with_extension("item.ron");
//! }
//! ```

use std::path::{Path, PathBuf};

use bevy::prelude::*;
use etcetera::{app_strategy::choose_native_strategy, AppStrategy, AppStrategyArgs};

#[cfg(target_os = "windows")]
type Strategy = etcetera::app_strategy::Windows;

#[cfg(target_os = "linux")]
type Strategy = etcetera::app_strategy::Xdg;

#[cfg(any(target_os = "macos", target_os = "ios"))]
type Strategy = etcetera::app_strategy::Apple;

#[derive(Resource)]
pub struct Directories(Strategy);

impl Directories {
    /// Creates a new [`SystemDirectory`] resource with the given options.
    ///
    /// # Parameters
    ///
    /// * `top_level_domain` - The top level domain of the application, e.g. `com`, `org`, or `io.github`.
    /// * `author` - The name of the author of the application.
    /// * `app_name` - The application’s name. This should be capitalised if appropriate.
    ///
    /// # Panics
    ///
    /// Panics if the home directory cannot be located.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_etcetera::Directories;
    /// use bevy::prelude::*;
    ///
    /// let mut world = World::new();
    /// let directories = Directories::new("com", "doomy", "Cool Bevy Game");
    /// world.insert_resource(directories);
    /// ```
    pub fn new(
        top_level_domain: impl Into<String>,
        author: impl Into<String>,
        app_name: impl Into<String>,
    ) -> Self {
        Self(
            choose_native_strategy(AppStrategyArgs {
                top_level_domain: top_level_domain.into(),
                author: author.into(),
                app_name: app_name.into(),
            })
            .unwrap(),
        )
    }

    /// Gets the home directory of the current user
    pub fn home_dir(&self) -> &Path {
        self.0.home_dir()
    }

    /// Gets the configuration directory for your application.
    pub fn config_dir(&self) -> PathBuf {
        self.0.config_dir()
    }

    /// Gets the data directory for your application.
    pub fn data_dir(&self) -> PathBuf {
        self.0.data_dir()
    }

    /// Gets the cache directory for your application.
    pub fn cache_dir(&self) -> PathBuf {
        self.0.cache_dir()
    }

    /// Gets the state directory for your application. State directory may not to exist for all conventions.
    pub fn state_dir(&self) -> Option<PathBuf> {
        self.0.state_dir()
    }

    /// Gets the runtime directory for your application. Runtime directory may not to exist for all conventions.
    pub fn runtime_dir(&self) -> Option<PathBuf> {
        self.0.runtime_dir()
    }

    /// Gets the underlying [`Strategy`] used to obtain directories
    pub fn get(&self) -> &Strategy {
        &self.0
    }
}

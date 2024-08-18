# `bevy_etcetera`

A very small bevy wrapper over [`etcetera`](https://docs.rs/etcetera/latest/etcetera/). It allows you to
access common directories across MacOS, Windows, and Linux.

# Basic usage

```toml
bevy_etcetera = { git = "https://github.com/piedoom/bevy_etcetera" }
# Alternatively, copy the contents of `lib.rs` into your project
```

```rs
use bevy_etcetera::Directories;
use bevy::prelude::*;

let mut world = World::new();
let directories = Directories::new("com", "doomy", "Cool Bevy Game");
world.insert_resource(directories);

fn my_system(directories: Res<Directories>) {
  // Path dependent on OS
  let path = directories.data_dir().join("some_file").with_extension("item.ron");
}
```

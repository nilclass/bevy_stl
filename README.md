# bevy_stl

[![Crate version](https://img.shields.io/crates/v/bevy_stl?style=flat-square)](https://crates.io/crates/bevy_stl/)

![Crate license](https://img.shields.io/crates/l/bevy_stl?style=flat-square)

A [STL](https://en.wikipedia.org/wiki/STL_(file_format)) loader for [bevy](https://bevyengine.org/).

STL is a very simple format, which supports only triangular geometry (positions + normals), without any color / uv / texture information.

It is supported as an output format by most CAD software.

## Alternatives

- by default bevy can load [glTF scenes](https://en.wikipedia.org/wiki/GlTF), which is a much better choice if you are looking for a way to load more complex models / scenes, including materials, animations, etc.
- [bevy_obj](https://crates.io/crates/bevy_obj) can load [Wavefront .obj files](https://en.wikipedia.org/wiki/Wavefront_.obj_file), which can carry more information than STL (such as color, material, UV coordinates)

## Usage

1. Add `bevy_stl` to your `Cargo.toml`
2. Add `bevy_stl::StlPlugin` plugin to the bevy `App`
3. Load STL assets by passing paths with ".stl" extension to `asset_server.load(..)`

### Example

```rust
fn main() {
    App::new()
        .add_plugin(bevy_stl::StlPlugin)
        .add_startup_system(setup)
        // ...
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands
        .spawn(PbrBundle {
            mesh: asset_server.load("disc.stl"),
            material: materials.add(Color::rgb(0.9, 0.4, 0.3).into()),
            ..Default::default()
        })
        // ...
}
```

You can find a more complete example in `examples/spinning_disc.rs` - use `cargo run --example spinning_disc --release` to run it.

## Optional Features

### Wireframe

By default `bevy_stl` produces a triangle mesh (`PrimitiveTopology::TriangleList`).
When the **optional** `wireframe` feature is enabled, an additional line mesh is produced (`PrimitiveTopology::LineList`).

The feature can be enabled via Cargo.toml:
```
[dependencies.bevy_stl]
features = ["wireframe"]
```

When enabled, the mesh can be accessed by appending the wireframe label to the path passed to the asset loader:
```
  // This returns the triangle mesh (the default):
  asset_server.load("disc.stl")

  // This returns the wireframe mesh:
  asset_server.load("disc.stl#wireframe")
```

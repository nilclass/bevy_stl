# bevy_stl

A [STL](https://en.wikipedia.org/wiki/STL_(file_format)) loader for [bevy](https://bevyengine.org/).

STL is a very simple format, which supports only triangular geometry (positions + normals), without any color / uv / texture information.

It is supported as an output format by most CAD software.

## Alternatives

- by default bevy can load [glTF scenes](https://en.wikipedia.org/wiki/GlTF), which is a much better choice if you are looking for a way to load more complex models / scenes, including materials, animations, etc.
- [bevy_obj](bevy_obj) can load [Wavefront .obj files](https://en.wikipedia.org/wiki/Wavefront_.obj_file), which can carry more information than STL (such as color, material, UV coordinates)

## Usage

1. Add `bevy_stl` to your `Cargo.toml`
2. Add `bevy_stl::StlPlugin` plugin to the bevy `App`
3. Load STL assets by passing paths with ".stl" extension to `asset_server.load(..)`

### Example

```rust
fn main() {
    App::build()
        .add_plugin(bevy_stl::StlPlugin)
        .add_startup_system(setup.system())
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

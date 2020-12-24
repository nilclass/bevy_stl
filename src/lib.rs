use anyhow::Result;
use thiserror::Error;
use std::io::Cursor;

use bevy_asset::{AssetLoader, LoadContext, LoadedAsset, AddAsset};
use bevy_render::{
    pipeline::PrimitiveTopology,
    mesh::{Mesh, VertexAttributeValues, Indices},
};
use bevy_utils::BoxedFuture;
use bevy_app::prelude::*;

pub struct StlPlugin;

impl Plugin for StlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_asset_loader::<StlLoader>();
    }
}

#[derive(Default)]
struct StlLoader;

impl AssetLoader for StlLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move { Ok(load_stl(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["stl"];
        EXTENSIONS
    }
}

#[derive(Error, Debug)]
enum StlError {
    #[error("Failed to load STL")]
    Io(#[from] std::io::Error),
}

async fn load_stl<'a, 'b>(bytes: &'a [u8], load_context: &'a mut LoadContext<'b>) -> Result<(), StlError> {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    let mut reader = Cursor::new(bytes);
    let stl = stl_io::read_stl(&mut reader)?;
    let vertex_count = stl.faces.len() * 3;

    let mut positions = Vec::with_capacity(vertex_count);
    let mut normals = Vec::with_capacity(vertex_count);
    let mut indices = Vec::with_capacity(vertex_count);

    for (i, face) in stl.faces.iter().enumerate() {
        for j in 0..3 {
            let vertex = stl.vertices[face.vertices[j]];
            positions.push([vertex[0], vertex[1], vertex[2]]);
            normals.push([face.normal[0], face.normal[1], face.normal[2]]);
            indices.push((i * 3 + j) as u32);
        }
    }

    let uvs = vec![[0.0, 0.0, 0.0]; vertex_count];

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, VertexAttributeValues::Float3(positions));
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::Float3(normals));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float3(uvs));
    mesh.set_indices(Some(Indices::U32(indices)));

    load_context.set_default_asset(LoadedAsset::new(mesh));
    Ok(())
}

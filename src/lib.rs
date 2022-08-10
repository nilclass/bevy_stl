use anyhow::Result;
use std::io::Cursor;
use thiserror::Error;

use bevy::{
    asset::{AddAsset, AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    render::{
        mesh::{Indices, Mesh, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
    utils::BoxedFuture,
};

pub struct StlPlugin;

impl Plugin for StlPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<StlLoader>();
    }
}

#[derive(Default)]
struct StlLoader;

impl AssetLoader for StlLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
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

async fn load_stl<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
) -> Result<(), StlError> {
    let mut reader = Cursor::new(bytes);
    let stl = stl_io::read_stl(&mut reader)?;

    load_context.set_default_asset(LoadedAsset::new(stl_to_triangle_mesh(&stl)));

    #[cfg(feature = "wireframe")]
    load_context.set_labeled_asset("wireframe", LoadedAsset::new(stl_to_wireframe_mesh(&stl)));

    Ok(())
}

fn stl_to_triangle_mesh(stl: &stl_io::IndexedMesh) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

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

    let uvs = vec![[0.0, 0.0]; vertex_count];

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(positions),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(normals),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(uvs));
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}

#[cfg(feature = "wireframe")]
fn stl_to_wireframe_mesh(stl: &stl_io::IndexedMesh) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);

    let positions = stl.vertices.iter().map(|v| [v[0], v[1], v[2]]).collect();
    let mut indices = Vec::with_capacity(stl.faces.len() * 3);
    let normals = vec![[1.0, 0.0, 0.0]; stl.vertices.len()];
    let uvs = vec![[0.0, 0.0, 0.0]; stl.vertices.len()];

    for face in &stl.faces {
        for j in 0..3 {
            indices.push(face.vertices[j] as u32);
            indices.push(face.vertices[(j + 1) % 3] as u32);
        }
    }

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(positions),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(normals),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(uvs));
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}

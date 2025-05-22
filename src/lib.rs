use std::io::Cursor;
use thiserror::Error;

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    render::{
        mesh::{Indices, Mesh, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
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
    type Asset = Mesh;
    type Settings = ();
    type Error = StlError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        #[allow(unused_variables)] load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let mut reader = Cursor::new(bytes);
        let stl = stl_io::read_stl(&mut reader)?;

        #[cfg(feature = "wireframe")]
        load_context.labeled_asset_scope("wireframe".to_string(), |_load_context| {
            stl_to_wireframe_mesh(&stl)
        });

        Ok(stl_to_triangle_mesh(&stl))
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["stl", "STL"];
        EXTENSIONS
    }
}

#[derive(Error, Debug)]
enum StlError {
    #[error("Failed to load STL")]
    Io(#[from] std::io::Error),
}

fn stl_to_triangle_mesh(stl: &stl_io::IndexedMesh) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

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
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

#[cfg(feature = "wireframe")]
fn stl_to_wireframe_mesh(stl: &stl_io::IndexedMesh) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());

    let positions = stl.vertices.iter().map(|v| [v[0], v[1], v[2]]).collect();
    let mut indices = Vec::with_capacity(stl.faces.len() * 3);
    let normals = vec![[1.0, 0.0, 0.0]; stl.vertices.len()];
    let uvs = vec![[0.0, 0.0]; stl.vertices.len()];
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
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

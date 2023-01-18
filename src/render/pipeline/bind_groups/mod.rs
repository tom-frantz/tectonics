use crate::render::prepare::MeshUniform;
use bevy::{
    prelude::*,
    render::render_resource::DynamicUniformBuffer
};

pub mod transform;
pub mod view;

#[derive(Resource, Default)]
pub struct MeshUniformBuffer(pub DynamicUniformBuffer<MeshUniform>);

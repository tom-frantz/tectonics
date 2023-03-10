use self::{
    pipeline::{
        OrthographicHexagonPipeline, ORTHOGRAPHIC_HEXAGON_FRAGMENT_SHADER_HANDLE,
        ORTHOGRAPHIC_HEXAGON_VERTEX_SHADER_HANDLE,
    },
    shader::include_ortho_hex_shader,
};
use crate::render::prepare::MeshUniform;
use bevy::{
    core_pipeline::core_2d::Transparent2d,
    prelude::*,
    render::{
        mesh::GpuMesh,
        render_phase::AddRenderCommand,
        render_resource::{DynamicUniformBuffer, SpecializedRenderPipelines},
        RenderApp, RenderStage,
    },
};
use draw::DrawHexWorld;
use extract::extract;
use prepare::prepare;
use queue::queue;
use crate::render::pipeline::bind_groups::MeshUniformBuffer;

mod draw;
mod pipeline;
mod shader;

mod extract;
mod prepare;
mod queue;

pub mod traits;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        let ortho_shader = include_ortho_hex_shader();

        shaders.set_untracked(
            ORTHOGRAPHIC_HEXAGON_VERTEX_SHADER_HANDLE,
            ortho_shader.vertex,
        );
        shaders.set_untracked(
            ORTHOGRAPHIC_HEXAGON_FRAGMENT_SHADER_HANDLE,
            ortho_shader.fragment,
        );

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_system_to_stage(RenderStage::Extract, extract)
            .add_system_to_stage(RenderStage::Prepare, prepare)
            .add_system_to_stage(RenderStage::Queue, queue)
            .init_resource::<OrthographicHexagonPipeline>()
            .init_resource::<SpecializedRenderPipelines<OrthographicHexagonPipeline>>()
            .init_resource::<MeshUniformBuffer>();

        render_app.add_render_command::<Transparent2d, DrawHexWorld>();
    }
}

// TODO make this actually useful.
pub(self) struct HexWorld(pub u8, GpuMesh);

#[derive(Component, Debug)]
pub struct HexWorldChunk(pub u8, GpuMesh);
impl HexWorld {
    pub fn _new(id: u8, gpu_mesh: GpuMesh) -> Self {
        HexWorld(id, gpu_mesh)
    }
}

use winny::{
    asset::server::AssetServer, gfx::render_pipeline::buffer::AsGpuBuffer, math::vector::Vec4f,
    prelude::*,
};

#[derive(Component, Debug, Clone)]
pub struct NuclearNeutron {
    pub modulation: Modulation,
    pub texture: Handle<Image>,
}

impl Material for NuclearNeutron {
    const BLEND_STATE: wgpu::BlendState = wgpu::BlendState::ALPHA_BLENDING;

    fn resource_state<'s, 'w>(
        &'s self,
        textures: &'w mut RenderAssets<Texture>,
        images: &Assets<Image>,
        context: &Res<RenderContext>,
    ) -> Option<<Self as AsWgpuResources>::State<'w>> {
        if let Some(image) = images.get(&self.texture) {
            Some(
                textures
                    .entry(self.texture.clone())
                    .or_insert_with(|| Texture::prepare_asset(image, &context)),
            )
        } else {
            None
        }
    }

    fn mesh_2d_fragment_shader(&self, server: &AssetServer) -> Handle<FragmentShaderSource> {
        server.load("res/shaders/nuclear.wgsl")
    }

    fn update(&self, context: &RenderContext, binding: &BindGroup) {
        RawNuclearNeutron::write_buffer(context, binding.single_buffer(), &[self.as_raw()]);
    }
}

impl AsWgpuResources for NuclearNeutron {
    type State<'s> = &'s Texture;

    fn as_wgpu_resources<'s>(
        self,
        context: &RenderContext,
        label: &'static str,
        state: Self::State<'s>,
        _buffer_type: Option<BufferType>,
    ) -> Vec<WgpuResource> {
        let texture_resources =
            state.as_wgpu_resources(context, label, SamplerFilterType::Nearest, None);
        let uniform_resources = <&[RawNuclearNeutron] as AsWgpuResources>::as_wgpu_resources(
            &[self.as_raw()],
            context,
            label,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            Some(BufferType::Init),
        );

        vec![texture_resources, uniform_resources]
            .into_iter()
            .flatten()
            .collect()
    }
}

impl AsBindGroup for NuclearNeutron {
    const LABEL: &'static str = "nuclear material";
    const BINDING_TYPES: &'static [wgpu::BindingType] =
        &[DEFAULT_TEXTURE_BINDING, DEFAULT_SAMPLER_BINDING, UNIFORM];
    const VISIBILITY: &'static [wgpu::ShaderStages] = &[wgpu::ShaderStages::FRAGMENT; 3];
}

impl NuclearNeutron {
    pub(crate) fn as_raw(&self) -> RawNuclearNeutron {
        RawNuclearNeutron {
            modulation: self.modulation.clamp(),
        }
    }
}

/// Uniform of [`NuclearNeutron`].
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawNuclearNeutron {
    modulation: Vec4f,
}

unsafe impl AsGpuBuffer for RawNuclearNeutron {}

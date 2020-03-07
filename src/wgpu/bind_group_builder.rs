use crate::wgpu;

/// A type aimed at simplifying the creation of a bind group layout.
#[derive(Debug, Default)]
pub struct LayoutBuilder {
    bindings: Vec<(wgpu::ShaderStage, wgpu::BindingType)>,
}

impl LayoutBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Specify a new binding.
    ///
    /// The `binding` position of each binding will be inferred as the index within the order that
    /// they are added to this builder type. If you require manually specifying the binding
    /// location, you may be better off not using the `BindGroupBuilder` and instead constructing
    /// the `BindGroupLayout` and `BindGroup` manually.
    pub fn binding(mut self, visibility: wgpu::ShaderStage, ty: wgpu::BindingType) -> Self {
        self.bindings.push((visibility, ty));
        self
    }

    /// Add a uniform buffer binding to the layout.
    pub fn uniform_buffer(self, visibility: wgpu::ShaderStage, dynamic: bool) -> Self {
        let ty = wgpu::BindingType::UniformBuffer { dynamic };
        self.binding(visibility, ty)
    }

    /// Add a storage buffer binding to the layout.
    pub fn storage_buffer(
        self,
        visibility: wgpu::ShaderStage,
        dynamic: bool,
        readonly: bool,
    ) -> Self {
        let ty = wgpu::BindingType::StorageBuffer { dynamic, readonly };
        self.binding(visibility, ty)
    }

    /// Add a sampler binding to the layout.
    pub fn sampler(self, visibility: wgpu::ShaderStage) -> Self {
        let ty = wgpu::BindingType::Sampler;
        self.binding(visibility, ty)
    }

    /// Add a sampled texture binding to the layout.
    pub fn sampled_texture(
        self,
        visibility: wgpu::ShaderStage,
        multisampled: bool,
        dimension: wgpu::TextureViewDimension,
    ) -> Self {
        let ty = wgpu::BindingType::SampledTexture {
            multisampled,
            dimension,
        };
        self.binding(visibility, ty)
    }

    /// Short-hand for adding a sampled textured binding for a full view of the given texture to
    /// the layout.
    ///
    /// The `multisampled` and `dimension` parameters are retrieved from the `Texture` itself.
    ///
    /// Note that if you wish to take a `Cube` or `CubeArray` view of the given texture, you will
    /// need to manually specify the `TextureViewDimension` via the `sampled_texture` method
    /// instead.
    pub fn sampled_texture_from(
        self,
        visibility: wgpu::ShaderStage,
        texture: &wgpu::Texture,
    ) -> Self {
        self.sampled_texture(
            visibility,
            texture.sample_count() > 1,
            texture.view_dimension(),
        )
    }

    /// Add a storage texture binding to the layout.
    pub fn storage_texture(
        self,
        visibility: wgpu::ShaderStage,
        dimension: wgpu::TextureViewDimension,
    ) -> Self {
        let ty = wgpu::BindingType::StorageTexture { dimension };
        self.binding(visibility, ty)
    }

    /// Build the bind group layout from the specified parameters.
    pub fn build(self, device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let mut bindings = Vec::with_capacity(self.bindings.len());
        for (i, (visibility, ty)) in self.bindings.into_iter().enumerate() {
            let layout_binding = wgpu::BindGroupLayoutBinding {
                binding: i as u32,
                visibility,
                ty,
            };
            bindings.push(layout_binding);
        }
        let descriptor = wgpu::BindGroupLayoutDescriptor {
            bindings: &bindings,
        };
        device.create_bind_group_layout(&descriptor)
    }
}
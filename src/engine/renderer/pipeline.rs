use ash::vk;
use std::fs::File;
use std::io::Read;

use crate::engine::renderer::device::Device;
use crate::engine::renderer::render_pass::RenderPass;
use crate::engine::renderer::swapchain::Swapchain;

pub struct Pipeline {
    pub pipeline: vk::Pipeline,        // 🔥 texture pipeline
    pub fade_pipeline: vk::Pipeline,   // 🔥 NEW fade pipeline
    pub layout: vk::PipelineLayout,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
}

fn read_shader(path: &str) -> Vec<u32> {
    let mut file = File::open(path).expect("Failed to open shader");
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();
    ash::util::read_spv(&mut std::io::Cursor::new(bytes)).unwrap()
}

impl Pipeline {
    pub fn new(
        device: &Device,
        render_pass: &RenderPass,
        swapchain: &Swapchain,
    ) -> Self {

        // 🔥 Load shaders
        let vert_code = read_shader("shaders/vert.spv");
        let frag_code = read_shader("shaders/frag.spv");
        let fade_frag_code = read_shader("shaders/fade.spv"); // 🔥 NEW

        let vert_module = unsafe {
            device.device.create_shader_module(
                &vk::ShaderModuleCreateInfo {
                    code_size: vert_code.len() * 4,
                    p_code: vert_code.as_ptr(),
                    ..Default::default()
                },
                None,
            ).unwrap()
        };

        let frag_module = unsafe {
            device.device.create_shader_module(
                &vk::ShaderModuleCreateInfo {
                    code_size: frag_code.len() * 4,
                    p_code: frag_code.as_ptr(),
                    ..Default::default()
                },
                None,
            ).unwrap()
        };

        let fade_frag_module = unsafe {
            device.device.create_shader_module(
                &vk::ShaderModuleCreateInfo {
                    code_size: fade_frag_code.len() * 4,
                    p_code: fade_frag_code.as_ptr(),
                    ..Default::default()
                },
                None,
            ).unwrap()
        };

        let entry = std::ffi::CString::new("main").unwrap();

        // 🔥 NORMAL pipeline stages
        let stages = [
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::VERTEX,
                module: vert_module,
                p_name: entry.as_ptr(),
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::FRAGMENT,
                module: frag_module,
                p_name: entry.as_ptr(),
                ..Default::default()
            },
        ];

        // 🔥 FADE pipeline stages
        let fade_stages = [
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::VERTEX,
                module: vert_module, // reuse vertex shader
                p_name: entry.as_ptr(),
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::FRAGMENT,
                module: fade_frag_module,
                p_name: entry.as_ptr(),
                ..Default::default()
            },
        ];

        // 🔥 Vertex input
        let binding = vk::VertexInputBindingDescription {
            binding: 0,
            stride: (4 * std::mem::size_of::<f32>()) as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        };

        let attributes = [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: 0,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: (2 * std::mem::size_of::<f32>()) as u32,
            },
        ];

        let vertex_input = vk::PipelineVertexInputStateCreateInfo {
            vertex_binding_description_count: 1,
            p_vertex_binding_descriptions: &binding,
            vertex_attribute_description_count: attributes.len() as u32,
            p_vertex_attribute_descriptions: attributes.as_ptr(),
            ..Default::default()
        };

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            ..Default::default()
        };

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: swapchain.extent.width as f32,
            height: swapchain.extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain.extent,
        };

        let viewport_state = vk::PipelineViewportStateCreateInfo {
            viewport_count: 1,
            p_viewports: &viewport,
            scissor_count: 1,
            p_scissors: &scissor,
            ..Default::default()
        };

        let rasterizer = vk::PipelineRasterizationStateCreateInfo {
            polygon_mode: vk::PolygonMode::FILL,
            line_width: 1.0,
            cull_mode: vk::CullModeFlags::NONE,
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            ..Default::default()
        };

        let multisample = vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            ..Default::default()
        };

        // 🔥 Alpha blending (important for fade)
        let color_blend_attachment = vk::PipelineColorBlendAttachmentState {
            color_write_mask: vk::ColorComponentFlags::RGBA,
            blend_enable: vk::TRUE,

            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,

            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
        };

        let color_blend = vk::PipelineColorBlendStateCreateInfo {
            attachment_count: 1,
            p_attachments: &color_blend_attachment,
            ..Default::default()
        };

        // 🔥 Descriptor layout (only for texture pipeline)
        let sampler_binding = vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::FRAGMENT,
            ..Default::default()
        };

        let descriptor_layout_info = vk::DescriptorSetLayoutCreateInfo {
            binding_count: 1,
            p_bindings: &sampler_binding,
            ..Default::default()
        };

        let descriptor_set_layout = unsafe {
            device.device
                .create_descriptor_set_layout(&descriptor_layout_info, None)
                .unwrap()
        };

        // 🔥 Push constants (transform + alpha)
        let push_constant_range = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
            offset: 0,
            size: std::mem::size_of::<[f32; 5]>() as u32,
        };

        let layout_info = vk::PipelineLayoutCreateInfo {
            set_layout_count: 1,
            p_set_layouts: &descriptor_set_layout,
            push_constant_range_count: 1,
            p_push_constant_ranges: &push_constant_range,
            ..Default::default()
        };

        let layout = unsafe {
            device.device.create_pipeline_layout(&layout_info, None).unwrap()
        };

        // 🔥 NORMAL pipeline
        let pipeline = unsafe {
            device.device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &[vk::GraphicsPipelineCreateInfo {
                        stage_count: stages.len() as u32,
                        p_stages: stages.as_ptr(),
                        p_vertex_input_state: &vertex_input,
                        p_input_assembly_state: &input_assembly,
                        p_viewport_state: &viewport_state,
                        p_rasterization_state: &rasterizer,
                        p_multisample_state: &multisample,
                        p_color_blend_state: &color_blend,
                        layout,
                        render_pass: render_pass.render_pass,
                        subpass: 0,
                        ..Default::default()
                    }],
                    None,
                )
                .unwrap()[0]
        };

        // 🔥 FADE pipeline
        let fade_pipeline = unsafe {
            device.device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &[vk::GraphicsPipelineCreateInfo {
                        stage_count: fade_stages.len() as u32,
                        p_stages: fade_stages.as_ptr(),
                        p_vertex_input_state: &vertex_input,
                        p_input_assembly_state: &input_assembly,
                        p_viewport_state: &viewport_state,
                        p_rasterization_state: &rasterizer,
                        p_multisample_state: &multisample,
                        p_color_blend_state: &color_blend,
                        layout,
                        render_pass: render_pass.render_pass,
                        subpass: 0,
                        ..Default::default()
                    }],
                    None,
                )
                .unwrap()[0]
        };

        println!("Pipelines created (texture + fade)");

        Self {
            pipeline,
            fade_pipeline,
            layout,
            descriptor_set_layout,
        }
    }
}
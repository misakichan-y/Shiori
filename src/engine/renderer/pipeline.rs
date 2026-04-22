use ash::vk;
use std::fs::File;
use std::io::Read;

use crate::engine::renderer::device::Device;
use crate::engine::renderer::render_pass::RenderPass;
use crate::engine::renderer::swapchain::Swapchain;

pub struct Pipeline {
    pub pipeline: vk::Pipeline,
    pub layout: vk::PipelineLayout,
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
        // 🔹 Load shaders
        let vert_code = read_shader("shaders/vert.spv");
        let frag_code = read_shader("shaders/frag.spv");

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

        let entry = std::ffi::CString::new("main").unwrap();

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

        // 🔹 Vertex input
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();

        // 🔹 Input assembly
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            ..Default::default()
        };

        // 🔹 Viewport
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

        // 🔹 Rasterizer
        let rasterizer = vk::PipelineRasterizationStateCreateInfo {
            polygon_mode: vk::PolygonMode::FILL,
            line_width: 1.0,
            cull_mode: vk::CullModeFlags::BACK,
            front_face: vk::FrontFace::CLOCKWISE,
            ..Default::default()
        };

        // 🔹 Multisampling
        let multisample = vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            ..Default::default()
        };

        // 🔹 Color blending
        let color_blend_attachment = vk::PipelineColorBlendAttachmentState {
            color_write_mask: vk::ColorComponentFlags::RGBA,
            blend_enable: vk::FALSE,
            ..Default::default()
        };

        let color_blend = vk::PipelineColorBlendStateCreateInfo {
            attachment_count: 1,
            p_attachments: &color_blend_attachment,
            ..Default::default()
        };

        // 🔥 PUSH CONSTANT (FIXED)
        let push_constant_range = vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::VERTEX,
            offset: 0,
            size: std::mem::size_of::<[f32; 2]>() as u32, // vec2 offset
        };

        let layout_info = vk::PipelineLayoutCreateInfo {
            push_constant_range_count: 1,
            p_push_constant_ranges: &push_constant_range,
            ..Default::default()
        };

        let layout = unsafe {
            device.device
                .create_pipeline_layout(&layout_info, None)
                .unwrap()
        };

        // 🔹 Pipeline
        let pipeline_info = vk::GraphicsPipelineCreateInfo {
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
        };

        let pipeline = unsafe {
            device.device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &[pipeline_info],
                    None,
                )
                .unwrap()[0]
        };

        println!("Pipeline created!");

        Self { pipeline, layout }
    }
}
use std::time::Instant;
use wgpu::{ util::DeviceExt, PipelineCompilationOptions, TextureFormat };
use bytemuck;
use image::RgbaImage;

// Constants for the texture dimensions
const TEXTURE_WIDTH: u32 = 256;
const TEXTURE_HEIGHT: u32 = 256;

async fn run() {
    /* ---------------- Initialize the WGPU instance and adapter ---------------- */
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();

    /* --------------------------- Shader compilation --------------------------- */
    let start_instant = Instant::now();
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });
    println!("shader compilation {:?}", start_instant.elapsed());

    /* --------------------- Create storage texture --------------------- */
    // Create a texture that can be written to in the compute shader and read back to the CPU
    let storage_texture = device.create_texture(
        &(wgpu::TextureDescriptor {
            label: Some("Storage Texture"),
            size: wgpu::Extent3d {
                width: TEXTURE_WIDTH,
                height: TEXTURE_HEIGHT,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        })
    );

    let storage_texture_view = storage_texture.create_view(&wgpu::TextureViewDescriptor::default());

    // Create a buffer to copy the texture data to for reading back to the CPU
    let output_buffer_size = (TEXTURE_WIDTH * TEXTURE_HEIGHT * 4) as u64; // 4 bytes per pixel (RGBA)
    let output_buffer = device.create_buffer(
        &(wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: output_buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    );

    // Create any additional input buffer if needed for your compute shader
    let input_v = (0..256).map(|i| i as f32).collect::<Vec<_>>();
    let input: &[u8] = bytemuck::cast_slice(&input_v);
    let input_buf = device.create_buffer_init(
        &(wgpu::util::BufferInitDescriptor {
            label: Some("Input Buffer"),
            contents: input,
            usage: wgpu::BufferUsages::STORAGE |
            wgpu::BufferUsages::COPY_DST |
            wgpu::BufferUsages::COPY_SRC,
        })
    );

    /* ----- Create bind group layout, pipeline layout, and compute pipeline ---- */
    let bind_group_layout = device.create_bind_group_layout(
        &(wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                // Input buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true }, // Input is read-only
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Storage texture (write-only from shader)
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        })
    );

    let compute_pipeline_layout = device.create_pipeline_layout(
        &(wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        })
    );

    let pipeline = device.create_compute_pipeline(
        &(wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &cs_module,
            entry_point: Some("main"),
            cache: None,
            compilation_options: PipelineCompilationOptions::default(),
        })
    );

    let bind_group = device.create_bind_group(
        &(wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: input_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&storage_texture_view),
                },
            ],
        })
    );

    /* -------------------- Command encoder and compute pass -------------------- */
    let mut encoder = device.create_command_encoder(
        &(wgpu::CommandEncoderDescriptor {
            label: Some("Compute Command Encoder"),
        })
    );

    // Execute the compute pass that writes to the storage texture
    {
        let mut cpass = encoder.begin_compute_pass(
            &(wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            })
        );
        cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        // Dispatch the compute shader with enough workgroups to cover the entire texture
        cpass.dispatch_workgroups(
            (TEXTURE_WIDTH + 15) / 16, // Divide by 16 (workgroup size) and round up
            (TEXTURE_HEIGHT + 15) / 16,
            1
        );
    }

    // Copy the texture data to a buffer for reading back to the CPU
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &storage_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &output_buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(TEXTURE_WIDTH * 4), // 4 bytes per pixel (RGBA)
                rows_per_image: Some(TEXTURE_HEIGHT),
            },
        },
        wgpu::Extent3d {
            width: TEXTURE_WIDTH,
            height: TEXTURE_HEIGHT,
            depth_or_array_layers: 1,
        }
    );

    // Submit the command buffer
    queue.submit(Some(encoder.finish()));

    /* ------------------ Wait for the GPU to finish processing ----------------- */
    let buf_slice = output_buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buf_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    let start_poll = std::time::Instant::now();
    device.poll(wgpu::PollType::Wait).expect("device.poll failed");
    println!("device.poll took {:?}", start_poll.elapsed());

    // Receive the data and save as an image
    if let Some(Ok(())) = receiver.receive().await {
        let data_raw = &*buf_slice.get_mapped_range();

        // Create an image from the raw data
        let image_data = RgbaImage::from_raw(
            TEXTURE_WIDTH,
            TEXTURE_HEIGHT,
            data_raw.to_vec()
        ).expect("Failed to create image from raw data");

        // Save the image to a file
        image_data.save("output_texture.png").expect("Failed to save image");

        println!("Image saved to output_texture.png");
    }
}

fn main() {
    pollster::block_on(run());
}

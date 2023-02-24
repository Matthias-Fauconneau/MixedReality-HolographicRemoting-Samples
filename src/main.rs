fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*let context = uvc::Context::new()?;
    let device = context.find_device(None, None, None)?;
    println!("Bus {:03} Device {:03} : {:?}", device.bus_number(), device.device_address(), description);
    let device = device.open().expect("Could not open device");
    let format = device.get_preferred_format(|x, y| { if x.fps >= y.fps && x.width * x.height >= y.width * y.height { x } else { y }}).unwrap();
    println!("Best format found: {:?}", format);
    let mut stream = device.get_stream_handle_with_format(format).unwrap();
    panic!();*/

    use windows::Win32::Media::MediaFoundation::*;
    unsafe{use windows::Win32::System::Com::*; CoInitializeEx(None, COINIT(0))}?;
    unsafe{MFStartup(MF_API_VERSION, MFSTARTUP_NOSOCKET)}?;
    let mut attributes = None;
    unsafe{MFCreateAttributes(&mut attributes, 1)}?;
    let attributes = attributes.unwrap();
    unsafe{attributes.SetGUID(&MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID)}?;
    let mut count = 0;
    let mut devices: std::mem::MaybeUninit<*mut Option<IMFActivate>> = std::mem::MaybeUninit::uninit();
    unsafe{MFEnumDeviceSources(&attributes, devices.as_mut_ptr(), &mut count)}?;
    let devices = unsafe{std::slice::from_raw_parts(devices.assume_init(), count as usize)};
    let media_source = unsafe{devices[0].as_ref().unwrap().ActivateObject::<IMFMediaSource>()}?;
    let mut attributes = None;
    unsafe{MFCreateAttributes(&mut attributes, 1)}?;
    let attributes = attributes.unwrap();
    unsafe{attributes.SetUINT32(&MF_READWRITE_DISABLE_CONVERTERS, u32::from(true))}?;
    let source_reader = unsafe{MFCreateSourceReaderFromMediaSource(&media_source, Some(&attributes))}?;
    const MEDIA_FOUNDATION_FIRST_VIDEO_STREAM: u32 = 0xFFFF_FFFC;
    for index in 0.. {
        let media_type = unsafe{source_reader.GetNativeMediaType(MEDIA_FOUNDATION_FIRST_VIDEO_STREAM, index)}?;
        let fourcc = unsafe{media_type.GetGUID(&MF_MT_SUBTYPE)}?;
        let (width, height) = media_type.GetUINT64(MF_MT_FRAME_SIZE);
        println!("{index} {fourcc:?} {width} {height}");
    }
    let mut sample: Option<IMFSample> = None; // drops buffer
    loop {
        let mut flags = 0;
        let mut _timestamp = 0;
        unsafe{source_reader.ReadSample(MEDIA_FOUNDATION_FIRST_VIDEO_STREAM, 0, None, Some(&mut flags), Some(&mut _timestamp), Some(&mut sample))}?;
        if flags & MF_SOURCE_READERF_STREAMTICK.0 as u32 == 0 { break; }
    }
    let buffer = unsafe{sample.unwrap().ConvertToContiguousBuffer()}?;
    let mut len = 0;
    let mut ptr = std::ptr::null_mut();
    unsafe{buffer.Lock(&mut ptr, None, Some(&mut len))}?;
    let buffer = unsafe{std::slice::from_raw_parts(ptr, len as usize)}; 

    /*std::env::set_var("XR_RUNTIME_JSON", "RemotingXR.json");
    use openxr as xr;
    let xr = xr::Entry::linked();
    let available_extensions = xr.enumerate_extensions()?;
    assert!(available_extensions.khr_d3d12_enable);
    assert!(available_extensions.msft_holographic_remoting);
    fn default<T:Default>() -> T { T::default() }
    let mut enabled_extensions = xr::ExtensionSet::default();
    enabled_extensions.khr_d3d12_enable = true;
    enabled_extensions.msft_holographic_remoting = true;
    let xr = xr.create_instance(&xr::ApplicationInfo{application_name: "IR", engine_name: "Rust", ..default()}, &enabled_extensions, &[])?;
    let system = xr.system(xr::FormFactor::HEAD_MOUNTED_DISPLAY)?;
    xr.graphics_requirements::<xr::D3D12>(system)?; // Microsoft Holographic Remoting implementation fails to create session without this call
    let remote_host_name = std::ffi::CString::new(std::env::args().skip(1).next().as_ref().map(|s| s.as_str()).unwrap_or("192.168.0.101"))?;
    xr::cvt(unsafe{(xr.exts().msft_holographic_remoting.unwrap().remoting_connect)(xr.as_raw(), system, &xr::sys::RemotingConnectInfoMSFT {
            ty: xr::StructureType::REMOTING_CONNECT_INFO_MSFT,
            next: std::ptr::null(),
            remote_host_name: remote_host_name.as_ptr(),
            remote_port: 8265,
            secure_connection: false.into(),
    })}).unwrap();
    use pollster::FutureExt as _;
    let adapter = wgpu::Instance::new(wgpu::Backend::Dx12.into()).request_adapter(&default()).block_on().unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor{features: wgpu::Features::TEXTURE_FORMAT_16BIT_NORM|wgpu::Features::MULTIVIEW, ..default()}, None).block_on().unwrap();
    use wgpu_hal::api::Dx12;
    let (session, mut frame_wait, mut frame_stream) = unsafe {
        let (device, queue) = device.as_hal::<Dx12, _, _>(|device| (device.unwrap().raw_device().as_mut_ptr(), device.unwrap().raw_queue().as_mut_ptr()));
        xr.create_session::<xr::D3D12>(system, &xr::d3d12::SessionCreateInfo{device: device.cast(), queue: queue.cast()})
    }?;
    let vert_shader = device.create_shader_module(wgpu::include_wgsl!("fullscreen.wgsl"));
    let frag_shader = device.create_shader_module(wgpu::include_wgsl!("sample.wgsl"));
    let ref layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label: None, entries: &[
        wgpu::BindGroupLayoutEntry{binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture{multisampled: false, view_dimension: wgpu::TextureViewDimension::D2, sample_type: wgpu::TextureSampleType::Float{filterable: true}}, count: None},
        wgpu::BindGroupLayoutEntry{binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None}
    ]});
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label: None, bind_group_layouts: &[layout], push_constant_ranges: &[]});
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let view_type = xr::ViewConfigurationType::PRIMARY_STEREO;
    let views = xr.enumerate_view_configuration_views(system, view_type)?;
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState{module: &vert_shader, entry_point: "fullscreen_vertex_shader", buffers: &[]},
        fragment: Some(wgpu::FragmentState{module: &frag_shader, entry_point: "sample_fragment_shader", targets: &[Some(format.into())]}),
        primitive: default(),
        depth_stencil: None,
        multisample: default(),
        multiview: None//(views.len() > 1).then(|| (views.len() as u32).try_into().ok().unwrap()),
    });
    if views.len() == 2 { assert_eq!(views[0], views[1]); } else { assert!(views.len()==1); }
    let xr::ViewConfigurationView{recommended_image_rect_width: width, recommended_image_rect_height: height, ..} = views[0];
    let mut swapchain = session.create_swapchain(&xr::SwapchainCreateInfo{
        create_flags: xr::SwapchainCreateFlags::EMPTY,
        usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT | xr::SwapchainUsageFlags::SAMPLED,
        format: winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
        sample_count: 1,
        width, height,
        face_count: 1,
        array_size: 1,//views.len() as u32,
        mip_count: 1,
    })?;
    let images = swapchain.enumerate_images()?.into_iter().map(|image| {
        let desc = wgpu::TextureDescriptor {label: None, size: wgpu::Extent3d{width, height, depth_or_array_layers: 1/*views.len() as u32*/}, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format, usage: wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::TEXTURE_BINDING};
        unsafe{device.create_texture_from_hal::<Dx12>(<Dx12 as wgpu_hal::Api>::Device::texture_from_raw(d3d12::Resource::from_raw(image.cast()), desc.format, desc.dimension, desc.size, desc.mip_level_count, desc.sample_count), &desc)}
    }).collect::<Box<_>>();
    let reference_space = session.create_reference_space(xr::ReferenceSpaceType::VIEW, xr::Posef::IDENTITY)?;

    println!("{}", local_ip_address::local_ip()?);
    let ref camera = (std::env::args().skip(2).next().map(|interface| interface.parse().unwrap()).unwrap_or(std::net::Ipv4Addr::UNSPECIFIED),6666);
    let camera = std::net::UdpSocket::bind(camera)?;
    loop {
        let mut event_storage = xr::EventDataBuffer::new();
        while let Some(event) = xr.poll_event(&mut event_storage)? {
            use xr::Event::*; match event {
                SessionStateChanged(e) => {use xr::SessionState as o; match e.state() {
                    o::IDLE|o::SYNCHRONIZED|o::VISIBLE|o::FOCUSED => {},
                    o::READY => { session.begin(view_type)?; println!("Ready"); }
                    o::STOPPING => { session.end()?; println!("Stopping"); return Ok(()); }
                    o::EXITING|o::LOSS_PENDING => { println!("Exiting|LossPending"); return Ok(()); }
                    _ => panic!("{:?}", e.state())
                }}
                InstanceLossPending(_) => { return Ok(()); }
                _ => {dbg!()}
            }
        }
        let frame_state = frame_wait.wait()?;
        frame_stream.begin()?;
        let environment_blend_mode = xr::EnvironmentBlendMode::ADDITIVE;
        if !frame_state.should_render { dbg!(); frame_stream.end(frame_state.predicted_display_time, environment_blend_mode, &[])?; continue; }
        let index = swapchain.acquire_image()? as usize;
        swapchain.wait_image(xr::Duration::INFINITE)?;

        let mut sample: Option<IMFSample> = None; // drops buffer
        loop {
            let mut flags = 0;
            let mut _timestamp = 0;
            unsafe{source_reader.ReadSample(MEDIA_FOUNDATION_FIRST_VIDEO_STREAM, 0, None, Some(&mut flags), Some(&mut _timestamp), Some(&mut sample))}?;
            if flags & MF_SOURCE_READERF_STREAMTICK.0 as u32 == 0 { break; }
        }
        let buffer = unsafe{sample.unwrap().ConvertToContiguousBuffer()}?;
        let mut len = 0;
        let mut ptr = std::ptr::null_mut();
        unsafe{buffer.Lock(&mut ptr, None, Some(&mut len))}?;
        let buffer = unsafe{std::slice::from_raw_parts(ptr, len as usize)}; 
        
        /*let mut image = vec![0u16; 160*120];
        //println!("receive");
        let (len, _sender) = camera.recv_from(bytemuck::cast_slice_mut(&mut image))?;
        //println!("received");
        assert!(len == image.len()*std::mem::size_of::<u16>());
        let min = *image.iter().min().unwrap();
        let max = *image.iter().max().unwrap();
        for value in image.iter_mut() { *value = ((*value - min) as u32 * ((1<<16)-1) / (max - min) as u32) as u16; } // Remap to full range. FIXME: does linear output get gamma compressed or wrongly interpreted as sRGB ?
        let size = wgpu::Extent3d{width: 160, height: 120, depth_or_array_layers: 1};*/
        let gpu_image = device.create_texture(&wgpu::TextureDescriptor{size, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R16Unorm, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, label: None});
        queue.write_texture(wgpu::ImageCopyTexture{texture: &gpu_image, mip_level: 0,origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All},
                    bytemuck::cast_slice(&image), wgpu::ImageDataLayout {offset: 0, bytes_per_row: std::num::NonZeroU32::new(2 * size.width), rows_per_image: std::num::NonZeroU32::new(size.height)},
                    size);
        let image_view = gpu_image.create_view(&default());
        let sampler = device.create_sampler(&default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{label: None, layout, entries: &[
                    wgpu::BindGroupEntry{binding: 0, resource: wgpu::BindingResource::TextureView(&image_view)},
                    wgpu::BindGroupEntry{binding: 1, resource: wgpu::BindingResource::Sampler(&sampler)}]});    
         
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {let ref view = images[index].create_view(&wgpu::TextureViewDescriptor{base_array_layer: 0, array_layer_count: 1/*(views.len() as u32)*/.try_into().ok(), ..default()});
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label: None,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment{view, resolve_target: None, ops: wgpu::Operations{load: wgpu::LoadOp::Clear(wgpu::Color::GREEN), store: true}})], depth_stencil_attachment: None});
        pass.set_pipeline(&render_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.draw(0..3, 0..1);}
        queue.submit(Some(encoder.finish()));
        swapchain.release_image()?;
        let (_, views) = session.locate_views(view_type, frame_state.predicted_display_time, &reference_space)?;
        frame_stream.end(frame_state.predicted_display_time, environment_blend_mode, &[&xr::CompositionLayerProjection::new().space(&reference_space).views(&[0,1].map(|i|
            xr::CompositionLayerProjectionView::new().pose(views[i].pose).fov(views[i].fov).sub_image(xr::SwapchainSubImage::new().swapchain(&swapchain).image_array_index(/*i as u32*/0).image_rect(xr::Rect2Di {offset: xr::Offset2Di{x: 0, y: 0}, extent: xr::Extent2Di{width: width as i32, height: height as i32}}))))])?;
    }*/
    Ok(())
}

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
 };
 
 fn main() {
     env_logger::init(); // Necessary for logging within WGPU
     let event_loop = EventLoop::new(); // Loop provided by winit for handling window events
     let window = WindowBuilder::new().build(&event_loop).unwrap();

     let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
     let surface = unsafe { instance.create_surface(&window) };
     let rescop = &surface.ok().unwrap();
     let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
         power_preference: wgpu::PowerPreference::default(),
         compatible_surface: Some(&surface.ok().unwrap()),
         force_fallback_adapter: false,
     }))
     .unwrap();
 
     let (device, queue) = pollster::block_on(adapter.request_device(
         &wgpu::DeviceDescriptor {
             label: None,
             features: wgpu::Features::empty(),
             limits: wgpu::Limits::default(),
         },
         None, // Trace path
     ))
     .unwrap();
 
     let size = window.inner_size();
     rescop.configure(&device, &wgpu::SurfaceConfiguration {
         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
         format: surface.get_preferred_format(&adapter).unwrap(),
         width: size.width,
         height: size.height,
         present_mode: wgpu::PresentMode::Fifo,
     });
     event_loop.run(move |event, _, control_flow| {});
 }

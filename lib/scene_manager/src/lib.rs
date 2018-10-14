extern crate cgmath;
extern crate core;
extern crate time;
extern crate winit;
extern crate tobj;

use std::path::Path;

use core::traits::Updateable;

pub struct SceneManager {
    renderables: Vec<Box<tobj::Model>>
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            // ToDo: Setup the object and it's structs
            renderables: Vec::new()
        }
    }
    pub fn load_file(&mut self, path: &Path) {
        println!("Loading file: {:?}", path);

        let file_data = tobj::load_obj(&Path::new(path));
        assert!(file_data.is_ok());
        let (models, materials) = file_data.unwrap();
        
        // println!("# of models: {}", models.len());
        // println!("# of materials: {}", materials.len());
        println!("pos: {:?}", models[0]);
        for (i, m) in models.iter().enumerate() {
            self.renderables.push(Box::new(m.clone()));
        }
        &self.renderables.iter().for_each(|r| { println!("pos: {:?}", r.mesh) });
    }
}

impl Updateable for SceneManager {
    fn update(&self) {
        // ToDo: The needful
        println!("Scene Manager");
        // previous_frame.cleanup_finished();

        // if recreate_swapchain {
        //     dimensions = surface
        //         .capabilities(physical)
        //         .expect("failed to get surface capabilities")
        //         .current_extent
        //         .unwrap_or([1024, 768]);

        //     let (new_swapchain, new_images) = match swapchain.recreate_with_dimension(dimensions) {
        //         Ok(r) => r,
        //         Err(vulkano::swapchain::SwapchainCreationError::UnsupportedDimensions) => {
        //             continue;
        //         }
        //         Err(err) => panic!("{:?}", err),
        //     };

        //     swapchain = new_swapchain;
        //     images = new_images;

        //     depth_buffer = vulkano::image::attachment::AttachmentImage::transient(
        //         device.clone(),
        //         dimensions,
        //         vulkano::format::D16Unorm,
        //     ).unwrap();

        //     framebuffers = None;

        //     proj = cgmath::perspective(
        //         cgmath::Rad(std::f32::consts::FRAC_PI_2),
        //         { dimensions[0] as f32 / dimensions[1] as f32 },
        //         0.01,
        //         100.0,
        //     );

        //     dynamic_state.viewports = Some(vec![vulkano::pipeline::viewport::Viewport {
        //         origin: [0.0, 0.0],
        //         dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        //         depth_range: 0.0..1.0,
        //     }]);

        //     recreate_swapchain = false;
        // }

        // if framebuffers.is_none() {
        //     framebuffers = Some(
        //         images
        //             .iter()
        //             .map(|image| {
        //                 Arc::new(
        //                     vulkano::framebuffer::Framebuffer::start(renderpass.clone())
        //                         .add(image.clone())
        //                         .unwrap()
        //                         .add(depth_buffer.clone())
        //                         .unwrap()
        //                         .build()
        //                         .unwrap(),
        //                 )
        //             }).collect::<Vec<_>>(),
        //     );
        // }

        // let uniform_buffer_subbuffer = {
        //     let elapsed = rotation_start.elapsed();
        //     let rotation =
        //         elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;
        //     let rotation = cgmath::Matrix3::from_angle_y(cgmath::Rad(rotation as f32));

        //     let uniform_data = vs::ty::Data {
        //         world: cgmath::Matrix4::from(rotation).into(),
        //         view: (view * scale).into(),
        //         proj: proj.into(),
        //     };

        //     uniform_buffer.next(uniform_data).unwrap()
        // };

        // let set = Arc::new(
        //     vulkano::descriptor::descriptor_set::PersistentDescriptorSet::start(
        //         pipeline.clone(),
        //         0,
        //     ).add_buffer(uniform_buffer_subbuffer)
        //     .unwrap()
        //     .build()
        //     .unwrap(),
        // );

        // let (image_num, acquire_future) =
        //     match vulkano::swapchain::acquire_next_image(swapchain.clone(), None) {
        //         Ok(r) => r,
        //         Err(vulkano::swapchain::AcquireError::OutOfDate) => {
        //             recreate_swapchain = true;
        //             continue;
        //         }
        //         Err(err) => panic!("{:?}", err),
        //     };

        // let command_buffer =
        //     vulkano::command_buffer::AutoCommandBufferBuilder::primary_one_time_submit(
        //         device.clone(),
        //         queue.family(),
        //     ).unwrap()
        //     .begin_render_pass(
        //         framebuffers.as_ref().unwrap()[image_num].clone(),
        //         false,
        //         vec![[0.0, 0.0, 1.0, 1.0].into(), 1f32.into()],
        //     ).unwrap()
        //     .draw_indexed(
        //         pipeline.clone(),
        //         &dynamic_state,
        //         (vertex_buffer.clone(), normals_buffer.clone()),
        //         index_buffer.clone(),
        //         set.clone(),
        //         (),
        //     ).unwrap()
        //     .end_render_pass()
        //     .unwrap()
        //     .build()
        //     .unwrap();

        // let future = previous_frame
        //     .join(acquire_future)
        //     .then_execute(queue.clone(), command_buffer)
        //     .unwrap()
        //     .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
        //     .then_signal_fence_and_flush();

        // match future {
        //     Ok(future) => {
        //         previous_frame = Box::new(future) as Box<_>;
        //     }
        //     Err(vulkano::sync::FlushError::OutOfDate) => {
        //         recreate_swapchain = true;
        //         previous_frame = Box::new(vulkano::sync::now(device.clone())) as Box<_>;
        //     }
        //     Err(e) => {
        //         println!("{:?}", e);
        //         previous_frame = Box::new(vulkano::sync::now(device.clone())) as Box<_>;
        //     }
        // }

        // let mut done = false;
        // events_loop.poll_events(|ev| match ev {
        //     winit::Event::WindowEvent {
        //         event: winit::WindowEvent::CloseRequested,
        //         ..
        //     } => done = true,
        //     _ => (),
        // });
        // if done {
        //     return;
        // }
    }
}

extern crate sdl2;
extern crate imgui;
extern crate imgui_sdl2;
extern crate gl;
extern crate imgui_opengl_renderer;

mod geita_ui;

use std::time::Instant;
use imgui::Condition;
use imgui::FontSource;
use imgui::FontGlyphRanges;
use imgui::Style;
use imgui::StyleVar;
use geita_ui::{GeitaUi, *};
use imgui::Context;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
//mod project_manager;
//use crate::project_manager::GeitaUi;

pub struct WindowSize {
  pub w: u32,
  pub h: u32,
}


fn draw_2d_cube(canvas: &mut Canvas<sdl2::video::Window>) {
  canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
  let rect1 = Rect::new(100, 100, 100, 100);
  let rect2 = Rect::new(200, 100, 100, 100);
  let rect3 = Rect::new(200, 200, 100, 100);
  let rect4 = Rect::new(100, 200, 100, 100);

  canvas.draw_rect(rect1).expect("Failed to draw rectangle");
  canvas.draw_rect(rect2).expect("Failed to draw rectangle");
  canvas.draw_rect(rect3).expect("Failed to draw rectangle");
  canvas.draw_rect(rect4).expect("Failed to draw rectangle");
}

fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video = sdl_context.video().unwrap();
  let mut ws = WindowSize {w: 1000u32, h: 1000u32 };
  {
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);
  }


  let window = video.window("Geite Project Manager", ws.w, ws.h)
    .position_centered()
    .resizable()
    .opengl()
    .allow_highdpi()
    .build()
    .unwrap();

  let _gl_context = window.gl_create_context().expect("Couldn't create GL context");
  gl::load_with(|s| video.gl_get_proc_address(s) as _);

  let mut imgui = imgui::Context::create();
  imgui.set_ini_filename(None);
  
  // FIXME: later.
  init_font(&mut imgui);

  let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);

  let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

  let mut event_pump = sdl_context.event_pump().unwrap();

  let mut last_frame = Instant::now();
  init_style(&mut imgui);
  let mut canvas = window.into_canvas().build().unwrap();

  'running: loop {
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;

    for event in event_pump.poll_iter() {
      imgui_sdl2.handle_event(&mut imgui, &event);
      if imgui_sdl2.ignore_event(&event) { continue; }

      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'running
        },
        _ => {}
      }
    }


    imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

    let now = Instant::now();
    let delta = now - last_frame;
    let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
    last_frame = now;
    imgui.io_mut().delta_time = delta_s;
    let ui = imgui.frame();

    ui.show_demo_window(&mut true);
    ui.show_project_manager_window(&mut true);

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    draw_2d_cube(&mut canvas);
    canvas.present();

    unsafe {
      gl::ClearColor(0.44, 0.44, 0.64, 0.5);
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    imgui_sdl2.prepare_render(&ui, &window);
    renderer.render(&mut imgui);

    window.gl_swap_window();
    println!("{:?}", window.size());
    ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
  }
}

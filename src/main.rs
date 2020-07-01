use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rusty-tanks", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut _canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
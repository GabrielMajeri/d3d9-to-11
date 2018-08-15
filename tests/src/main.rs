//! Direct3D9 test binary.
//!
//! Unfortunately D3D9 does not allow creating devices without a valid window,
//! so we cannot use Rust's test harness and must instead build our own.

#![cfg(windows)]

use std::{
    ptr,
    time::{Duration, Instant},
};

use winit::os::windows::WindowExt;
use winit::{EventsLoop, Window};

mod context;
mod device;

fn main() {
    let mut events_loop = EventsLoop::new();
    let window = Window::new(&events_loop).expect("Failed to create window");

    let ctx = context::create_context();
    context::run_tests(&ctx);

    let hwnd = window.get_hwnd() as *mut _;
    let device = device::create_device(&ctx, hwnd);
    device::run_tests(&device);

    const MAX_TIME: Duration = Duration::from_secs(5);
    let start = Instant::now();

    let mut should_close = false;
    while !should_close {
        // Need to poll for events and handle them, to keep window responsive.
        events_loop.poll_events(|event| {
            use winit::Event;
            if let Event::WindowEvent { event, .. } = event {
                use winit::WindowEvent;
                // Allow the user to close the window by clicking on the "X"
                // or by pressing Escape.
                match event {
                    WindowEvent::CloseRequested => should_close = true,
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(vk) = input.virtual_keycode {
                            use winit::VirtualKeyCode as VK;
                            match vk {
                                VK::Escape => should_close = true,
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
        });

        // Render calls here:
        unsafe {
            device.Present(
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }

        if Instant::now() - start > MAX_TIME {
            should_close = true;
        }
    }

    println!("D3D9 tests ran successfuly");

    std::process::exit(0);
}

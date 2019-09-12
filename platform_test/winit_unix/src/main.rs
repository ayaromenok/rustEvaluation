extern crate winit;

use std::collections::HashMap;


fn main(){
    let mut events_loop = winit::EventsLoop::new();
    let mut windows = HashMap::new();

    for _ in 0..3{
        let window = winit::Window::new(&events_loop).unwrap();
        windows.insert(window.id(), window);
    }

    println!("winit test - few empty windows");

    events_loop.run_forever( |event| {
        match event{
            winit::Event::WindowEvent {
                event: winit::WindowEvent::CloseRequested,
                window_id
            } => {
                    println!("Window {:?} has recieved the signal to close", window_id);

                    windows.remove(&window_id);
                    if windows.is_empty(){
                        println!("windows HashMap is empty, exiting...");
                        return winit::ControlFlow::Break;
                    }
                }
                _  => (),
        }
        winit::ControlFlow::Continue
    })

}

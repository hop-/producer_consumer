use crate::application::Application;

mod application;
mod core;

fn main() {
    println!("Producer Consumer");

    let mut app = Application::new();
    //let (mut app, app_int) = Application::create();

    //{
    //    ctrlc::set_handler(move || {
    //        println!("Stop gracefully");
    //        app_int.send(()).unwrap();
    //    }).expect("Unable to set Ctrl+C handler");
    //}

    app.start();
    app.stop();
}

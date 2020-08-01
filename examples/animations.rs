// For windows, you might want to change the window to a DoubleWindow and remove the sleep from the event loop.

use fltk::{app::*, frame::*, image::*, window::*};

macro_rules! clock {() => ("<?xml version=\"1.0\" encoding=\"utf-8\"?>
<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
<svg version=\"1.1\" id=\"Layer_1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" x=\"0px\" y=\"0px\"
	 width=\"640px\" height=\"480px\" viewBox=\"0 0 640 480\" enable-background=\"new 0 0 640 480\" xml:space=\"preserve\">
<path transform=\"rotate(-45 320 240)\" d=\"M282.658,250.271c0,5.31-1.031,10.156-3.087,14.543c-2.059,4.387-4.984,8.152-8.774,11.293
	c-3.793,3.144-8.477,5.58-14.055,7.312c-5.581,1.731-11.836,2.601-18.767,2.601c-9.968,0-18.605-1.572-25.917-4.713
	s-13.299-6.986-17.955-11.536l13.812-15.111c4.116,3.684,8.584,6.499,13.405,8.449c4.819,1.95,9.993,2.925,15.518,2.925
	c5.525,0,9.856-1.219,12.999-3.656c3.141-2.438,4.712-5.769,4.712-9.993c0-2.056-0.3-3.844-0.894-5.361
	c-0.596-1.517-1.653-2.925-3.168-4.226c-1.518-1.3-3.549-2.519-6.093-3.655c-2.546-1.138-5.768-2.301-9.668-3.494
	c-6.5-2.056-11.943-4.25-16.33-6.58c-4.387-2.328-7.937-4.9-10.643-7.719c-2.709-2.815-4.659-5.931-5.849-9.343
	c-1.193-3.412-1.788-7.23-1.788-11.455c0-5.2,1.082-9.831,3.25-13.893c2.166-4.062,5.144-7.5,8.937-10.318
	c3.791-2.815,8.178-4.956,13.162-6.418c4.981-1.462,10.343-2.193,16.086-2.193c8.449,0,15.842,1.247,22.179,3.737
	c6.337,2.493,11.997,6.121,16.98,10.887l-12.674,14.624c-7.583-6.281-15.655-9.424-24.21-9.424c-4.875,0-8.721,0.95-11.537,2.844
	c-2.818,1.896-4.225,4.578-4.225,8.043c0,1.843,0.297,3.412,0.894,4.712c0.594,1.3,1.65,2.519,3.168,3.656
	c1.516,1.137,3.656,2.249,6.418,3.331c2.763,1.084,6.309,2.33,10.643,3.736c5.306,1.734,10.046,3.631,14.218,5.688
	c4.169,2.06,7.662,4.524,10.48,7.394c2.815,2.871,4.981,6.174,6.5,9.911C281.898,240.603,282.658,245.071,282.658,250.271z
	 M335.953,260.833l20.637-90.181h27.46l-32.011,112.604h-33.634l-32.173-112.604h28.598l20.311,90.181H335.953z M437.832,286.019
	c-16.357,0-28.896-5.01-37.615-15.03c-8.722-10.019-13.081-24.779-13.081-44.278c0-9.531,1.407-17.98,4.225-25.348
	c2.815-7.366,6.688-13.54,11.618-18.524c4.928-4.981,10.668-8.747,17.223-11.293c6.555-2.544,13.568-3.818,21.043-3.818
	c8.23,0,15.436,1.3,21.611,3.899c6.174,2.6,11.537,5.959,16.086,10.075l-14.137,14.624c-3.467-3.032-6.906-5.281-10.318-6.744
	s-7.393-2.193-11.941-2.193c-4.01,0-7.693,0.731-11.051,2.193s-6.256,3.793-8.691,6.987c-2.438,3.196-4.334,7.287-5.688,12.268
	c-1.355,4.984-2.031,10.996-2.031,18.037c0,7.367,0.486,13.567,1.463,18.604c0.975,5.037,2.408,9.1,4.305,12.187
	c1.895,3.087,4.307,5.309,7.23,6.662c2.926,1.355,6.338,2.031,10.238,2.031c5.631,0,10.613-1.244,14.947-3.737v-25.186h-14.785
	l-2.6-18.849h43.547v55.57c-5.85,3.793-12.297,6.718-19.336,8.774C453.051,284.987,445.631,286.019,437.832,286.019z M523.5,151.5
	c0-6.627-5.373-12-12-12h-343c-6.627,0-12,5.373-12,12v150c0,6.627,5.373,12,12,12h343c6.627,0,12-5.373,12-12V151.5z\"/>
</svg>")}

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut wind = DoubleWindow::default()
        .with_label("svg test")
        .with_size(720, 486)
        .center_screen();
    let mut frame = Frame::new(-30, 200, 30, 30, "");
    let mut svg = SvgImage::from_data(&clock!()).unwrap();
    svg.scale(200, 200, true, true);
    frame.set_image(Some(svg));
    wind.set_color(Color::White);
    wind.end();
    wind.show_with_env_args();

    while app.wait().unwrap() {
        let x = frame.x();
        let y = frame.y();
        let w = frame.width();
        let h = frame.height();
        if x > wind.width() + w + 30 {
            app.quit();
        }
        frame.resize(x + 5, y, w, h);
        wind.redraw();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

// // Another way is using app::add_timeout and app::repeat_timout

// fn move_image(mut frm: Frame) {
//     let (x, y) = (frm.x(), frm.y());
//     frm.set_pos(x + 5, y);
//     redraw();
//     repeat_timeout(0.016, Box::new(move || {
//         let frm = frm.clone();
//         move_image(frm);
//     }));
// }

// fn main() {
//     let app = App::default();
//     let mut wind = DoubleWindow::default()
//         .with_label("svg test")
//         .with_size(720, 486)
//         .center_screen();
//     let mut frame = Frame::new(-30, 200, 30, 30, "");
//     let mut svg = SvgImage::from_data(&clock!()).unwrap();
//     svg.scale(200, 200, true, true);
//     frame.set_image(Some(svg));
//     wind.set_color(Color::White);
//     wind.end();
//     wind.show_with_env_args();

//     add_timeout(0.016, Box::new(move || {
//         let frame = frame.clone();
//         move_image(frame);
//     }));
//     app.run().unwrap();
// }

use cursive::event::*;
use cursive::traits::*;
use cursive::views::*;
use cursive::theme::*;
use cursive::Cursive;
use cursive::Printer;
use cursive::Rect;
use unicode_width::UnicodeWidthStr;

fn draw(_: &(), p: &Printer) {
    // We use the view size to calibrate the color
    let x_max = p.size.x as u8;
    let y_max = p.size.y as u8;
    
    for x in 0..x_max {
        for y in 0..y_max {
            // We'll use a different style for each cell
            let style = ColorStyle::new(
                Color::Rgb(255, 255, 255),
                color_gradient(x, x_max),
            );

            p.with_color(style, |printer| {
                printer.print((x, y), " ");
            });
        }
    }
}

// Gradient for the background color
fn color_gradient(x: u8, x_max: u8,) -> Color {
    let r = 25 + (x as f32 / x_max as f32 * 25.0) as u8;
    Color::Rgb(r, 25, 55)
}

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::crossterm();
    
    // We can quit by pressing q
    siv.add_global_callback('q', |s| s.quit());

    // Canvas view
    let state = String::new();
    let canvas = Canvas::new(()).with_draw(draw).fixed_size((120, 30));
    

    // The main dialog will just have a textarea.
    // Its size expand automatically with the content.
    siv.add_fullscreen_layer(
        canvas
    );

    siv.add_layer(
        cursive::views::Dialog::around(
            FixedLayout::new()
                .child(Rect::from_size((0, 0), (1, 1)), TextView::new("/"))
                .child(Rect::from_size((37, 3), (1, 1)), TextView::new(r"\"))
                .child(Rect::from_size((0, 2), (1, 1)), TextView::new(r"\"))
                .child(Rect::from_size((14, 2), (1, 1)), TextView::new("/"))
                .child(
                    Rect::from_size((2, 1), (11, 1)),
                    Button::new("Click me!", |s| s.quit()),
                ),
        )
        .button("Quit", |s| s.quit()),
    );

    // Starts the event loop.
    siv.run();
}
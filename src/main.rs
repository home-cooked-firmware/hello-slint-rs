use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let screen_width = std::env::var("HELLO_SLINT_WIDTH").unwrap_or("640".to_string());
    let screen_height = std::env::var("HELLO_SLINT_HEIGHT").unwrap_or("480".to_string());

    let ui = MainWindow::new()?;

    ui.set_screen_width(screen_width.parse::<i32>().unwrap());
    ui.set_screen_height(screen_height.parse::<i32>().unwrap());

    ui.run()?;

    Ok(())
}

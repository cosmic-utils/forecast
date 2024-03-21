mod app;
mod key_bind;
mod menu;

use cosmic::app::Settings;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::default()
        .antialiasing(true)
        .client_decorations(true)
        .debug(false);
        
    cosmic::app::run::<App>(settings, ())?;
    
    Ok(())
}

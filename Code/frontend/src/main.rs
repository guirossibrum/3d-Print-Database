mod api;
mod models;
mod state;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = state::App::new()?;
    
    // Load initial data
    app.load_all_data().await?;
    
    // Run the application
    while app.running {
        // TODO: Handle events and render UI
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    Ok(())
}
use std::time::Instant;

use colored::Colorize;
use gemeindeschluessel_skript::request::get_meldungsuebersicht;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let timestamp = Instant::now();
    println!("{}", "startet...".green());
    get_meldungsuebersicht().await?;
    println!(
        "Skript hat {}s gebraucht.",
        timestamp.elapsed().as_secs_f32().to_string().green()
    );
    Ok(())
}

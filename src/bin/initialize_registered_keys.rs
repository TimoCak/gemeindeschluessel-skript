use std::{fs, time::Instant};

use colored::Colorize;
use gemeindeschluessel_skript::request::get_meldungsuebersicht_initialize;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let timestamp = Instant::now();
    println!("{}", "startet...".green());
    fs::create_dir_all("./registeredKeys").expect("should have created folder!");
    //write extra function maybe
    let paths = fs::read_dir("./gemeindeschluessel").expect("should have red all subfiles!");
    for p in paths {
        let file_name = p.unwrap().file_name();

        get_meldungsuebersicht_initialize(file_name.to_str().unwrap()).await?;
    }

    println!(
        "Skript hat {}s gebraucht.",
        timestamp.elapsed().as_secs_f32().to_string().green()
    );
    Ok(())
}

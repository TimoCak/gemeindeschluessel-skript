use colored::Colorize;
use gemeindeschluessel_skript::request::get_meldungsuebersicht;
use gemeindeschluessel_skript::sound::play_sound;
use reqwest::Error;
use std::fs;
use std::thread;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let timestamp = Instant::now();
    println!("{}", "startet...".green());
    let mut counter = 1;
    loop {
        let paths = fs::read_dir("./registeredKeys").expect("should have red all subfiles!");
        for p in paths {
            let file_name = p.unwrap().file_name();

            let body = get_meldungsuebersicht(file_name.to_str().unwrap()).await?;

            if body.ne("[]") {
                if file_name.eq("Stuttgart.json") {
                    play_sound("song_1.mp3");
                }
            }
        }
        println!(
            "{}.Durchlauf: {}s.",
            counter,
            timestamp.elapsed().as_secs_f32().to_string().green()
        );
        let one_min = Duration::from_secs(60);
        thread::sleep(one_min);
        counter = counter + 1;
    }
    Ok(())
}

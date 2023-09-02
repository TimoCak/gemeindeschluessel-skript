/* Anfragen an Bundesamt für Bevölkerungsschutz: NINA API

    Von Time Caktu
*/

//dieses File kann auch durch eine JavaScript-Datei ersetzt werden!

use colored::Colorize;
use reqwest::Error;
use std::fs;
use std::io;

const URL: &str = "https://warnung.bund.de/api31";

pub fn get_gemeindeschlüssel(stadt: &str) -> Result<String, io::Error> {
    let file = fs::read_to_string(format!("./gemeindeschluessel/{}", stadt))
        .expect("should have been able to read the file!");

    let stadt_warnung: Vec<Option<String>> =
        serde_json::from_str(&file).expect("expected to deserialize data!");

    let gemeindeschlüssel = stadt_warnung
        .get(0)
        .unwrap()
        .to_owned()
        .expect("Stadt besitzt kein Gemeindeschlüssel!");

    println!(
        "Gemeindeschlüssel von {}: {}",
        stadt.purple(),
        gemeindeschlüssel.yellow()
    );

    Ok(gemeindeschlüssel)
}

//initialize the registeredKeys from API
pub async fn get_meldungsuebersicht_initialize(stadt: &str) -> Result<(), Error> {
    //eine Übersicht der Meldungen in deiner Region
    let gemeindeschlüssel =
        get_gemeindeschlüssel(stadt).expect("Gemeindeschlüssel sollte erhalten sein!");

    let request_url = format!("{}/dashboard/{}.json", URL, gemeindeschlüssel);
    println!("{}", request_url.blue());
    let response = reqwest::get(request_url).await?;
    let status = response.status();
    let body = response.text().await?;

    if status == 200 {
        let gemeinde_file = fs::read_to_string(format!("./gemeindeschluessel/{}", stadt))
            .expect("red file failed!");
        fs::write(
            format!("./registeredKeys/{}", stadt),
            serde_json::to_string_pretty(&gemeinde_file)
                .expect("deserialize should be successful!"),
        )
        .expect("should write file in ./registeredKeys");
    }
    println!("{}", status);
    println!("{}", body);
    Ok(())
}

//change parameter from ars to City Name !
pub async fn get_meldungsuebersicht(stadt: &str) -> Result<String, Error> {
    let gemeindeschlüssel =
        get_gemeindeschlüssel(stadt).expect("Gemeindeschlüssel sollte erhalten sein!");

    let request_url = format!("{}/dashboard/{}.json", URL, gemeindeschlüssel);
    println!("{}", request_url.blue());
    let response = reqwest::get(request_url).await?;
    let status = response.status();
    let body = response.text().await?;
    println!("{}", status);
    println!("{}", body);
    Ok(body)
}
pub fn get_warnungen_details(id: String) {
    //detailinformation  zu einer Warnung!
    todo!()
}

pub fn get_warnungen_geo(id: String) {
    //geojson informationen zu einer Warnung!
    todo!()
}

//usw.

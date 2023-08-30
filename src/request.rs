/* Anfragen an Bundesamt für Bevölkerungsschutz: NINA API

    Von Time Caktu
*/

//dieses File kann auch durch eine JavaScript-Datei ersetzt werden!

use std::fs;
use std::io;
use reqwest::Error;

const URL: &str = "https://warnung.bund.de/api31";

pub fn get_gemeindeschlüssel(stadt: &str) -> Result<String, io::Error> {
    let file = fs::read_to_string(format!("./gemeindeschluessel/{}.json", stadt))
        .expect("should have been able to read the file!");

    let stadt_warnung: Vec<Option<String>> =
        serde_json::from_str(&file).expect("expected to deserialize data!");

    let gemeindeschlüssel = stadt_warnung
        .get(0)
        .unwrap()
        .to_owned()
        .expect("Stadt besitzt kein Gemeindeschlüssel!");

    println!("Gemeindeschlüssel von {}: {}", stadt, gemeindeschlüssel);

    Ok(gemeindeschlüssel)
}

//change parameter from ars to City Name !
pub async fn get_meldungsuebersicht() -> Result<(), Error> {
    //eine Übersicht der Meldungen in deiner Region
    let gemeindeschlüssel = get_gemeindeschlüssel("Chemnitz");
    let body = reqwest::get(format!("{}/dashboard/{:?}.json", URL, gemeindeschlüssel))
        .await?
        .text()
        .await?;

    println!("{:?}", body);
    Ok(())
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

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BundResponse {
    pub metadaten: Meta,
    pub spalten: Vec<Spalte>,
    pub daten: Vec<Vec<Option<String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Name {
    pub value: Option<String>,
    pub lang: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Code {
    pub code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Spalte {
    pub spaltennameLang: String,
    pub spaltennameTechnisch: String,
    pub datentyp: String,
    pub codeSpalte: bool,
    pub verwendung: Code,
    pub empfohleneCodeSpalte: bool,
    pub sprache: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub kennung: String,
    pub kennungInhalt: String,
    pub version: String,
    pub nameKurz: Vec<Name>,
    pub nameLang: Vec<Name>,
    pub nameTechnisch: String,
    pub herausgebernameLang: Vec<Name>,
    pub herausgebernameKurz: Vec<Name>,
    pub beschreibung: Vec<Name>,
    pub versionBeschreibung: Vec<Name>,
    pub aenderungZurVorversion: Vec<Name>,
    pub handbuchVersion: String,
    pub xoevHandbuch: bool,
    pub gueltigAb: u64,
    pub bezugsorte: Vec<String>,
}

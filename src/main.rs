use std::{fs, time::Instant};

use gemeindeschluessel_skript::model::BundResponse;

fn main() {
    let timestamp = Instant::now();
    println!("startet...");
    fs::create_dir_all("./output").expect("deser/ser failed! ");
    deserialize_and_serialize();
    println!(
        "Skript hat {:?}s gebraucht.",
        timestamp.elapsed().as_secs_f32()
    );
}

fn read_gemeindeschluesel() -> String {
    let contents = fs::read_to_string("./data/Regionalschl_ssel_2021-07-31.json")
        .expect("should have been able to read the file!");
    contents
}

fn deserialize_and_serialize() {
    let deserialze_data: BundResponse = serde_json::from_str(&read_gemeindeschluesel()).unwrap();

    for item in deserialze_data.daten {
        let gemeinde_json = serde_json::to_string_pretty(&item).expect("serialize failed!");
        let mut dataname = item.get(1).unwrap().to_owned().unwrap();
        let mut newName = String::from("");
        for character in dataname.chars() {
            if character == ',' || character == '/' {
                break;
            } else {
                print!("{} \n", character);
                newName += character.to_string().as_ref();
            }
        }
        println!("Datenname: {}", newName);

        fs::write(format!("./output/{}.json", newName), gemeinde_json)
            .expect("write file failed!");
    }
}

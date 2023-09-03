use colored::Colorize;
use std::{fs, time::Instant};

use gemeindeschluessel_skript::model::BundResponse;

fn main() {
    let timestamp = Instant::now();
    println!("{}", "startet...".green());
    fs::create_dir_all("./gemeindeschluessel").expect("create dir should be successful");
    deserialize_and_serialize();

    println!(
        "Skript hat {}s gebraucht.",
        timestamp.elapsed().as_secs_f32().to_string().green()
    );
}

fn read_gemeindeschluesel() -> String {
    let contents = fs::read_to_string("./data/Regionalschl_ssel_2021-07-31.json")
        .expect("should have been able to read the file!");
    contents
}

fn deserialize_and_serialize() {
    let deserialze_data: BundResponse = serde_json::from_str(&read_gemeindeschluesel()).unwrap();
    for mut item in deserialze_data.daten {
        item[0] = Some(set_last_seven_to_zero(item.get(0) .unwrap().to_owned().unwrap()));
        println!("{:?}", item[0]);
        let gemeinde_json = serde_json::to_string_pretty(&item).expect("serialize failed!");
        let mut dataname = item.get(1).unwrap().to_owned().unwrap();
        let mut newName = String::from("");
        for character in dataname.chars() {
            if character == ',' || character == '/' {
                break;
            } else {
                newName += character.to_string().as_ref();
            }
        }
        println!("Datei für {} gespeichert!", newName.purple().bold());

        fs::write(format!("./gemeindeschluessel/{}.json", newName), gemeinde_json).expect("write file failed!");
    }
}

fn set_last_seven_to_zero(g_id: String) -> String {
    let mut counter = 0;
    let mut string_result = String::from("");
    for s in g_id.chars() {
        if  counter>4 {
           string_result.push('0');
        } else {
            string_result.push(s);
        }
        counter = counter + 1;
    }
    println!("RESULT: {}", string_result);
    string_result
}

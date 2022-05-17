use std::{ collections::HashSet };
use std::{ thread, time };
use csv::{ ReaderBuilder };
use serde::Deserialize;

mod params;
use params::module;

#[derive(Debug, Deserialize)]
struct Record {
    nro_licitacion: usize,
    _etapa_licitacion: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut v1 = get_data().await?;
    let verb = time::Duration::from_secs(600); // Every 10 minutes (600 secs)

    loop {
        let v2 = get_data().await?;
        let (is_new, tenders) = get_compare(v1.clone(), v2.clone());
        if is_new {
            println!("[+] HAY NUEVAS LICITACIONES: {:#?}", tenders);
            v1 = v2;
        }else{
            println!("[-] SIN LICITACIONES");
        }
        thread::sleep(verb);
    }
}


fn get_compare(mut vec1: Vec<usize>, mut vec2: Vec<usize>)-> (bool, Vec<usize>) {

    vec1.sort(); // SORT
    vec2.sort();

    let w1: HashSet<usize> = vec1.into_iter().collect(); // Remove duplicates
    let w2: HashSet<usize> = vec2.into_iter().collect();
    let diff: HashSet<_> = w2.difference(&w1).cloned().collect();

    let vec = Vec::from_iter(diff);
    let is_diff = if vec.len() > 0 { true } else { false };
    
    return (is_diff, vec);
}


async fn get_data() -> Result<Vec<usize>, Box<dyn std::error::Error>> {

    let url = "https://www.contrataciones.gov.py/";
    let parameters = module::Params::new();

    let uri = format!("{}{}", url, parameters);

    let data = reqwest::get(uri)
    .await?
    .text()
    .await?;

    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(data.as_bytes());
    let mut current_vec: Vec<usize> = Vec::new();

    for record in reader.deserialize() {
        let dd: Record = record?;
        current_vec.push(dd.nro_licitacion);
    }
    current_vec.sort();
    Ok(current_vec)
}


use std::{ collections::HashSet };
use csv::{ ReaderBuilder };
use serde::Deserialize;


mod params;
use params::module;


#[derive(Debug, Deserialize)]
struct Record {
    planificacion_slug: String,
    convocatoria_slug: String,
    adjudicacion_slug: String,
    precalificacion_slug: String,
    convenio_slug: String,
    nro_licitacion: usize,
    nombre_licitacion: String,
    tipo_procedimiento: String,
    categoria: String,
    convocante: String,
    _etapa_licitacion: String,
    etapa_licitacion: String,
    fecha_entrega_oferta: String,
    tipo_licitacion: String,
    fecha_estimada: String,
    fecha_publicacion_convocatoria: String,
    geo: String
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

 
    let v1 = get_data().await?;
    let v2 = vec![414915,414914,414904];

    let is_new = get_compare(v1, v2).0;
    println!("{:#?}", is_new);

    Ok(())
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
    println!("{:#?}", &current_vec);
    Ok(current_vec)
}


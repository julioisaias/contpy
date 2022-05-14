use std::{ thread, time, collections::HashSet };
use csv::{ Error, ReaderBuilder };
use serde::Deserialize;
use std::iter::Iterator;

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

    //let v = get_data().await?;


    let v1 = vec![1,2,3];
    let v2 = vec![3,2,1];


    let r = is_new(&v1,&v2);

    if r {
        let n = get_new(v1, v2);
        println!("HAY NUEVOS: {:#?}", n);
    }else{
        println!("NO HAY NUEVOS");
    }

    
    //println!("{:#?}", v);

//    let ten_millis = time::Duration::from_millis(2000);
//    let now = time::Instant::now();

    Ok(())
}


fn get_new(vec1: Vec<usize>, vec2: Vec<usize>)-> Vec<usize> {
    let item_set: HashSet<_> = vec1.iter().collect();
    let difference: Vec<_> = vec2.into_iter().filter(|item| !item_set.contains(item)).collect();
    return difference;
}


fn is_new(vec1: &Vec<usize>, vec2: &Vec<usize>)-> bool {
    return !vec1.iter().eq(vec2.iter())
}




async fn get_data() -> Result<Vec<usize>, Box<dyn std::error::Error>> {

    let data = reqwest::get("https://www.contrataciones.gov.py/buscador/licitaciones.csv?nro_nombre_licitacion=&etapas_licitacion[0]=CONV&fecha_desde=&fecha_hasta=&tipo_fecha=&convocante_tipo=&convocante_nombre_codigo=&codigo_contratacion=&catalogo[codigos_catalogo_n4]=&catalogo[codigos_catalogo_n4_label]=&page=&order=&convocante_codigos=&convocante_tipo_codigo=&unidad_contratacion_codigo=")
    .await?
    .text()
    //.json::<HashMap<String, String>>()
    .await?;


    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(data.as_bytes());


    //let cantidad: usize = reader.records().count(); // cantidad de registros CSV

    let mut current_vec: Vec<usize> = Vec::new();

    for record in reader.deserialize() {
        let dd: Record = record?;
        current_vec.push(dd.nro_licitacion);
        //println!("nro_licitacion: {}", dd.nro_licitacion );
    }

    current_vec.sort();

    Ok(current_vec)
}


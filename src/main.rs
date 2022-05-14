use csv::{Error, ReaderBuilder};
use serde::Deserialize;

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
    let data = reqwest::get("https://www.contrataciones.gov.py/buscador/licitaciones.csv?nro_nombre_licitacion=&etapas_licitacion[0]=CONV&fecha_desde=&fecha_hasta=&tipo_fecha=&convocante_tipo=&convocante_nombre_codigo=&codigo_contratacion=&catalogo[codigos_catalogo_n4]=&catalogo[codigos_catalogo_n4_label]=&page=&order=&convocante_codigos=&convocante_tipo_codigo=&unidad_contratacion_codigo=")
        .await?
        .text()
        //.json::<HashMap<String, String>>()
        .await?;
    //println!("{:#?}", resp);

    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(data.as_bytes());


    //let cantidad: usize = reader.records().count(); // cantidad de registros CSV

    let mut licitaciones_actuales: Vec<usize> = Vec::new();
  
    for record in reader.deserialize() {
        let dd: Record = record?;
        licitaciones_actuales.push(dd.nro_licitacion);
        //println!("nro_licitacion: {}", dd.nro_licitacion );
    }

    println!("{:#?}", licitaciones_actuales);
    




    Ok(())
}
pub struct Params {
    nro_nombre_licitacion: String,
    etapas_licitacion: String,
    fecha_desde: String,
    fecha_hasta: String,
    tipo_fecha: String,
    convocante_tipo: String,
    convocante_nombre_codigo: String,
    codigo_contratacion: String,
    catalogo_n4: String,
    catalogo_n4_label: String,
    page: String,
    order: String,
    convocante_codigos: String,
    convocante_tipo_codigo: String,
    unidad_contratacion_codigo: String
}

trait Fields {
    fn nro_nombre_licitacion(&self) -> String;
    fn etapas_licitacion(&self) -> String;
    fn fecha_desde(&self) -> String;
    fn fecha_hasta(&self) -> String;
    fn convocante_tipo(&self) -> String;
    fn tipo_fecha(&self) -> String;
    fn convocante_nombre_codigo(&self) -> String;
    fn codigo_contratacion(&self) -> String;
    fn catalogo_n4(&self) -> String;
    fn catalogo_n4_label(&self) -> String;
    fn page(&self) -> String;
    fn order(&self) -> String;
    fn convocante_codigos(&self) -> String;
    fn convocante_tipo_codigo(&self) -> String;
    fn unidad_contratacion_codigo(&self) -> String;
}

impl Fields for Params {
    fn nro_nombre_licitacion(&self) -> String {
        format!("?nro_nombre_licitacion={}", self.nro_nombre_licitacion)
    }
    fn etapas_licitacion(&self) -> String {
        format!("&etapas_licitacion[0]={}", self.etapas_licitacion)
    }
    fn fecha_desde(&self) -> String {
        format!("&fecha_desde={}", self.fecha_desde)
    }
    fn fecha_hasta(&self) -> String {
        format!("&fecha_hasta={}", self.fecha_hasta)
    }
    fn convocante_tipo(&self) -> String {
        format!("&convocante_tipo={}", self.convocante_tipo)
    }
    fn tipo_fecha(&self) -> String {
        format!("&tipo_fecha={}", self.tipo_fecha)
    }
    fn convocante_nombre_codigo(&self) -> String {
        format!("&convocante_nombre_codigo={}", self.convocante_nombre_codigo)
    }
    fn codigo_contratacion(&self) -> String {
        format!("&codigo_contratacion={}", self.codigo_contratacion)
    }
    fn catalogo_n4(&self) -> String {
        format!("&catalogo[codigos_catalogo_n4]={}", self.catalogo_n4)
    }
    fn catalogo_n4_label(&self) -> String {
        format!("&catalogo[codigos_catalogo_n4_label]={}", self.catalogo_n4_label)
    }
    fn page(&self) -> String {
        format!("&page={}", self.page)
    }
    fn order(&self) -> String {
        format!("&order={}", self.order)
    }
    fn convocante_codigos(&self) -> String {
        format!("&convocante_codigos={}", self.convocante_codigos)
    }
    fn convocante_tipo_codigo(&self) -> String {
        format!("&convocante_tipo_codigo={}", self.convocante_tipo_codigo)
    }
    fn unidad_contratacion_codigo(&self) -> String {
        format!("&unidad_contratacion_codigo={}", self.unidad_contratacion_codigo)
    }

}

impl Params {
    pub fn new() -> String {
        let p = Params {
            nro_nombre_licitacion: "".to_string(),
            etapas_licitacion: "CONV".to_string(),
            fecha_desde: "".to_string(),
            fecha_hasta: "".to_string(),
            tipo_fecha: "".to_string(),
            convocante_tipo: "".to_string(),
            convocante_nombre_codigo: "".to_string(),
            codigo_contratacion: "".to_string(),
            catalogo_n4: "".to_string(),
            catalogo_n4_label: "".to_string(),
            page: "".to_string(),
            order: "".to_string(),
            convocante_codigos: "".to_string(),
            convocante_tipo_codigo: "".to_string(),
            unidad_contratacion_codigo: "".to_string()
        };
        
        format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            p.nro_nombre_licitacion(),
            p.etapas_licitacion(),
            p.fecha_desde(),
            p.fecha_hasta(),
            p.tipo_fecha(),
            p.convocante_tipo(),
            p.convocante_nombre_codigo(),
            p.codigo_contratacion(),
            p.catalogo_n4(),
            p.catalogo_n4_label(),
            p.page(),
            p.order(),
            p.convocante_codigos(),
            p.convocante_tipo_codigo(),
            p.unidad_contratacion_codigo(),
        )
    }
}

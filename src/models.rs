use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Libro {
    pub id: i32,
    pub titulo: String,
    pub autor: String,
    pub paginas: i32,
    pub genero: String,
    pub estado: String,
    pub fecha_inicio: String,
    pub fecha_final: String,

}
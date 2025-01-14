use cli::{actualizar_libro, agregar_libro, eliminar_libro, listar_libros, mostrar_menu};
use database::initialize_db;

mod database;
mod cli;
mod models;

fn main(){
    //cli::run_cli();
    loop {
        let opcion = mostrar_menu();

        match opcion.as_str() {
            "Inicializar base de datos" => {
                println!("Inicializando base de datos...");
                initialize_db().expect("Error");
            }
            "Agregar un libro" => {
                println!("Agregando un libro...");
                agregar_libro();
            }
            "Listar libros" => {
                println!("listando libros...");
                listar_libros();
            }
            "Eliminar libro" => {
                println!("Eliminando un libro...");
                eliminar_libro();
            }
            "Actualizar libro" =>{
                println!("Actualizando libro...");
                actualizar_libro();
            }
            "Salir" => {
                println!("Saliendo...");
                break;
            }
            _ => println!("Opcion no valida")
        }
    }
}

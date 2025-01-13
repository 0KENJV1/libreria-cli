//si se utilizara la libreria clap CLI está orientada a comandos que se ejecutan directamente desde la línea de comandos con argumentos.
//use clap::{Arg, Command};

// pub fn run_cli() {
//     let matches = Command::new("Libreria CLI")
//         .version("1.0")
//         .about("Gestiona tu blblioteca personal")
//         .subcommand(Command::new("init").about("Inicializa la base de datos"))
//         .subcommand(Command::new("add")
//             .about("Agrega un libro a la biblioteca")
//             .arg(Arg::new("titulo").required(true))
//             .arg(Arg::new("autor").required(true))
//             .arg(Arg::new("paginas").required(true))
//             .arg(Arg::new("genero").required(true))
//             .arg(Arg::new("estado").required(true))
//             .arg(Arg::new("fecha_inicio").required(true))
//             .arg(Arg::new("fecha_final").required(true))

//         )
//         .subcommand(Command::new("list"). about("muestra todos los libros"))
//         .get_matches();

//     let conn = initialize_db().expect("error al inicializar la base de datos");
//     match matches.subcommand() {
//         Some(("init", _)) => {
//             create_table(&conn).expect("error al crear tabla");
//             println!("Base de datos Inicializada");
//         }
//         Some(("add", sub_m)) => {
//             let libro = Libro {
//                 id: 0,
//                 titulo: sub_m.get_one::<String>("titulo").unwrap().to_string(),
//                 autor: sub_m.get_one::<String>("autor").unwrap().to_string(),
//                 paginas: sub_m.get_one::<String>("paginas").unwrap().parse().expect("Debe ser un numero"),
//                 genero: sub_m.get_one::<String>("genero").unwrap().to_string(),
//                 estado: sub_m.get_one::<String>("estado").unwrap().to_string(),
//                 fecha_inicio: sub_m.get_one::<String>("fecha_inicio").unwrap().to_string(),
//                 fecha_final: sub_m.get_one::<String>("fecha_final").unwrap().to_string(),

//             };
//             instert_book(&conn, &libro).expect("error al insertar libro");
//             println!("libro agregado: {:#?}", libro);
//         }
//         Some(("list", _)) => {
//             let libros = list_books(&conn);
//             for libro in libros {
//                 println!("{:#?}",libro)
//             }
//         }
//         _ => {}
//     }
// }

use inquire::{Select, Text, CustomType};
use crate::database::{delete_book, initialize_db, instert_book, list_books};
use crate::models::Libro;

//mostrar menu
pub fn mostrar_menu() -> String {
    let opciones = vec![
        "Inicializar base de datos",
        "Agregar un libro",
        "Listar libros",
        "Eliminar libro",
        "Salir",
    ];

    return Select::new("Elige una opcion: ", opciones)
        .prompt()
        .expect("Error al mostrar MENU")
        .to_string();
}

//Capturar datos
pub fn agregar_libro(){
    let titulo_variable = Text::new("Introduce el titulo del libro:")
        .prompt()
        .expect("Error al capturar el titulo");

    let autor_variable = Text::new("Introduce el autor del libro:")
        .prompt()
        .expect("Error al capturar el titulo");

    let paginas_variable = CustomType::<i32>::new("Introduce el número de páginas:")
        .prompt()
        .expect("Error al capturar el número de páginas");

    let genero_variable = Text::new("Introduce el genero del libro:")
        .prompt()
        .expect("Error al capturar el titulo");

    let estado_variable = Select::new(
        "Selecciona el estado del libro:",
        vec!["Nuevo", "En proceso", "Terminado"]
    )
        .prompt()
        .expect("Error al capturar el estado")
        .to_string();

    let fecha_inicio_variable = Text::new("Introduce la fecha inicio:")
        .prompt()
        .expect("Error al capturar la fecha inicio");

    let fecha_final_variable = Text::new("Introduce la fecha final:")
        .prompt()
        .expect("Error al capturar la fecha final");

    let libro = Libro {
        id: 0,
        titulo: titulo_variable,
        autor: autor_variable,
        paginas: paginas_variable,
        genero: genero_variable,
        estado: estado_variable,
        fecha_inicio: fecha_inicio_variable,
        fecha_final: fecha_final_variable,
    };
    
    let conn = initialize_db(). expect("Error");
    instert_book(&conn, &libro).expect("Error al guardar libro");

}

pub fn listar_libros(){
    let conn = initialize_db().expect("Error");
    let libros = list_books(&conn);

    for libro in libros  {
        println!("{:#?}", libro);
    }
}

pub fn eliminar_libro(){
    let conn = initialize_db().expect("Error");
    let id = Text::new("Introduce el id del libro que deseas borrar")
        .prompt()
        .expect("Error al introducir id")
        .parse()
        .unwrap();

    delete_book(&conn, id).expect("Error al eliminar el libro");
}
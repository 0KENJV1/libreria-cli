use rusqlite::{params, Connection, Result, ToSql};
use std::fs;

use crate::models::Libro;

//funcion para inicializar la db
pub fn initialize_db() -> Result<Connection> {

    match fs::create_dir_all("data") {
        Ok(_) => println!("Directorio creado exitosamente"),
        Err(e) => eprintln!("Error al crear el directorio: {}", e),        
    }
    //ruta
    let db_path = "data/libreria.db";

    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS libros (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                titulo TEXT NOT NULL, 
                autor TEXT NOT NULL,
                paginas INTEGER NOT NULL,
                genero TEXT NOT NULL,
                estado TEXT NOT NULL CHECK(estado IN ('Nuevo', 'En proceso', 'Terminado', 'Abandonado', 'Por releer')),
                fecha_inicio DATE,
                fecha_final DATE
        )", 
        [],
    )?;
    Ok(conn)
}


//insertar libros
pub fn instert_book(conn: &Connection, libro: &Libro) -> Result<()>{
    conn.execute(
        "INSERT INTO libros (titulo, autor, paginas, genero, estado, fecha_inicio, fecha_final) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", 
        params![libro.titulo, libro.autor, libro.paginas, libro.genero, libro.estado, libro.fecha_inicio, libro.fecha_final],
    )?;

    Ok(())
}

//listar libros
pub fn list_books(conn: &Connection) -> Vec<Libro> {
    let mut stmt = conn.prepare(
        "SELECT id, titulo, autor, paginas, genero, estado, fecha_inicio, fecha_final FROM libros"
    ).expect("Error al consultar");
    let libro_iter = stmt.query_map([], |row|{
        Ok(Libro {
            id: row.get(0)?,
            titulo: row.get(1)?,
            autor: row.get(2)?,
            paginas: row.get(3)?,
            genero: row.get(4)?,
            estado: row.get(5)?,
            fecha_inicio: row.get(6)?,
            fecha_final: row.get(7)?,
        })
    }). expect("Error al mapear resultados");

    libro_iter.filter_map(Result::ok).collect()
    
    // println!("libros en la base de datos:");
    // for libro in libro_iter {
    //     println!("{:#?}", libro?);
    // }

    
}

pub fn delete_book(conn: &Connection, id: i32) -> Result<()>{
    conn.execute("DELETE FROM libros WHERE id = ?1", [id])?;
    Ok(())
}

pub fn update_book<T: ToSql>(conn: &Connection, campo:&str, nuevo_valor: T, id: i32 ) -> Result<()> {
    let sql = format!("UPDATE libros SET {} = ?1 WHERE id = ?2", campo);
    conn.execute(&sql, params![nuevo_valor, id])?;

    Ok(())
}

// pub fn delete_table(conn: &Connection) -> Result<()> {
//     conn.execute("DROP TABLE IF EXISTS libros", [])?;
//     Ok(())
// }

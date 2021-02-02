use std::error::Error;
use std::fs;
use std::process;
use csv::{ReaderBuilder};
use postgres::{Client, NoTls};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
//use std::io;

use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

use std::io::prelude::*;



pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/upload", web::post().to(upload_files));
    //cfg.service(web::scope("/upload").route("/", web::post().to(upload_files)));
    
}

pub async fn upload_files() -> impl Responder {
    HttpResponse::Ok().json("Se procesó la carga del archivo")
}

fn carga_archivo() -> Result<(), Box<dyn Error>> {
//pub fn carga_archivo(cfg: &mut web::ServiceConfig) {
    
    let contents = fs::read_to_string("data/personas.csv").expect("No se encontro el archivo");
    
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(contents.as_bytes());

    let mut client = Client::connect("postgresql://postgres:admin@localhost:5432", NoTls)?;

    for result in rdr.records() {
        let record = result?;
        client.execute(
                    "INSERT INTO PUBLIC.\"PERSONA\" (\"IDENTIFICACION\", \"NOMBRE\", \"GENERO\", \"ESTADOCIVIL\", \"FECNAC\", \"TELEFONO\", \"DIRECCION\", \"EMAIL\") VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                    &[&record.get(0), &record.get(1), &record.get(2), &record.get(3), &record.get(4), &record.get(5), &record.get(6), &record.get(7)],
            )?;
    }
    Ok(())
}

//#[actix_rt::main]
//async fn main() -> io::Result<()> {
fn main() {
    // Construct app and configure routes
    let listener = TcpListener::bind("127.0.0.1:3000/upload"). unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream)
        });
    }

    /*if let Err(err) = carga_archivo() {
        println!("Error al cargar archivo: {}", err);
    }*/

    /*let app = move || {
        App::new()
        .configure(general_routes)
    };

    // Start HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await*/
}

fn handle_connection(mut stream: TcpStream) {
    let contents = "Se procesó la carga del archivo";

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    if let Err(err) = carga_archivo() {
        println!("Error al cargar archivo: {}", err);
    }
}



#![allow(unused_must_use)]

use std::io;
use std::path::Path;
use std::sync::Arc;

use std::error::Error;

use actix_cors::Cors;
// use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use arc_swap::ArcSwap;
use joie::builder::DatabaseBuilder;
use satt::{DatabaseHandle, EpMetadata, StoredEpisode};
use tempfile::TempDir;

// use curiosity::db::Db;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn blank_db(path: impl AsRef<Path>) -> io::Result<DatabaseHandle> {
    let builder: DatabaseBuilder<StoredEpisode, EpMetadata, ()> = DatabaseBuilder::default();

    let dir = TempDir::new_in(path)?;

    Ok(DatabaseHandle {
        db: builder.build_in(dir.path())?,
        underlying_dir: dir,
    })
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::fs::create_dir_all("./database");
    let db_handle = Arc::new(ArcSwap::from_pointee(blank_db("./database")?));
    satt::update::update_database(db_handle.clone(), "./database".into())
        .await
        .unwrap();

    let db_for_update = db_handle.clone();
    actix_web::rt::spawn(satt::update::update_database_periodically(
        db_for_update,
        "./database",
    ));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(web::scope("/api").service(satt::api::search::search))
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
            .app_data(web::Data::new(db_handle.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}

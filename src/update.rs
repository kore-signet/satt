use std::io::{Cursor, Read};

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::BTreeMap, path::Path};

use joie::builder::{DatabaseBuilder, DocumentData};
use tempfile::TempDir;

use crate::{
    DatabaseHandle, DownloadOptions, EpMetadata, Season, SeasonId, SharedDatabaseHandle,
    StoredEpisode,
};

pub async fn update_database_periodically(db: SharedDatabaseHandle, db_dir: impl AsRef<Path>) {
    let path: PathBuf = db_dir.as_ref().into();
    let mut interval = actix_web::rt::time::interval(Duration::from_secs(6 * 60 * 60));
    loop {
        interval.tick().await;
        println!("Updating!");

        if let Err(e) = update_database(db.clone(), path.clone()).await {
            println!("error during db update: {e}");
        }
    }
}

pub async fn update_database(
    db: SharedDatabaseHandle,
    db_dir: PathBuf,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    actix_web::rt::task::spawn_blocking(move || {
        let mut mirror_reader = ureq::get("https://memorious-records-2.cat-girl.gay/all-plaintext.zip").call()?.into_reader();
        let mut mirror_bytes = Vec::new();
        mirror_reader.read_to_end(&mut mirror_bytes)?;

        let mut mirror = zip::ZipArchive::new(Cursor::new(mirror_bytes))?;
        let mut builder: DatabaseBuilder<StoredEpisode, EpMetadata, ()> =
            DatabaseBuilder::default();

        let seasons: BTreeMap<SeasonId, Season> = serde_json::from_reader(
            mirror.by_name("seasons.json")?,
        )?;

        for Season { id, episodes, .. } in seasons.into_values() {
            let season_id = id;

            for episode in episodes {
                let Some(DownloadOptions { plain }) = episode.download else { continue };

                let mut episode_file = match mirror.by_name(
                    &plain.to_string_lossy()
                ) {
                    Ok(v) => v,
                    Err(e) => { eprintln!("Couldn't read {}: {e}", &episode.title); continue },
                };

                

                let mut text = String::with_capacity((episode_file.compressed_size() * 2) as usize);
                episode_file.read_to_string(&mut text)?;

                let ep_id: u32 = ((season_id as u32 + 1) * 1000) + episode.sorting_number as u32;

                println!("ID: {ep_id} - {}", &episode.title);

                builder.add_document(DocumentData {
                    id: ep_id,
                    text: &text,
                    metadata: EpMetadata {
                        season: season_id as u8,
                    },
                    data: StoredEpisode {
                        title: episode.title,
                        slug: episode.slug,
                        docs_id: episode.docs_id,
                        season: season_id,
                    },
                });
            }
        }

        let temp_dir = TempDir::new_in(db_dir)?;
        let database = builder.build_in(temp_dir.path())?;

        let handle = DatabaseHandle {
            db: database,
            underlying_dir: temp_dir,
        };

        println!("finished updating");

        db.swap(Arc::new(handle));

        Ok(())
    })
    .await
    .unwrap()
}

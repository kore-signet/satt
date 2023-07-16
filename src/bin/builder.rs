use std::{collections::BTreeMap, path::Path};

use joie::{
    builder::{DatabaseBuilder, DocumentData},
    sentence::SentencePart,
};
use satt::{DownloadOptions, EpMetadata, Season, SeasonId, StoredEpisode};

#[allow(dead_code)]
fn print_highlights(parts: &[SentencePart<'_>]) {
    for part in parts {
        match part {
            SentencePart::Normal(s) => print!("{s}"),
            SentencePart::Highlight(s) => print!("*{s}*"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = std::fs::create_dir_all("./database");
    let mut builder: DatabaseBuilder<StoredEpisode, EpMetadata, ()> = DatabaseBuilder::default();

    let seasons: BTreeMap<SeasonId, Season> =
        serde_json::from_str(include_str!("../../data/seasons.json"))?;

    for Season { id, episodes, .. } in seasons.into_values() {
        let season_id = id;

        for episode in episodes {
            let Some(DownloadOptions { plain }) = episode.download else { continue };
            let text = std::fs::read_to_string(Path::new("./data/").join(plain))?;
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

    let db = builder.build_in("./database")?;

    db.persist("./database")?;

    Ok(())
}

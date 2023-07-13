use std::{collections::BTreeMap, path::Path};

use joie::{
    builder::{DatabaseBuilder, DocumentData},
    sentence::SentencePart,
};
use satt::{DownloadOptions, EpMetadata, Season, SeasonId, StoredEpisode};

fn print_highlights(parts: &[SentencePart<'_>]) {
    for part in parts {
        match part {
            SentencePart::Normal(s) => print!("{s}"),
            SentencePart::Highlight(s) => print!("*{s}*"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("./database");
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

    let q = "(keith or austin) and any sound";

    let query = db.parse_query(q, (), false).unwrap();

    let query_opt = db.parse_query(q, (), true).unwrap();

    assert_eq!(db.query(&query_opt).count(), db.query(&query).count());

    println!(
        "full query (optimized): {}",
        easybench::bench(|| { db.query(&query_opt).take(50).for_each(|_| {}) })
    );

    println!(
        "full query (non-optimized): {}",
        easybench::bench(|| { db.query(&query).take(50).for_each(|_| {}) })
    );

    for mut res in db.query(&query_opt).take(50) {
        query_opt.find_highlights(&mut res);
        println!("--/ {} /--", db.get_doc(&res.id.doc).unwrap().title);
        print_highlights(&res.highlights());
        println!();
        println!();
    }

    Ok(())
}

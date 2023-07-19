use joie::{sentence::SentencePart, Database};
use satt::{EpMetadata, SeasonId, StoredEpisode};

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
    let db: Database<StoredEpisode, EpMetadata, ()> = Database::load("./database")?;

    bench_query("(jack or lem) and emmanuel", &db);
    bench_query("belgard or signet", &db);
    bench_query("(keith or austin) and any sound", &db);
    bench_query("(keith and austin) and any sound", &db);

    // for mut res in db.query(&query_opt).take(50) {
    //     query_opt.find_highlights(&mut res);
    //     println!("--/ {} /--", db.get_doc(&res.id.doc).unwrap().title);
    //     print_highlights(&res.highlights());
    //     println!();
    //     println!();
    // }

    Ok(())
}

fn bench_query(q: &str, db: &Database<StoredEpisode, EpMetadata, ()>) {
    println!("-// benchmarking query: '{q}' //-");

    let query = db
        .parse_query(
            q,
            |s: &EpMetadata| s.season == SeasonId::TwilightMirage as u8,
            false,
        )
        .unwrap();
    let query_opt = db
        .parse_query(
            q,
            |s: &EpMetadata| s.season == SeasonId::TwilightMirage as u8,
            true,
        )
        .unwrap();
    assert_eq!(db.query(&query_opt).count(), db.query(&query).count());

    println!(
        "\toptimized:    {:>6}",
        easybench::bench(|| { db.query(&query_opt).take(50).for_each(|_| {}) })
    );

    println!(
        "\tnon-optimized: {:>6}",
        easybench::bench(|| { db.query(&query).take(50).for_each(|_| {}) })
    );

    println!();
}

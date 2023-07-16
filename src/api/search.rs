use std::time::Instant;

use actix_web::{http::StatusCode, web, HttpResponse, HttpResponseBuilder};

use itertools::Itertools;
use joie::query::Query;
use joie::sentence::SentenceRange;
use nyoom_json::UnescapedStr;

use crate::{
    api::QueryKind, EpMetadata, SeasonId, ServerError, ServerResult, SharedDatabaseHandle,
};

use super::SearchRequest;

#[actix_web::get("/search")]
pub async fn search(
    query: web::Query<SearchRequest>,
    db: web::Data<SharedDatabaseHandle>,
) -> ServerResult<HttpResponse> {
    let request = query.into_inner();
    let mut request: SearchRequest =
        if let Some(page) = request.page.as_ref().filter(|page| page.as_str() != "null") {
            let mut out = Vec::with_capacity(128);
            base64_url::decode_to_vec(&page, &mut out).map_err(|_| ServerError::BadPageToken)?;
            postcard::from_bytes(&out).map_err(|_| ServerError::BadPageToken)?
        } else {
            request
        };

    let db = db.load();

    request.seasons.sort();

    let query = {
        if request.seasons.is_empty() {
            match request.kind {
                QueryKind::Phrase => db.phrase_query(&request.query, ()),
                QueryKind::Advanced => db
                    .parse_query(&request.query, (), true)
                    .ok_or(ServerError::InvalidQuery)?,
            }
        } else {
            let seasons = request.seasons.clone();
            let filter = move |meta: &EpMetadata| {
                let season: u8 = meta.season;
                let season: SeasonId = unsafe { std::mem::transmute(season) };

                seasons.binary_search(&season).is_ok()
            };

            match request.kind {
                QueryKind::Phrase => db.phrase_query(&request.query, filter),
                QueryKind::Advanced => db
                    .parse_query(&request.query, filter, true)
                    .ok_or(ServerError::InvalidQuery)?,
            }
            // db.parse_query(&request.query, )
        }
    };

    let mut out = String::with_capacity(50_000);
    let mut ser = nyoom_json::Serializer::new(&mut out);
    let mut response_obj = ser.object();
    let mut episodes = response_obj.array_field("episodes");

    let mut results_count = 0;
    let page_size = std::cmp::min(100, request.page_size);

    let querying_start_moment = Instant::now();

    let results_iter = db.query(&query).group_by(|r| r.id.doc);
    for (doc_id, results) in results_iter
        .into_iter()
        .skip(request._curiosity_internal_offset)
        .take(page_size)
    {
        results_count += 1;

        let mut episode = episodes.add_object();
        let doc = db.get_doc(&doc_id).ok_or(ServerError::NotFound)?;

        episode.field("curiosity_id", doc_id);
        episode.field("slug", doc.slug.as_str());
        episode.field("title", doc.title.as_str());

        if let Some(docs_id) = doc.docs_id.as_ref() {
            episode.field("docs_id", docs_id.as_str());
        }

        episode.field("season", doc.season.as_ref());

        let mut highlights = episode.array_field("highlights");
        for mut res in results {
            query.find_highlights(&mut res);

            let mut sentence = highlights.add_array();

            let mut cursor = 0;

            // let mut results = Vec::with_capacity(ranges.len() * 2);

            for SentenceRange { start, end } in res.highlighted_parts.iter().copied() {
                if cursor < start {
                    sentence.add_complex(|v| {
                        let mut v = v.object();
                        v.field("highlighted", false);
                        v.field("text", &res.sentence.text[cursor..start]);
                        v.end()
                    });
                    // results.push(SentencePart::Normal(&text[cursor..start]));
                }

                sentence.add_complex(|v| {
                    let mut v = v.object();
                    v.field("highlighted", true);
                    v.field("text", &res.sentence.text[start..end]);
                    v.end()
                });

                cursor = end;
            }

            if cursor < res.sentence.text.len() {
                sentence.add_complex(|v| {
                    let mut v = v.object();
                    v.field("highlighted", false);
                    v.field("text", &res.sentence.text[cursor..]);
                    v.end()
                });
            }

            sentence.end();
        }

        highlights.end();

        episode.end();
    }

    let query_time = querying_start_moment.elapsed();

    let next_page = if results_count >= page_size {
        request._curiosity_internal_offset += results_count;
        Some(base64_url::encode(&postcard::to_stdvec(&request).unwrap()))
    } else {
        None
    };

    episodes.end();

    response_obj.field(UnescapedStr::create("next_page"), next_page.as_deref());
    response_obj.field(UnescapedStr::create("query_time"), query_time.as_secs_f64());

    response_obj.end();

    Ok(HttpResponseBuilder::new(StatusCode::OK)
        .content_type("application/json")
        .body(out))
}

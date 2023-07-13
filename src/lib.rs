// use std::collections::{BTreeMap, HashMap};
// use std::error::Error;

// use std::path::{Path, PathBuf};

// pub mod db;
// pub mod store;
// use store::*;

// use crate::db::{Db, Sentence, SentenceId, TermMap, print_highlights};

use std::{fmt::Display, ops::Deref, path::PathBuf, sync::Arc};

use actix_web::{body::BoxBody, http::StatusCode, HttpResponseBuilder, ResponseError};

use arc_swap::ArcSwap;
use joie::Database;
use tempfile::TempDir;
pub mod api;
pub mod update;

pub type SharedDatabaseHandle = Arc<ArcSwap<DatabaseHandle>>;
pub struct DatabaseHandle {
    pub db: Database<StoredEpisode, EpMetadata, ()>,
    pub underlying_dir: TempDir,
}

impl Deref for DatabaseHandle {
    type Target = Database<StoredEpisode, EpMetadata, ()>;

    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

use strum::{AsRefStr, Display, IntoStaticStr};

#[derive(
    Debug,
    PartialEq,
    Eq,
    Display,
    IntoStaticStr,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    Ord,
    PartialOrd,
)]
#[serde(rename_all = "kebab-case")]
#[archive_attr(derive(
    Debug,
    PartialEq,
    Eq,
    AsRefStr,
    Display,
    rkyv::Archive,
    rkyv::Deserialize,
    rkyv::Serialize,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    Ord,
    PartialOrd,
))]
#[archive_attr(serde(rename_all = "kebab-case"))]
#[archive_attr(strum(serialize_all = "kebab-case"))]
#[repr(u8)]
#[derive(Default)]
pub enum SeasonId {
    #[strum(serialize = "autumn-in-hieron")]
    AutumnInHieron = 0,
    #[strum(serialize = "marielda")]
    Marielda = 1,
    #[strum(serialize = "winter-in-hieron")]
    WinterInHieron = 2,
    #[strum(serialize = "spring-in-hieron")]
    SpringInHieron = 3,
    #[strum(serialize = "counterweight")]
    Counterweight = 4,
    #[strum(serialize = "twilight-mirage")]
    TwilightMirage = 5,
    #[strum(serialize = "road-to-partizan")]
    RoadToPartizan = 6,
    #[strum(serialize = "partizan")]
    Partizan = 7,
    #[strum(serialize = "road-to-palisade")]
    RoadToPalisade = 8,
    #[strum(serialize = "palisade")]
    Palisade = 9,
    #[strum(serialize = "sangfielle")]
    Sangfielle = 10,
    #[strum(serialize = "extras")]
    Extras = 11,
    #[strum(serialize = "patreon")]
    Patreon = 12,
    #[strum(serialize = "unknown-string")]
    #[default]
    Other = 13,
}

#[derive(serde::Deserialize)]
pub struct Season {
    pub title: String,
    pub id: SeasonId,
    pub episodes: Vec<Episode>,
}

#[derive(serde::Deserialize)]
pub struct Episode {
    pub title: String,
    pub slug: String,
    pub done: bool,
    pub sorting_number: usize,
    pub docs_id: Option<String>,
    pub download: Option<DownloadOptions>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, Default)]
pub struct StoredEpisode {
    pub title: String,
    pub slug: String,
    pub docs_id: Option<String>,
    pub season: SeasonId,
}

#[derive(serde::Deserialize)]
pub struct DownloadOptions {
    pub plain: PathBuf,
}

#[derive(
    Clone, Default, PartialEq, Eq, PartialOrd, Ord, Copy, bytemuck::Pod, bytemuck::Zeroable,
)]
#[repr(transparent)]
pub struct EpMetadata {
    pub season: u8,
}

pub type SentenceMetadata = ();

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ServerError {
    NotFound,
    InvalidQuery,
    BadPageToken,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status(), self.reason())
    }
}

impl ServerError {
    pub fn reason(&self) -> &'static str {
        match self {
            ServerError::NotFound => "not found!",
            ServerError::InvalidQuery => "invalid query!",
            ServerError::BadPageToken => "bad page token!",
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            ServerError::NotFound => StatusCode::NOT_FOUND,
            ServerError::InvalidQuery | ServerError::BadPageToken => StatusCode::BAD_REQUEST,
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> actix_web::HttpResponse<BoxBody> {
        #[derive(serde::Serialize)]
        struct ErrResponse<'a> {
            err: bool,
            msg: &'a str,
        }

        HttpResponseBuilder::new(self.status()).json(ErrResponse {
            err: true,
            msg: self.reason(),
        })
    }
}

pub type ServerResult<T> = Result<T, ServerError>;

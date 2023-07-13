pub mod search;

use crate::SeasonId;

use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use smartstring::{Compact, SmartString};

#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    #[serde(alias = "q", default)]
    pub query: SmartString<Compact>,
    #[serde(default)]
    pub kind: QueryKind,
    #[serde(default, deserialize_with = "deserialize_stringified_list")]
    pub seasons: ArrayVec<SeasonId, 16>,
    #[serde(default)]
    pub highlight: bool,
    #[serde(default)]
    pub _curiosity_internal_offset: usize,
    #[serde(default)]
    pub page: Option<String>,
    #[serde(default = "page_size_default")]
    pub page_size: usize,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum QueryKind {
    #[default]
    Phrase,
    Advanced,
}

use serde::de::IntoDeserializer;

pub fn page_size_default() -> usize {
    50
}

// https://github.com/actix/actix-web/issues/1301#issuecomment-747403932
pub fn deserialize_stringified_list<'de, D, I>(
    deserializer: D,
) -> std::result::Result<ArrayVec<I, 16>, D::Error>
where
    D: serde::de::Deserializer<'de>,
    I: serde::de::DeserializeOwned,
{
    struct StringVecVisitor<I>(std::marker::PhantomData<I>);

    impl<'de, I> serde::de::Visitor<'de> for StringVecVisitor<I>
    where
        I: serde::de::DeserializeOwned,
    {
        type Value = ArrayVec<I, 16>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing a list")
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v.is_empty() {
                return Ok(ArrayVec::new());
            }

            let mut ids = ArrayVec::new();
            for id in v.split(',') {
                let id = I::deserialize(id.into_deserializer())?;
                ids.push(id);
            }
            Ok(ids)
        }
    }

    if deserializer.is_human_readable() {
        deserializer.deserialize_any(StringVecVisitor(std::marker::PhantomData::<I>))
    } else {
        ArrayVec::deserialize(deserializer)
    }
}

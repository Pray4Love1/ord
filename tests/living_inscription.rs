use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InscriptionCore {
    pub version: u32,
    pub parent_hash: Option<String>,
    pub creator: String,
    pub timestamp: DateTime<Utc>,
    pub content_uri: String,
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InscriptionState {
    pub block_height: u64,
    pub external_entropy: Option<String>,
    pub mood: Option<String>,
    pub mirror_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivingInscription {
    pub core: InscriptionCore,
    pub state: InscriptionState,
    pub signature: String,
}

impl LivingInscription {
    pub fn commitment(&self) -> String {
        let mut core = Map::new();
        core.insert("version".into(), json!(self.core.version));
        if let Some(parent_hash) = &self.core.parent_hash {
            core.insert("parent_hash".into(), json!(parent_hash));
        }
        core.insert("creator".into(), json!(self.core.creator));
        core.insert(
            "timestamp".into(),
            json!(self.core.timestamp.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)),
        );
        core.insert("content_uri".into(), json!(self.core.content_uri));
        core.insert("metadata".into(), self.core.metadata.clone());

        let mut state = Map::new();
        state.insert("block_height".into(), json!(self.state.block_height));
        if let Some(entropy) = &self.state.external_entropy {
            state.insert("external_entropy".into(), json!(entropy));
        }
        if let Some(mood) = &self.state.mood {
            state.insert("mood".into(), json!(mood));
        }
        if let Some(mirror_hash) = &self.state.mirror_hash {
            state.insert("mirror_hash".into(), json!(mirror_hash));
        }

        let mut root = Map::new();
        root.insert("core".into(), Value::Object(core));
        root.insert("state".into(), Value::Object(state));

        Value::Object(root).to_string()
    }
}

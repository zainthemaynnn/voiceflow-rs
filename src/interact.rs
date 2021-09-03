//! Input requests to Voiceflow.

use crate::intents::IntentTypes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerboseValue {
    pub canonical_text: String,
    pub raw_text: String,
    pub start_index: i32,
}

/// Semantic information within a phrase.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose_value: Option<Vec<VerboseValue>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum Request<I: IntentTypes> {
    Text(String),
    #[serde(rename_all = "camelCase")]
    Intent {
        query: String,
        intent: I,
        entities: Vec<Entity>,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence: Option<f32>,
    },
    Launch,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Trace {
    Block,
    Debug,
    Flow,
}

/// Settings for requests to state API.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// Send back a text-to-speech mp3 audio snippet for every speak block.
    pub tts: bool,
    /// Remove all SSML tags in the response message.
    pub strip_ssml: bool,
    /// Stop on all custom traces.
    pub stop_all: bool,
    /// Which trace types to return prematurely on, define your own path.
    ///
    /// Since types are used as enum names, some of which have fields, it's not feasible
    /// to serialize the actual variants as types. For this reason, `&'static str` is used instead.
    /// The `VFAction`s already contained inside of this crate use `#[derive(strum_macros::IntoStaticStr)]`.
    pub stop_types: Vec<&'static str>,
    /// Trace types to not include in the response.
    pub exclude_types: Vec<Trace>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tts: false,
            strip_ssml: true,
            stop_all: true,
            stop_types: Vec::new(),
            exclude_types: vec![Trace::Block, Trace::Debug, Trace::Flow],
        }
    }
}

/// Enfo and config required for an interaction.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Interact<'a, I: IntentTypes> {
    /// An object representing a user's action or event.
    request: Option<&'a Request<I>>,
    /// Optional settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<&'a Config>,
}

/// Request for the `Interact` API.
impl<'a, I: IntentTypes + Serialize> Interact<'a, I> {
    pub fn new(request: &'a Request<I>) -> Self {
        Self {
            request: Some(request),
            config: None,
        }
    }

    pub fn with_conf(request: &'a Request<I>, config: &'a Config) -> Self {
        Self {
            request: Some(request),
            config: Some(config),
        }
    }

    /// Serializes request to json.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    pub fn request(&self) -> Option<&Request<I>> {
        self.request
    }

    pub fn config(&self) -> Option<&Config> {
        self.config
    }
}

use serde::{Serialize, Deserialize};

/// Serializes and deserializes into an intent.
pub trait IntentTypes {}

/// An intent (semantic response). Default VF intents are included.
/// To implement custom `Intent` variants, use the `Intents` enum with a generic.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "SCREAMING_SNAKE_CASE")]
// TODO: use this when `macro_attributes_in_derive_output` feature becomes stable
// crate: `serde_prefix`
// #[prefix_all("VF.")]
pub enum VFIntent {
    #[serde(rename = "VF.YES")]
    Yes,
    #[serde(rename = "VF.NO")]
    No,
    #[serde(rename = "VF.RESUME")]
    Resume,
    #[serde(rename = "VF.PAUSE")]
    Pause,
    #[serde(rename = "VF.NEXT")]
    Next,
    #[serde(rename = "VF.PREVIOUS")]
    Previous,
    #[serde(rename = "VF.REPEAT")]
    Repeat,
    #[serde(rename = "VF.STOP")]
    Stop,
    #[serde(rename = "VF.HELP")]
    Help,
    #[serde(rename = "VF.CANCEL")]
    Cancel,
    #[serde(rename = "VF.START_OVER")]
    StartOver,
    #[serde(rename = "None")]
    Fallback,
}

impl IntentTypes for VFIntent {}
impl Default for VFIntent {
    fn default() -> Self {
        Self::Fallback
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Intent<I: IntentTypes> {
    VF(VFIntent),
    Custom(I),
}

impl<I: IntentTypes> IntentTypes for Intent<I> {}

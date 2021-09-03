//! Output responses from Voiceflow.

use crate::interact::Request;
use crate::intents::IntentTypes;
use serde::de::{self, IntoDeserializer};
use serde::{Deserialize, Deserializer};
use strum_macros::IntoStaticStr;

pub mod visual {
    use super::*;

    #[derive(Copy, Clone, Debug, Deserialize)]
    #[serde(rename_all = "snake_case")] // why is this casing different anyways?
    pub enum Device {
        Mobile,
        Tablet,
        Desktop,
        #[serde(rename = "echo_show_8")] // `8` is not separated
        EchoShow8,
        GoogleNestHub,
        SmartWatch,
        Television,
        InCarDisplay,
    }

    #[derive(Copy, Clone, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum CanvasVisibility {
        Full,
        Cropped,
        Hidden,
    }

    #[derive(Copy, Clone, Debug, Deserialize)]
    pub struct Dimensions {
        pub width: u32,
        pub height: u32,
    }

    #[derive(Clone, Debug, Deserialize)]
    #[serde(tag = "visualType", rename_all = "camelCase")]
    pub enum Visual {
        #[serde(rename_all = "camelCase")]
        Image {
            #[serde(rename = "image")] // renamed for consistency. deal with it.
            source: String,
            device: Option<Device>,
            dimensions: Option<Dimensions>,
            canvas_visibility: CanvasVisibility,
        },
    }
}

pub use visual::Visual;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Speak {
    #[serde(rename_all = "camelCase")]
    Message { message: String },
    #[serde(rename_all = "camelCase")]
    Audio {
        #[serde(rename = "src")]
        source: String,
        message: String,
    },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Button<I: IntentTypes> {
    pub name: String,
    pub request: Request<I>,
}

// uses custom deserialize to work with paths properly
// see implementation for more info
/// Describes a link between two conversation blocks.
#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", remote = "Path")]
pub enum Path {
    Path,
    Choice(u32),
    Jump,
    Capture,
}

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some(s) = s.strip_prefix("choice") {
            // `choice` paths look like this
            // `"choice:<n>"` where *n* is an integer
            let n = s
                .strip_prefix(':')
                .ok_or(de::Error::custom("expected `:` delimiter in `choice` path"))?
                .parse::<u32>()
                .map_err(|_| de::Error::custom("expected `u32` in `choice` path"))?;
            Ok(Self::Choice(n))
        } else {
            Self::deserialize(s.into_deserializer())
        }
    }
}

/// Next action. Returned from POST request to the endpoint.
#[derive(Clone, Debug, Deserialize, IntoStaticStr)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum VFAction<I: IntentTypes> {
    Visual(Visual),
    Speak(Speak),
    Choice {
        buttons: Vec<Button<I>>,
    },
    Path {
        path: Path,
    },
    End,
}

/// Use a generic to extend the default action types.
/// This allows JSON to deserialize into either category.
///
/// ```
/// use voiceflow::output::{Action, ActionTypes, VFAction};
/// use voiceflow::intents::VFIntent; // default intents
///
/// enum CustomAction {
///     MyAction,
///     // ...
/// }
///
/// impl ActionTypes for CustomActions {}
///
/// type A = Action<VFIntent, CustomAction>;
/// let variant_1 = A::VF(VFAction::Yes);
/// let variant_2 = A::Custom(CustomAction::MyAction);
/// ```
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Action<I: IntentTypes, A: ActionTypes> {
    VF(VFAction<I>),
    Custom(A),
}

/// Contains action variants.
pub trait ActionTypes {}

impl<I: IntentTypes> ActionTypes for VFAction<I> {}
impl<I: IntentTypes, A: ActionTypes> ActionTypes for Action<I, A> {}

pub trait Event {}

/// This is similar to a normal action, however it contains two extra fields.
/// If you are not using custom actions then raw `VFAction`s will save memory.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomAction<A: ActionTypes, E: Event> {
    #[serde(flatten)]
    pub action: A,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_path: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<E>>,
}

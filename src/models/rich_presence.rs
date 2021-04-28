#![cfg(feature = "rich_presence")]

use std::default::Default;

use utils;

use super::shared::PartialUser;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SetActivityArgs {
    pid: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity: Option<Activity>,
}

impl SetActivityArgs {
    pub fn new(activity: Activity) -> Self {
        Self { pid: utils::pid(), activity: Some(activity) }
    }
}

impl Default for SetActivityArgs {
    fn default() -> Self {
        Self { pid: utils::pid(), activity: None }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SendActivityJoinInviteArgs {
    pub user_id: String,
}

pub type CloseActivityRequestArgs = SendActivityJoinInviteArgs;

impl SendActivityJoinInviteArgs {
    pub fn new(user_id: u64) -> Self {
        Self { user_id: user_id.to_string() }
    }
}

builder! {
    ActivityJoinEvent {
        secret: String,
    }
}

builder! {
    ActivitySpectateEvent {
        secret: String,
    }
}

builder! {
    ActivityJoinRequestEvent {
        user: PartialUser,
    }
}


#[derive(Builder, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[builder(setter(strip_option))]
pub struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    instance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    timestamps: Option<ActivityTimestamps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    assets: Option<ActivityAssets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    party: Option<ActivityParty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    secrets: Option<ActivitySecrets>,
    #[serde(skip_serializing_if = "Option::is_none", skip_deserializing)]
    #[builder(default)]
    buttons: Option<Vec<ActivityButton>>,
}

builder! {
    ActivityTimestamps {
        start: u64,
        end: u64,
    }
}

builder! {
    ActivityAssets {
        large_image: String,
        large_text: String,
        small_image: String,
        small_text: String,
    }
}

builder! {
    ActivityParty {
        id: u32,
        size: (u32, u32),
    }
}

builder! {
    ActivityButton {
        label: String,
        url: String,
    }
}

#[derive(Builder, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[builder(setter(strip_option))]
pub struct ActivitySecrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    join: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    spectate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "match")]
    #[builder(default)]
    game: Option<String>,
}


#[cfg(test)]
mod tests {
    use serde_json;

    use super::*;

    const FULL_JSON: &'static str =
        r###"{
  "state": "rusting",
  "details": "detailed",
  "instance": true,
  "timestamps": {
    "start": 1000,
    "end": 2000
  },
  "assets": {
    "large_image": "ferris",
    "large_text": "Ferris",
    "small_image": "rusting",
    "small_text": "Rusting..."
  },
  "party": {
    "id": 1,
    "size": [
      3,
      6
    ]
  },
  "secrets": {
    "join": "025ed05c71f639de8bfaa0d679d7c94b2fdce12f",
    "spectate": "e7eb30d2ee025ed05c71ea495f770b76454ee4e0",
    "match": "4b2fdce12f639de8bfa7e3591b71a0d679d7c93f"
  },
  "buttons": [
    {
      "label": "Click me!",
      "url": "https://example.com"
    }
  ]
}"###;

    #[test]
    fn test_serialize_full_activity() {
        let timestamps = ActivityTimestampsBuilder::default()
            .start(1000)
            .end(2000)
            .build().unwrap();
        let assets = ActivityAssetsBuilder::default()
            .large_image("ferris".into())
            .large_text("Ferris".into())
            .small_image("rusting".into())
            .small_text("Rusting...".into())
            .build().unwrap();
        let party = ActivityPartyBuilder::default()
            .id(1)
            .size((3, 6))
            .build().unwrap();
        let secrets = ActivitySecretsBuilder::default()
            .join("025ed05c71f639de8bfaa0d679d7c94b2fdce12f".into())
            .spectate("e7eb30d2ee025ed05c71ea495f770b76454ee4e0".into())
            .game("4b2fdce12f639de8bfa7e3591b71a0d679d7c93f".into())
            .build().unwrap();
        let button = ActivityButtonBuilder::default()
            .label("Click me!".into())
            .url("https://example.com".into())
            .build().unwrap();
        let activity = ActivityBuilder::default()
            .state("rusting".into())
            .details("detailed".into())
            .instance(true)
            .timestamps(timestamps)
            .assets(assets)
            .party(party)
            .secrets(secrets)
            .buttons(vec![button])
            .build().unwrap();

        let json = serde_json::to_string_pretty(&activity).unwrap();

        assert_eq![json, FULL_JSON];
    }

    #[test]
    fn test_serialize_empty_activity() {
        let activity = Activity::new();
        let json = serde_json::to_string(&activity).unwrap();
        assert_eq![json, "{}"];
    }
}

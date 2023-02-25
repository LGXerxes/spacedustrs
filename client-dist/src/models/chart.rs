/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://v2.api.spacetraders.io\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// Chart : The chart of a system or waypoint, which makes the location visible to other agents.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Chart {
    #[serde(rename = "waypointSymbol", skip_serializing_if = "Option::is_none")]
    pub waypoint_symbol: Option<String>,
    #[serde(rename = "submittedBy", skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    #[serde(rename = "submittedOn", skip_serializing_if = "Option::is_none")]
    pub submitted_on: Option<String>,
}

impl Chart {
    /// The chart of a system or waypoint, which makes the location visible to other agents.
    pub fn new() -> Chart {
        Chart {
            waypoint_symbol: None,
            submitted_by: None,
            submitted_on: None,
        }
    }
}



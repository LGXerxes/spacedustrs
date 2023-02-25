/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://v2.api.spacetraders.io\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// WaypointType : The type of waypoint.

/// The type of waypoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WaypointType {
    #[serde(rename = "PLANET")]
    Planet,
    #[serde(rename = "GAS_GIANT")]
    GasGiant,
    #[serde(rename = "MOON")]
    Moon,
    #[serde(rename = "ORBITAL_STATION")]
    OrbitalStation,
    #[serde(rename = "JUMP_GATE")]
    JumpGate,
    #[serde(rename = "ASTEROID_FIELD")]
    AsteroidField,
    #[serde(rename = "NEBULA")]
    Nebula,
    #[serde(rename = "DEBRIS_FIELD")]
    DebrisField,
    #[serde(rename = "GRAVITY_WELL")]
    GravityWell,
}

impl ToString for WaypointType {
    fn to_string(&self) -> String {
        match self {
            Self::Planet => String::from("PLANET"),
            Self::GasGiant => String::from("GAS_GIANT"),
            Self::Moon => String::from("MOON"),
            Self::OrbitalStation => String::from("ORBITAL_STATION"),
            Self::JumpGate => String::from("JUMP_GATE"),
            Self::AsteroidField => String::from("ASTEROID_FIELD"),
            Self::Nebula => String::from("NEBULA"),
            Self::DebrisField => String::from("DEBRIS_FIELD"),
            Self::GravityWell => String::from("GRAVITY_WELL"),
        }
    }
}

impl Default for WaypointType {
    fn default() -> WaypointType {
        Self::Planet
    }
}

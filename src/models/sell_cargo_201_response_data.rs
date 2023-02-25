/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://v2.api.spacetraders.io\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SellCargo201ResponseData {
    #[serde(rename = "agent")]
    pub agent: Box<crate::models::Agent>,
    #[serde(rename = "cargo")]
    pub cargo: Box<crate::models::ShipCargo>,
    #[serde(rename = "transaction")]
    pub transaction: Box<crate::models::MarketTransaction>,
}

impl SellCargo201ResponseData {
    pub fn new(
        agent: crate::models::Agent,
        cargo: crate::models::ShipCargo,
        transaction: crate::models::MarketTransaction,
    ) -> SellCargo201ResponseData {
        SellCargo201ResponseData {
            agent: Box::new(agent),
            cargo: Box::new(cargo),
            transaction: Box::new(transaction),
        }
    }
}

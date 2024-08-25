#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetZonesFilter {
    /// The account identifier to target for the resource.
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    /// The type of search to perform for the `name` value when querying the zone API. Available values: `contains`, `exact`. Defaults to `exact`.
    #[serde(rename = "lookupType")]
    pub r#lookup_type: Box<Option<String>>,
    /// A RE2 compatible regular expression to filter the	results. This is performed client side whereas the `name` and `lookup_type`	are performed on the Cloudflare server side.
    #[serde(rename = "match")]
    pub r#match: Box<Option<String>>,
    /// A string value to search for.
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Paused status of the zone to lookup. Defaults to `false`.
    #[serde(rename = "paused")]
    pub r#paused: Box<Option<bool>>,
    /// Status of the zone to lookup.
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetZonesFilter {
    /// The account identifier to target for the resource.
    #[builder(into, default)]
    #[serde(rename = "accountId")]
    pub r#account_id: Box<Option<String>>,
    /// The type of search to perform for the `name` value when querying the zone API. Available values: `contains`, `exact`. Defaults to `exact`.
    #[builder(into, default)]
    #[serde(rename = "lookupType")]
    pub r#lookup_type: Box<Option<String>>,
    /// A RE2 compatible regular expression to filter the	results. This is performed client side whereas the `name` and `lookup_type`	are performed on the Cloudflare server side.
    #[builder(into, default)]
    #[serde(rename = "match")]
    pub r#match_: Box<Option<String>>,
    /// A string value to search for.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Paused status of the zone to lookup. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "paused")]
    pub r#paused: Box<Option<bool>>,
    /// Status of the zone to lookup.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}

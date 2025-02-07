#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceApi {
    /// A list of Method objects; structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "methods")]
    pub r#methods: Box<Option<Vec<super::super::types::endpoints::ServiceApiMethod>>>,
    /// The simple name of the endpoint as described in the config.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// `SYNTAX_PROTO2` or `SYNTAX_PROTO3`.
    #[builder(into, default)]
    #[serde(rename = "syntax")]
    pub r#syntax: Box<Option<String>>,
    /// A version string for this api. If specified, will have the form major-version.minor-version, e.g. `1.10`.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}

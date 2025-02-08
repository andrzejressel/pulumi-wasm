#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SecurityPolicyAdvancedOptionsConfigJsonCustomConfig {
    /// A list of custom Content-Type header values to apply the JSON parsing. The
    /// format of the Content-Type header values is defined in
    /// [RFC 1341](https://www.ietf.org/rfc/rfc1341.txt). When configuring a custom Content-Type header
    /// value, only the type/subtype needs to be specified, and the parameters should be excluded.
    #[builder(into)]
    #[serde(rename = "contentTypes")]
    pub r#content_types: Box<Vec<String>>,
}

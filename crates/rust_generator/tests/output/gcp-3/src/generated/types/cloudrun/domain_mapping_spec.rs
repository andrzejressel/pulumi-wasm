#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DomainMappingSpec {
    /// The mode of the certificate.
    /// Default value is `AUTOMATIC`.
    /// Possible values are: `NONE`, `AUTOMATIC`.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "certificateMode")]
    pub r#certificate_mode: Box<Option<String>>,
    /// If set, the mapping will override any mapping set before this spec was set.
    /// It is recommended that the user leaves this empty to receive an error
    /// warning about a potential conflict and only set it once the respective UI
    /// has given such a warning.
    #[builder(into, default)]
    #[serde(rename = "forceOverride")]
    pub r#force_override: Box<Option<bool>>,
    /// The name of the Cloud Run Service that this DomainMapping applies to.
    /// The route must exist.
    #[builder(into)]
    #[serde(rename = "routeName")]
    pub r#route_name: Box<String>,
}

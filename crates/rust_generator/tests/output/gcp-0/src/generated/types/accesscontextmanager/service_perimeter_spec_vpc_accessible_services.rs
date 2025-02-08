#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServicePerimeterSpecVpcAccessibleServices {
    /// The list of APIs usable within the Service Perimeter.
    /// Must be empty unless `enableRestriction` is True.
    #[builder(into, default)]
    #[serde(rename = "allowedServices")]
    pub r#allowed_services: Box<Option<Vec<String>>>,
    /// Whether to restrict API calls within the Service Perimeter to the
    /// list of APIs specified in 'allowedServices'.
    #[builder(into, default)]
    #[serde(rename = "enableRestriction")]
    pub r#enable_restriction: Box<Option<bool>>,
}

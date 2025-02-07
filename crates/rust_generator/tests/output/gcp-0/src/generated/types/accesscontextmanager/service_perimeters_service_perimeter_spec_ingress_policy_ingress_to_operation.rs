#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServicePerimetersServicePerimeterSpecIngressPolicyIngressToOperation {
    /// API methods or permissions to allow. Method or permission must belong
    /// to the service specified by `serviceName` field. A single MethodSelector
    /// entry with `*` specified for the `method` field will allow all methods
    /// AND permissions for the service specified in `serviceName`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "methodSelectors")]
    pub r#method_selectors: Box<Option<Vec<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterSpecIngressPolicyIngressToOperationMethodSelector>>>,
    /// The name of the API whose methods or permissions the `IngressPolicy` or
    /// `EgressPolicy` want to allow. A single `ApiOperation` with serviceName
    /// field set to `*` will allow all methods AND permissions for all services.
    #[builder(into, default)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Box<Option<String>>,
}

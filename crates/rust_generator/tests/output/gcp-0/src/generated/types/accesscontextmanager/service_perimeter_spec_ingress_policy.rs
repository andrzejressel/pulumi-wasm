#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServicePerimeterSpecIngressPolicy {
    /// Defines the conditions on the source of a request causing this `IngressPolicy`
    /// to apply.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ingressFrom")]
    pub r#ingress_from: Box<Option<super::super::types::accesscontextmanager::ServicePerimeterSpecIngressPolicyIngressFrom>>,
    /// Defines the conditions on the `ApiOperation` and request destination that cause
    /// this `IngressPolicy` to apply.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ingressTo")]
    pub r#ingress_to: Box<Option<super::super::types::accesscontextmanager::ServicePerimeterSpecIngressPolicyIngressTo>>,
}

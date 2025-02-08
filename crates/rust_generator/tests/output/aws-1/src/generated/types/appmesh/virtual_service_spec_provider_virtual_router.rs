#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualServiceSpecProviderVirtualRouter {
    /// Name of the virtual router that is acting as a service provider. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "virtualRouterName")]
    pub r#virtual_router_name: Box<String>,
}

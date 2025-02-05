#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServicePerimeterSpecIngressPolicyIngressFromSource {
    /// An AccessLevel resource name that allows resources outside the ServicePerimeter to be accessed from the inside.
    #[builder(into, default)]
    #[serde(rename = "accessLevel")]
    pub r#access_level: Box<Option<String>>,
    /// A Google Cloud resource that is allowed to ingress the perimeter.
    /// Requests from these resources will be allowed to access perimeter data.
    /// Currently only projects are allowed. Format `projects/{project_number}`
    /// The project may be in any Google Cloud organization, not just the
    /// organization that the perimeter is defined in. `*` is not allowed, the case
    /// of allowing all Google Cloud resources only is not supported.
    #[builder(into, default)]
    #[serde(rename = "resource")]
    pub r#resource: Box<Option<String>>,
}

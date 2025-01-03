#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServicePerimeterIngressPolicyIngressFromSource {
    /// An `AccessLevel` resource name that allow resources within the
    /// `ServicePerimeters` to be accessed from the internet. `AccessLevels` listed
    /// must be in the same policy as this `ServicePerimeter`. Referencing a nonexistent
    /// `AccessLevel` will cause an error. If no `AccessLevel` names are listed,
    /// resources within the perimeter can only be accessed via Google Cloud calls
    /// with request origins within the perimeter.
    /// Example `accessPolicies/MY_POLICY/accessLevels/MY_LEVEL.`
    /// If * is specified, then all IngressSources will be allowed.
    #[builder(into, default)]
    #[serde(rename = "accessLevel")]
    pub r#access_level: Box<Option<String>>,
    /// A Google Cloud resource that is allowed to ingress the perimeter.
    /// Requests from these resources will be allowed to access perimeter data.
    /// Currently only projects and VPCs are allowed.
    /// Project format: `projects/{projectNumber}`
    /// VPC network format:
    /// `//compute.googleapis.com/projects/{PROJECT_ID}/global/networks/{NAME}`.
    /// The project may be in any Google Cloud organization, not just the
    /// organization that the perimeter is defined in. `*` is not allowed, the case
    /// of allowing all Google Cloud resources only is not supported.
    #[builder(into, default)]
    #[serde(rename = "resource")]
    pub r#resource: Box<Option<String>>,
}

#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServicePerimeterIngressPolicyIngressFrom {
    /// Identities can be an individual user, service account, Google group,
    /// or third-party identity. For third-party identity, only single identities
    /// are supported and other identity types are not supported.The v1 identities
    /// that have the prefix user, group and serviceAccount in
    /// https://cloud.google.com/iam/docs/principal-identifiers#v1 are supported.
    #[builder(into, default)]
    #[serde(rename = "identities")]
    pub r#identities: Box<Option<Vec<String>>>,
    /// Specifies the type of identities that are allowed access from outside the
    /// perimeter. If left unspecified, then members of `identities` field will be
    /// allowed access.
    /// Possible values are: `ANY_IDENTITY`, `ANY_USER_ACCOUNT`, `ANY_SERVICE_ACCOUNT`.
    #[builder(into, default)]
    #[serde(rename = "identityType")]
    pub r#identity_type: Box<Option<String>>,
    /// Sources that this `IngressPolicy` authorizes access from.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sources")]
    pub r#sources: Box<Option<Vec<super::super::types::accesscontextmanager::ServicePerimeterIngressPolicyIngressFromSource>>>,
}

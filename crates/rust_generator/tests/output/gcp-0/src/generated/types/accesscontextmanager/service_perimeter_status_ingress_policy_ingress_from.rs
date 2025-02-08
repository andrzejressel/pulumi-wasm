#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServicePerimeterStatusIngressPolicyIngressFrom {
    /// A list of identities that are allowed access through this ingress policy.
    /// Should be in the format of email address. The email address should represent
    /// individual user or service account only.
    #[builder(into, default)]
    #[serde(rename = "identities")]
    pub r#identities: Box<Option<Vec<String>>>,
    /// Specifies the type of identities that are allowed access from outside the
    /// perimeter. If left unspecified, then members of `identities` field will be
    /// allowed access.
    /// Possible values are: `IDENTITY_TYPE_UNSPECIFIED`, `ANY_IDENTITY`, `ANY_USER_ACCOUNT`, `ANY_SERVICE_ACCOUNT`.
    #[builder(into, default)]
    #[serde(rename = "identityType")]
    pub r#identity_type: Box<Option<String>>,
    /// Sources that this `IngressPolicy` authorizes access from.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sources")]
    pub r#sources: Box<Option<Vec<super::super::types::accesscontextmanager::ServicePerimeterStatusIngressPolicyIngressFromSource>>>,
}

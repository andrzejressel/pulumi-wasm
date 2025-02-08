#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServicePerimetersServicePerimeterSpecEgressPolicyEgressTo {
    /// A list of external resources that are allowed to be accessed. A request
    /// matches if it contains an external resource in this list (Example:
    /// s3://bucket/path). Currently '*' is not allowed.
    #[builder(into, default)]
    #[serde(rename = "externalResources")]
    pub r#external_resources: Box<Option<Vec<String>>>,
    /// A list of `ApiOperations` that this egress rule applies to. A request matches
    /// if it contains an operation/service in this list.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "operations")]
    pub r#operations: Box<Option<Vec<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterSpecEgressPolicyEgressToOperation>>>,
    /// A list of resources, currently only projects in the form
    /// `projects/<projectnumber>`, that match this to stanza. A request matches
    /// if it contains a resource in this list. If * is specified for resources,
    /// then this `EgressTo` rule will authorize access to all resources outside
    /// the perimeter.
    #[builder(into, default)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<Vec<String>>>,
}

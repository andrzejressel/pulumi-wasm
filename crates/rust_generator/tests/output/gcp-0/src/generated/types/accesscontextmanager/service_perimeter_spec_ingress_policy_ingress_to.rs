#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServicePerimeterSpecIngressPolicyIngressTo {
    /// A list of `ApiOperations` the sources specified in corresponding `IngressFrom`
    /// are allowed to perform in this `ServicePerimeter`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "operations")]
    pub r#operations: Box<Option<Vec<super::super::types::accesscontextmanager::ServicePerimeterSpecIngressPolicyIngressToOperation>>>,
    /// A list of resources, currently only projects in the form
    /// `projects/<projectnumber>`, protected by this `ServicePerimeter`
    /// that are allowed to be accessed by sources defined in the
    /// corresponding `IngressFrom`. A request matches if it contains
    /// a resource in this list. If `*` is specified for resources,
    /// then this `IngressTo` rule will authorize access to all
    /// resources inside the perimeter, provided that the request
    /// also matches the `operations` field.
    #[builder(into, default)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<Vec<String>>>,
}

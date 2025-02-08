#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServicePerimetersServicePerimeter {
    /// (Output)
    /// Time the AccessPolicy was created in UTC.
    #[builder(into, default)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<Option<String>>,
    /// Description of the ServicePerimeter and its use. Does not affect
    /// behavior.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Resource name for the ServicePerimeter. The short_name component must
    /// begin with a letter and only include alphanumeric and '_'.
    /// Format: accessPolicies/{policy_id}/servicePerimeters/{short_name}
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the type of the Perimeter. There are two types: regular and
    /// bridge. Regular Service Perimeter contains resources, access levels,
    /// and restricted services. Every resource can be in at most
    /// ONE regular Service Perimeter.
    /// In addition to being in a regular service perimeter, a resource can also
    /// be in zero or more perimeter bridges. A perimeter bridge only contains
    /// resources. Cross project operations are permitted if all effected
    /// resources share some perimeter (whether bridge or regular). Perimeter
    /// Bridge does not contain access levels or services: those are governed
    /// entirely by the regular perimeter that resource is in.
    /// Perimeter Bridges are typically useful when building more complex
    /// topologies with many independent perimeters that need to share some data
    /// with a common perimeter, but should not be able to share data among
    /// themselves.
    /// Default value is `PERIMETER_TYPE_REGULAR`.
    /// Possible values are: `PERIMETER_TYPE_REGULAR`, `PERIMETER_TYPE_BRIDGE`.
    #[builder(into, default)]
    #[serde(rename = "perimeterType")]
    pub r#perimeter_type: Box<Option<String>>,
    /// Proposed (or dry run) ServicePerimeter configuration.
    /// This configuration allows to specify and test ServicePerimeter configuration
    /// without enforcing actual access restrictions. Only allowed to be set when
    /// the `useExplicitDryRunSpec` flag is set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "spec")]
    pub r#spec: Box<Option<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterSpec>>,
    /// ServicePerimeter configuration. Specifies sets of resources,
    /// restricted services and access levels that determine
    /// perimeter content and boundaries.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterStatus>>,
    /// Human readable title. Must be unique within the Policy.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Box<String>,
    /// (Output)
    /// Time the AccessPolicy was updated in UTC.
    #[builder(into, default)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<Option<String>>,
    /// Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists
    /// for all Service Perimeters, and that spec is identical to the status for those
    /// Service Perimeters. When this flag is set, it inhibits the generation of the
    /// implicit spec, thereby allowing the user to explicitly provide a
    /// configuration ("spec") to use in a dry-run version of the Service Perimeter.
    /// This allows the user to test changes to the enforced config ("status") without
    /// actually enforcing them. This testing is done through analyzing the differences
    /// between currently enforced and suggested restrictions. useExplicitDryRunSpec must
    /// bet set to True if any of the fields in the spec are set to non-default values.
    #[builder(into, default)]
    #[serde(rename = "useExplicitDryRunSpec")]
    pub r#use_explicit_dry_run_spec: Box<Option<bool>>,
}

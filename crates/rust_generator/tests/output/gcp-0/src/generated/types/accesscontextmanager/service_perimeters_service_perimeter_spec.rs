#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServicePerimetersServicePerimeterSpec {
    /// A list of AccessLevel resource names that allow resources within
    /// the ServicePerimeter to be accessed from the internet.
    /// AccessLevels listed must be in the same policy as this
    /// ServicePerimeter. Referencing a nonexistent AccessLevel is a
    /// syntax error. If no AccessLevel names are listed, resources within
    /// the perimeter can only be accessed via GCP calls with request
    /// origins within the perimeter. For Service Perimeter Bridge, must
    /// be empty.
    /// Format: accessPolicies/{policy_id}/accessLevels/{access_level_name}
    #[builder(into, default)]
    #[serde(rename = "accessLevels")]
    pub r#access_levels: Box<Option<Vec<String>>>,
    /// List of EgressPolicies to apply to the perimeter. A perimeter may
    /// have multiple EgressPolicies, each of which is evaluated separately.
    /// Access is granted if any EgressPolicy grants it. Must be empty for
    /// a perimeter bridge.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "egressPolicies")]
    pub r#egress_policies: Box<Option<Vec<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterSpecEgressPolicy>>>,
    /// List of `IngressPolicies` to apply to the perimeter. A perimeter may
    /// have multiple `IngressPolicies`, each of which is evaluated
    /// separately. Access is granted if any `Ingress Policy` grants it.
    /// Must be empty for a perimeter bridge.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ingressPolicies")]
    pub r#ingress_policies: Box<Option<Vec<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterSpecIngressPolicy>>>,
    /// A list of GCP resources that are inside of the service perimeter.
    /// Currently only projects are allowed.
    /// Format: projects/{project_number}
    #[builder(into, default)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<Vec<String>>>,
    /// GCP services that are subject to the Service Perimeter
    /// restrictions. Must contain a list of services. For example, if
    /// `storage.googleapis.com` is specified, access to the storage
    /// buckets inside the perimeter must meet the perimeter's access
    /// restrictions.
    #[builder(into, default)]
    #[serde(rename = "restrictedServices")]
    pub r#restricted_services: Box<Option<Vec<String>>>,
    /// Specifies how APIs are allowed to communicate within the Service
    /// Perimeter.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "vpcAccessibleServices")]
    pub r#vpc_accessible_services: Box<Option<super::super::types::accesscontextmanager::ServicePerimetersServicePerimeterSpecVpcAccessibleServices>>,
}

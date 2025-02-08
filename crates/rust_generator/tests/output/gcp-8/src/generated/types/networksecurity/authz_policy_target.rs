#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AuthzPolicyTarget {
    /// All gateways and forwarding rules referenced by this policy and extensions must share the same load balancing scheme.
    /// For more information, refer to [Backend services overview](https://cloud.google.com/load-balancing/docs/backend-service).
    /// Possible values are: `INTERNAL_MANAGED`, `EXTERNAL_MANAGED`, `INTERNAL_SELF_MANAGED`.
    #[builder(into)]
    #[serde(rename = "loadBalancingScheme")]
    pub r#load_balancing_scheme: Box<String>,
    /// A list of references to the Forwarding Rules on which this policy will be applied.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "resources")]
    pub r#resources: Box<Option<Vec<String>>>,
}

#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterFleet {
    /// The resource name of the fleet Membership resource associated to this cluster with format `//gkehub.googleapis.com/projects/{{project}}/locations/{{location}}/memberships/{{name}}`. See the official doc for [fleet management](https://cloud.google.com/kubernetes-engine/docs/fleets-overview).
    #[builder(into, default)]
    #[serde(rename = "membership")]
    pub r#membership: Box<Option<String>>,
    /// The short name of the fleet membership, extracted from `fleet.0.membership`. You can use this field to configure `membership_id` under google_gkehub_feature_membership.
    #[builder(into, default)]
    #[serde(rename = "membershipId")]
    pub r#membership_id: Box<Option<String>>,
    /// The location of the fleet membership,  extracted from `fleet.0.membership`. You can use this field to configure `membership_location` under google_gkehub_feature_membership.
    #[builder(into, default)]
    #[serde(rename = "membershipLocation")]
    pub r#membership_location: Box<Option<String>>,
    /// Whether the cluster has been registered via the fleet API.
    #[builder(into, default)]
    #[serde(rename = "preRegistered")]
    pub r#pre_registered: Box<Option<bool>>,
    /// The name of the Fleet host project where this cluster will be registered.
    #[builder(into, default)]
    #[serde(rename = "project")]
    pub r#project: Box<Option<String>>,
}

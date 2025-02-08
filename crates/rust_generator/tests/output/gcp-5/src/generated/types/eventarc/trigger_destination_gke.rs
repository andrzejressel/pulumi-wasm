#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TriggerDestinationGke {
    /// Required. The name of the cluster the GKE service is running in. The cluster must be running in the same project as the trigger being created.
    #[builder(into)]
    #[serde(rename = "cluster")]
    pub r#cluster: Box<String>,
    /// Required. The name of the Google Compute Engine in which the cluster resides, which can either be compute zone (for example, us-central1-a) for the zonal clusters or region (for example, us-central1) for regional clusters.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// Required. The namespace the GKE service is running in.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<String>,
    /// Optional. The relative path on the GKE service the events should be sent to. The value must conform to the definition of a URI path segment (section 3.3 of RFC2396). Examples: "/route", "route", "route/subroute".
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Required. Name of the GKE service.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}

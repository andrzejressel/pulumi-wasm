#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceStatus {
    /// Array of observed Service Conditions, indicating the current ready state of the service.
    #[builder(into)]
    #[serde(rename = "conditions")]
    pub r#conditions: Box<Vec<super::super::types::cloudrun::GetServiceStatusCondition>>,
    /// From ConfigurationStatus. LatestCreatedRevisionName is the last revision that was created
    /// from this Service's Configuration. It might not be ready yet, for that use
    /// LatestReadyRevisionName.
    #[builder(into)]
    #[serde(rename = "latestCreatedRevisionName")]
    pub r#latest_created_revision_name: Box<String>,
    /// From ConfigurationStatus. LatestReadyRevisionName holds the name of the latest Revision
    /// stamped out from this Service's Configuration that has had its "Ready" condition become
    /// "True".
    #[builder(into)]
    #[serde(rename = "latestReadyRevisionName")]
    pub r#latest_ready_revision_name: Box<String>,
    /// ObservedGeneration is the 'Generation' of the Route that was last processed by the
    /// controller.
    /// 
    /// Clients polling for completed reconciliation should poll until observedGeneration =
    /// metadata.generation and the Ready condition's status is True or False.
    #[builder(into)]
    #[serde(rename = "observedGeneration")]
    pub r#observed_generation: Box<i32>,
    /// Traffic specifies how to distribute traffic over a collection of Knative Revisions
    /// and Configurations
    #[builder(into)]
    #[serde(rename = "traffics")]
    pub r#traffics: Box<Vec<super::super::types::cloudrun::GetServiceStatusTraffic>>,
    /// From RouteStatus. URL holds the url that will distribute traffic over the provided traffic
    /// targets. It generally has the form
    /// https://{route-hash}-{project-hash}-{cluster-level-suffix}.a.run.app
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}

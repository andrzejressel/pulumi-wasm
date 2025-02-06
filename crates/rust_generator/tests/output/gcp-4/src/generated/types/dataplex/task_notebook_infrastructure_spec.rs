#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TaskNotebookInfrastructureSpec {
    /// Compute resources needed for a Task when using Dataproc Serverless.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "batch")]
    pub r#batch: Box<Option<super::super::types::dataplex::TaskNotebookInfrastructureSpecBatch>>,
    /// Container Image Runtime Configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "containerImage")]
    pub r#container_image: Box<Option<super::super::types::dataplex::TaskNotebookInfrastructureSpecContainerImage>>,
    /// Vpc network.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "vpcNetwork")]
    pub r#vpc_network: Box<Option<super::super::types::dataplex::TaskNotebookInfrastructureSpecVpcNetwork>>,
}

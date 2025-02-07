#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResolverSyncConfig {
    /// Conflict Detection strategy to use. Valid values are `NONE` and `VERSION`.
    #[builder(into, default)]
    #[serde(rename = "conflictDetection")]
    pub r#conflict_detection: Box<Option<String>>,
    /// Conflict Resolution strategy to perform in the event of a conflict. Valid values are `NONE`, `OPTIMISTIC_CONCURRENCY`, `AUTOMERGE`, and `LAMBDA`.
    #[builder(into, default)]
    #[serde(rename = "conflictHandler")]
    pub r#conflict_handler: Box<Option<String>>,
    /// Lambda Conflict Handler Config when configuring `LAMBDA` as the Conflict Handler. See Lambda Conflict Handler Config.
    #[builder(into, default)]
    #[serde(rename = "lambdaConflictHandlerConfig")]
    pub r#lambda_conflict_handler_config: Box<Option<super::super::types::appsync::ResolverSyncConfigLambdaConflictHandlerConfig>>,
}

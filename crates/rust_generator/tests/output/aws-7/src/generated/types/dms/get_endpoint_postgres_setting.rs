#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEndpointPostgresSetting {
    #[builder(into)]
    #[serde(rename = "afterConnectScript")]
    pub r#after_connect_script: Box<String>,
    #[builder(into)]
    #[serde(rename = "babelfishDatabaseName")]
    pub r#babelfish_database_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "captureDdls")]
    pub r#capture_ddls: Box<bool>,
    #[builder(into)]
    #[serde(rename = "databaseMode")]
    pub r#database_mode: Box<String>,
    #[builder(into)]
    #[serde(rename = "ddlArtifactsSchema")]
    pub r#ddl_artifacts_schema: Box<String>,
    #[builder(into)]
    #[serde(rename = "executeTimeout")]
    pub r#execute_timeout: Box<i32>,
    #[builder(into)]
    #[serde(rename = "failTasksOnLobTruncation")]
    pub r#fail_tasks_on_lob_truncation: Box<bool>,
    #[builder(into)]
    #[serde(rename = "heartbeatEnable")]
    pub r#heartbeat_enable: Box<bool>,
    #[builder(into)]
    #[serde(rename = "heartbeatFrequency")]
    pub r#heartbeat_frequency: Box<i32>,
    #[builder(into)]
    #[serde(rename = "heartbeatSchema")]
    pub r#heartbeat_schema: Box<String>,
    #[builder(into)]
    #[serde(rename = "mapBooleanAsBoolean")]
    pub r#map_boolean_as_boolean: Box<bool>,
    #[builder(into)]
    #[serde(rename = "mapJsonbAsClob")]
    pub r#map_jsonb_as_clob: Box<bool>,
    #[builder(into)]
    #[serde(rename = "mapLongVarcharAs")]
    pub r#map_long_varchar_as: Box<String>,
    #[builder(into)]
    #[serde(rename = "maxFileSize")]
    pub r#max_file_size: Box<i32>,
    #[builder(into)]
    #[serde(rename = "pluginName")]
    pub r#plugin_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "slotName")]
    pub r#slot_name: Box<String>,
}

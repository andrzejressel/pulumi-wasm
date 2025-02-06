#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowTask {
    /// Operation to be performed on the provided source fields. See Connector Operator for details.
    #[builder(into, default)]
    #[serde(rename = "connectorOperators")]
    pub r#connector_operators: Box<Option<Vec<super::super::types::appflow::FlowTaskConnectorOperator>>>,
    /// Field in a destination connector, or a field value against which Amazon AppFlow validates a source field.
    #[builder(into, default)]
    #[serde(rename = "destinationField")]
    pub r#destination_field: Box<Option<String>>,
    /// Source fields to which a particular task is applied.
    #[builder(into, default)]
    #[serde(rename = "sourceFields")]
    pub r#source_fields: Box<Option<Vec<String>>>,
    /// Map used to store task-related information. The execution service looks for particular information based on the `TaskType`. Valid keys are `VALUE`, `VALUES`, `DATA_TYPE`, `UPPER_BOUND`, `LOWER_BOUND`, `SOURCE_DATA_TYPE`, `DESTINATION_DATA_TYPE`, `VALIDATION_ACTION`, `MASK_VALUE`, `MASK_LENGTH`, `TRUNCATE_LENGTH`, `MATH_OPERATION_FIELDS_ORDER`, `CONCAT_FORMAT`, `SUBFIELD_CATEGORY_MAP`, and `EXCLUDE_SOURCE_FIELDS_LIST`.
    #[builder(into, default)]
    #[serde(rename = "taskProperties")]
    pub r#task_properties: Box<Option<std::collections::HashMap<String, String>>>,
    /// Particular task implementation that Amazon AppFlow performs. Valid values are `Arithmetic`, `Filter`, `Map`, `Map_all`, `Mask`, `Merge`, `Passthrough`, `Truncate`, and `Validate`.
    #[builder(into)]
    #[serde(rename = "taskType")]
    pub r#task_type: Box<String>,
}

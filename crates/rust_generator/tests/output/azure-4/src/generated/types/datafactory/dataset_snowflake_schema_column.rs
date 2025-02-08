#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatasetSnowflakeSchemaColumn {
    /// The name of the column.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The total number of digits allowed.
    #[builder(into, default)]
    #[serde(rename = "precision")]
    pub r#precision: Box<Option<i32>>,
    /// The number of digits allowed to the right of the decimal point.
    #[builder(into, default)]
    #[serde(rename = "scale")]
    pub r#scale: Box<Option<i32>>,
    /// Type of the column. Valid values are `NUMBER`, `DECIMAL`, `NUMERIC`, `INT`, `INTEGER`, `BIGINT`, `SMALLINT`, `FLOAT``FLOAT4`, `FLOAT8`, `DOUBLE`, `DOUBLE PRECISION`, `REAL`, `VARCHAR`, `CHAR`, `CHARACTER`, `STRING`, `TEXT`, `BINARY`, `VARBINARY`, `BOOLEAN`, `DATE`, `DATETIME`, `TIME`, `TIMESTAMP`, `TIMESTAMP_LTZ`, `TIMESTAMP_NTZ`, `TIMESTAMP_TZ`, `VARIANT`, `OBJECT`, `ARRAY`, `GEOGRAPHY`. Please note these values are case sensitive.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}

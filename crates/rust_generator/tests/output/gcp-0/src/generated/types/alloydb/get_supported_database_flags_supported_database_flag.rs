#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSupportedDatabaseFlagsSupportedDatabaseFlag {
    /// Whether the database flag accepts multiple values. If true, a comma-separated list of stringified values may be specified.
    #[builder(into)]
    #[serde(rename = "acceptsMultipleValues")]
    pub r#accepts_multiple_values: Box<bool>,
    /// The name of the database flag, e.g. "max_allowed_packets". The is a possibly key for the Instance.database_flags map field.
    #[builder(into)]
    #[serde(rename = "flagName")]
    pub r#flag_name: Box<String>,
    /// Restriction on `INTEGER` type value. Specifies the minimum value and the maximum value that can be specified, if applicable.
    #[builder(into)]
    #[serde(rename = "integerRestrictions")]
    pub r#integer_restrictions: Box<super::super::types::alloydb::GetSupportedDatabaseFlagsSupportedDatabaseFlagIntegerRestrictions>,
    /// The name of the flag resource, following Google Cloud conventions, e.g.: * projects/{project}/locations/{location}/flags/{flag} This field currently has no semantic meaning.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Whether setting or updating this flag on an Instance requires a database restart. If a flag that requires database restart is set, the backend will automatically restart the database (making sure to satisfy any availability SLO's).
    #[builder(into)]
    #[serde(rename = "requiresDbRestart")]
    pub r#requires_db_restart: Box<bool>,
    /// Restriction on `STRING` type value. The list of allowed values, if bounded. This field will be empty if there is a unbounded number of allowed values.
    #[builder(into)]
    #[serde(rename = "stringRestrictions")]
    pub r#string_restrictions: Box<super::super::types::alloydb::GetSupportedDatabaseFlagsSupportedDatabaseFlagStringRestrictions>,
    /// Major database engine versions for which this flag is supported. The supported values are `POSTGRES_14` and `DATABASE_VERSION_UNSPECIFIED`.
    #[builder(into)]
    #[serde(rename = "supportedDbVersions")]
    pub r#supported_db_versions: Box<Vec<String>>,
    /// ValueType describes the semantic type of the value that the flag accepts. Regardless of the ValueType, the Instance.database_flags field accepts the stringified version of the value, i.e. "20" or "3.14". The supported values are `VALUE_TYPE_UNSPECIFIED`, `STRING`, `INTEGER`, `FLOAT` and `NONE`.
    #[builder(into)]
    #[serde(rename = "valueType")]
    pub r#value_type: Box<String>,
}

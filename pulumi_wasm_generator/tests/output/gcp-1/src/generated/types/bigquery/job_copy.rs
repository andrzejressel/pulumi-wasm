#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobCopy {
    /// Specifies whether the job is allowed to create new tables. The following values are supported:
    /// CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table.
    /// CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result.
    /// Creation, truncation and append actions occur as one atomic update upon job completion
    /// Default value is `CREATE_IF_NEEDED`.
    /// Possible values are: `CREATE_IF_NEEDED`, `CREATE_NEVER`.
    #[builder(into, default)]
    #[serde(rename = "createDisposition")]
    pub r#create_disposition: Box<Option<String>>,
    /// Custom encryption configuration (e.g., Cloud KMS keys)
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "destinationEncryptionConfiguration")]
    pub r#destination_encryption_configuration: Box<Option<super::super::types::bigquery::JobCopyDestinationEncryptionConfiguration>>,
    /// The destination table.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "destinationTable")]
    pub r#destination_table: Box<Option<super::super::types::bigquery::JobCopyDestinationTable>>,
    /// Source tables to copy.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sourceTables")]
    pub r#source_tables: Box<Vec<super::super::types::bigquery::JobCopySourceTable>>,
    /// Specifies the action that occurs if the destination table already exists. The following values are supported:
    /// WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data and uses the schema from the query result.
    /// WRITE_APPEND: If the table already exists, BigQuery appends the data to the table.
    /// WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result.
    /// Each action is atomic and only occurs if BigQuery is able to complete the job successfully.
    /// Creation, truncation and append actions occur as one atomic update upon job completion.
    /// Default value is `WRITE_EMPTY`.
    /// Possible values are: `WRITE_TRUNCATE`, `WRITE_APPEND`, `WRITE_EMPTY`.
    #[builder(into, default)]
    #[serde(rename = "writeDisposition")]
    pub r#write_disposition: Box<Option<String>>,
}

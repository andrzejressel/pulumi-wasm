#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDatabaseInstanceClone {
    /// The name of the allocated ip range for the private ip CloudSQL instance. For example: "google-managed-services-default". If set, the cloned instance ip will be created in the allocated range. The range name must comply with [RFC 1035](https://tools.ietf.org/html/rfc1035). Specifically, the name must be 1-63 characters long and match the regular expression a-z?.
    #[builder(into)]
    #[serde(rename = "allocatedIpRange")]
    pub r#allocated_ip_range: Box<String>,
    /// (SQL Server only, use with point_in_time) clone only the specified databases from the source instance. Clone all databases if empty.
    #[builder(into)]
    #[serde(rename = "databaseNames")]
    pub r#database_names: Box<Vec<String>>,
    /// The timestamp of the point in time that should be restored.
    #[builder(into)]
    #[serde(rename = "pointInTime")]
    pub r#point_in_time: Box<String>,
    /// (Point-in-time recovery for PostgreSQL only) Clone to an instance in the specified zone. If no zone is specified, clone to the same zone as the source instance.
    #[builder(into)]
    #[serde(rename = "preferredZone")]
    pub r#preferred_zone: Box<String>,
    /// The name of the instance from which the point in time should be restored.
    #[builder(into)]
    #[serde(rename = "sourceInstanceName")]
    pub r#source_instance_name: Box<String>,
}

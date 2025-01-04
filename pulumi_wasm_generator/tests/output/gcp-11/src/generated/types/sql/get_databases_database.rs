#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDatabasesDatabase {
    /// The charset value. See MySQL's
    /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
    /// and Postgres' [Character Set Support](https://www.postgresql.org/docs/9.6/static/multibyte.html)
    /// for more details and supported values. Postgres databases only support
    /// a value of 'UTF8' at creation time.
    #[builder(into)]
    #[serde(rename = "charset")]
    pub r#charset: Box<String>,
    /// The collation value. See MySQL's
    /// [Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)
    /// and Postgres' [Collation Support](https://www.postgresql.org/docs/9.6/static/collation.html)
    /// for more details and supported values. Postgres databases only support
    /// a value of 'en_US.UTF8' at creation time.
    #[builder(into)]
    #[serde(rename = "collation")]
    pub r#collation: Box<String>,
    /// The deletion policy for the database. Setting ABANDON allows the resource
    /// to be abandoned rather than deleted. This is useful for Postgres, where databases cannot be
    /// deleted from the API if there are users other than cloudsqlsuperuser with access. Possible
    /// values are: "ABANDON", "DELETE". Defaults to "DELETE".
    #[builder(into)]
    #[serde(rename = "deletionPolicy")]
    pub r#deletion_policy: Box<String>,
    /// The name of the Cloud SQL database instance in which the database belongs.
    #[builder(into)]
    #[serde(rename = "instance")]
    pub r#instance: Box<String>,
    /// The name of the database in the Cloud SQL instance.
    /// This does not include the project ID or instance name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID of the project in which the instance belongs.
    /// 
    /// > **Note** This datasource performs client-side sorting to provide consistent ordering of the databases.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
    #[builder(into)]
    #[serde(rename = "selfLink")]
    pub r#self_link: Box<String>,
}

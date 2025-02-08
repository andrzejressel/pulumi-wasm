#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAutonomousDatabasesAutonomousDatabase {
    /// The password for the default ADMIN user.
    #[builder(into)]
    #[serde(rename = "adminPassword")]
    pub r#admin_password: Box<String>,
    /// The ID of the Autonomous Database to create. This value is restricted
    /// to (^a-z?$) and must be a maximum of 63
    /// characters in length. The value must start with a letter and end with
    /// a letter or a number.
    #[builder(into)]
    #[serde(rename = "autonomousDatabaseId")]
    pub r#autonomous_database_id: Box<String>,
    /// The subnet CIDR range for the Autonmous Database.
    #[builder(into)]
    #[serde(rename = "cidr")]
    pub r#cidr: Box<String>,
    /// The date and time that the Autonomous Database was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Box<String>,
    /// The name of the Autonomous Database. The database name must be unique in
    /// the project. The name must begin with a letter and can
    /// contain a maximum of 30 alphanumeric characters.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: Box<String>,
    #[builder(into)]
    #[serde(rename = "deletionProtection")]
    pub r#deletion_protection: Box<bool>,
    /// The display name for the Autonomous Database. The name does not have to
    /// be unique within your project.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: Box<std::collections::HashMap<String, String>>,
    /// The ID of the subscription entitlement associated with the Autonomous
    /// Database.
    #[builder(into)]
    #[serde(rename = "entitlementId")]
    pub r#entitlement_id: Box<String>,
    /// The labels or tags associated with the Autonomous Database. 
    /// 
    /// **Note**: This field is non-authoritative, and will only manage the labels present in your configuration.
    /// Please refer to the field 'effective_labels' for all of the labels present on the resource.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// The location of the resource.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// Identifier. The name of the Autonomous Database resource in the following format:
    /// projects/{project}/locations/{region}/autonomousDatabases/{autonomous_database}
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the VPC network used by the Autonomous Database.
    /// Format: projects/{project}/global/networks/{network}
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
    /// The project to which the resource belongs. If it
    /// is not provided, the provider project is used.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: Box<String>,
    /// The properties of an Autonomous Database.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Box<Vec<super::super::types::oracledatabase::GetAutonomousDatabasesAutonomousDatabaseProperty>>,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: Box<std::collections::HashMap<String, String>>,
}

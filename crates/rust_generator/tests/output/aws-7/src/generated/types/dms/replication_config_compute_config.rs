#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicationConfigComputeConfig {
    /// The Availability Zone where the DMS Serverless replication using this configuration will run. The default value is a random.
    #[builder(into, default)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Box<Option<String>>,
    /// A list of custom DNS name servers supported for the DMS Serverless replication to access your source or target database.
    #[builder(into, default)]
    #[serde(rename = "dnsNameServers")]
    pub r#dns_name_servers: Box<Option<String>>,
    /// An Key Management Service (KMS) key Amazon Resource Name (ARN) that is used to encrypt the data during DMS Serverless replication. If you don't specify a value for the KmsKeyId parameter, DMS uses your default encryption key.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Specifies the maximum value of the DMS capacity units (DCUs) for which a given DMS Serverless replication can be provisioned. A single DCU is 2GB of RAM, with 2 DCUs as the minimum value allowed. The list of valid DCU values includes 2, 4, 8, 16, 32, 64, 128, 192, 256, and 384.
    #[builder(into, default)]
    #[serde(rename = "maxCapacityUnits")]
    pub r#max_capacity_units: Box<Option<i32>>,
    /// Specifies the minimum value of the DMS capacity units (DCUs) for which a given DMS Serverless replication can be provisioned. The list of valid DCU values includes 2, 4, 8, 16, 32, 64, 128, 192, 256, and 384. If this value isn't set DMS scans the current activity of available source tables to identify an optimum setting for this parameter.
    #[builder(into, default)]
    #[serde(rename = "minCapacityUnits")]
    pub r#min_capacity_units: Box<Option<i32>>,
    /// Specifies if the replication instance is a multi-az deployment. You cannot set the `availability_zone` parameter if the `multi_az` parameter is set to `true`.
    #[builder(into, default)]
    #[serde(rename = "multiAz")]
    pub r#multi_az: Box<Option<bool>>,
    /// The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).
    /// 
    /// - Default: A 30-minute window selected at random from an 8-hour block of time per region, occurring on a random day of the week.
    /// - Format: `ddd:hh24:mi-ddd:hh24:mi`
    /// - Valid Days: `mon, tue, wed, thu, fri, sat, sun`
    /// - Constraints: Minimum 30-minute window.
    #[builder(into, default)]
    #[serde(rename = "preferredMaintenanceWindow")]
    pub r#preferred_maintenance_window: Box<Option<String>>,
    /// Specifies a subnet group identifier to associate with the DMS Serverless replication.
    #[builder(into)]
    #[serde(rename = "replicationSubnetGroupId")]
    pub r#replication_subnet_group_id: Box<String>,
    /// Specifies the virtual private cloud (VPC) security group to use with the DMS Serverless replication. The VPC security group must work with the VPC containing the replication.
    #[builder(into, default)]
    #[serde(rename = "vpcSecurityGroupIds")]
    pub r#vpc_security_group_ids: Box<Option<Vec<String>>>,
}

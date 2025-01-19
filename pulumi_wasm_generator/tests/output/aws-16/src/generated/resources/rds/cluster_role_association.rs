/// Manages a RDS DB Cluster association with an IAM Role. Example use cases:
///
/// * [Creating an IAM Role to Allow Amazon Aurora to Access AWS Services](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/AuroraMySQL.Integrating.Authorizing.IAM.CreateRole.html)
/// * [Importing Amazon S3 Data into an RDS PostgreSQL DB Cluster](https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_PostgreSQL.S3Import.html)
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = cluster_role_association::create(
///         "example",
///         ClusterRoleAssociationArgs::builder()
///             .db_cluster_identifier("${exampleAwsRdsCluster.id}")
///             .feature_name("S3_INTEGRATION")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_rds_cluster_role_association` using the DB Cluster Identifier and IAM Role ARN separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:rds/clusterRoleAssociation:ClusterRoleAssociation example my-db-cluster,arn:aws:iam::123456789012:role/my-role
/// ```
pub mod cluster_role_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterRoleAssociationArgs {
        /// DB Cluster Identifier to associate with the IAM Role.
        #[builder(into)]
        pub db_cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// Name of the feature for association. This can be found in the AWS documentation relevant to the integration or a full list is available in the `SupportedFeatureNames` list returned by [AWS CLI rds describe-db-engine-versions](https://docs.aws.amazon.com/cli/latest/reference/rds/describe-db-engine-versions.html).
        #[builder(into)]
        pub feature_name: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM Role to associate with the DB Cluster.
        #[builder(into)]
        pub role_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterRoleAssociationResult {
        /// DB Cluster Identifier to associate with the IAM Role.
        pub db_cluster_identifier: pulumi_wasm_rust::Output<String>,
        /// Name of the feature for association. This can be found in the AWS documentation relevant to the integration or a full list is available in the `SupportedFeatureNames` list returned by [AWS CLI rds describe-db-engine-versions](https://docs.aws.amazon.com/cli/latest/reference/rds/describe-db-engine-versions.html).
        pub feature_name: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the IAM Role to associate with the DB Cluster.
        pub role_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ClusterRoleAssociationArgs,
    ) -> ClusterRoleAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let db_cluster_identifier_binding = args.db_cluster_identifier.get_inner();
        let feature_name_binding = args.feature_name.get_inner();
        let role_arn_binding = args.role_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/clusterRoleAssociation:ClusterRoleAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "dbClusterIdentifier".into(),
                    value: &db_cluster_identifier_binding,
                },
                register_interface::ObjectField {
                    name: "featureName".into(),
                    value: &feature_name_binding,
                },
                register_interface::ObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "dbClusterIdentifier".into(),
                },
                register_interface::ResultField {
                    name: "featureName".into(),
                },
                register_interface::ResultField {
                    name: "roleArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ClusterRoleAssociationResult {
            db_cluster_identifier: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("dbClusterIdentifier").unwrap(),
            ),
            feature_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("featureName").unwrap(),
            ),
            role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("roleArn").unwrap(),
            ),
        }
    }
}

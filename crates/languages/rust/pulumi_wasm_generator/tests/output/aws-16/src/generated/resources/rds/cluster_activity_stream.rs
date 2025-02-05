/// Manages RDS Aurora Cluster Database Activity Streams.
///
/// Database Activity Streams have some limits and requirements, refer to the [Monitoring Amazon Aurora using Database Activity Streams](https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/DBActivityStreams.html) documentation for detailed limitations and requirements.
///
/// > **Note:** This resource always calls the RDS [`StartActivityStream`][2] API with the `ApplyImmediately` parameter set to `true`. This is because the provider needs the activity stream to be started in order for it to get the associated attributes.
///
/// > **Note:** This resource depends on having at least one `aws.rds.ClusterInstance` created. To avoid race conditions when all resources are being created together, add an explicit resource reference using the resource `depends_on` meta-argument.
///
/// > **Note:** This resource is available in all regions except the following: `cn-north-1`, `cn-northwest-1`, `us-gov-east-1`, `us-gov-west-1`
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = cluster::create(
///         "default",
///         ClusterArgs::builder()
///             .availability_zones(vec!["us-west-2a", "us-west-2b", "us-west-2c",])
///             .cluster_identifier("aurora-cluster-demo")
///             .database_name("mydb")
///             .engine("aurora-postgresql")
///             .engine_version("13.4")
///             .master_password("mustbeeightcharaters")
///             .master_username("foo")
///             .build_struct(),
///     );
///     let defaultClusterActivityStream = cluster_activity_stream::create(
///         "defaultClusterActivityStream",
///         ClusterActivityStreamArgs::builder()
///             .kms_key_id("${defaultKey.keyId}")
///             .mode("async")
///             .resource_arn("${default.arn}")
///             .build_struct(),
///     );
///     let defaultClusterInstance = cluster_instance::create(
///         "defaultClusterInstance",
///         ClusterInstanceArgs::builder()
///             .cluster_identifier("${default.clusterIdentifier}")
///             .engine("${default.engine}")
///             .identifier("aurora-instance-demo")
///             .instance_class("db.r6g.large")
///             .build_struct(),
///     );
///     let defaultKey = key::create(
///         "defaultKey",
///         KeyArgs::builder()
///             .description("AWS KMS Key to encrypt Database Activity Stream")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import RDS Aurora Cluster Database Activity Streams using the `resource_arn`. For example:
///
/// ```sh
/// $ pulumi import aws:rds/clusterActivityStream:ClusterActivityStream default arn:aws:rds:us-west-2:123456789012:cluster:aurora-cluster-demo
/// ```
pub mod cluster_activity_stream {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterActivityStreamArgs {
        /// Specifies whether the database activity stream includes engine-native audit fields. This option only applies to an Oracle DB instance. By default, no engine-native audit fields are included. Defaults `false`.
        #[builder(into, default)]
        pub engine_native_audit_fields_included: pulumi_wasm_rust::InputOrOutput<
            Option<bool>,
        >,
        /// The AWS KMS key identifier for encrypting messages in the database activity stream. The AWS KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.
        #[builder(into)]
        pub kms_key_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the mode of the database activity stream. Database events such as a change or access generate an activity stream event. The database session can handle these events either synchronously or asynchronously. One of: `sync`, `async`.
        #[builder(into)]
        pub mode: pulumi_wasm_rust::InputOrOutput<String>,
        /// The Amazon Resource Name (ARN) of the DB cluster.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterActivityStreamResult {
        /// Specifies whether the database activity stream includes engine-native audit fields. This option only applies to an Oracle DB instance. By default, no engine-native audit fields are included. Defaults `false`.
        pub engine_native_audit_fields_included: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the Amazon Kinesis data stream to be used for the database activity stream.
        pub kinesis_stream_name: pulumi_wasm_rust::Output<String>,
        /// The AWS KMS key identifier for encrypting messages in the database activity stream. The AWS KMS key identifier is the key ARN, key ID, alias ARN, or alias name for the KMS key.
        pub kms_key_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the mode of the database activity stream. Database events such as a change or access generate an activity stream event. The database session can handle these events either synchronously or asynchronously. One of: `sync`, `async`.
        pub mode: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) of the DB cluster.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ClusterActivityStreamArgs,
    ) -> ClusterActivityStreamResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let engine_native_audit_fields_included_binding = args
            .engine_native_audit_fields_included
            .get_output(context)
            .get_inner();
        let kms_key_id_binding = args.kms_key_id.get_output(context).get_inner();
        let mode_binding = args.mode.get_output(context).get_inner();
        let resource_arn_binding = args.resource_arn.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:rds/clusterActivityStream:ClusterActivityStream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "engineNativeAuditFieldsIncluded".into(),
                    value: &engine_native_audit_fields_included_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterActivityStreamResult {
            engine_native_audit_fields_included: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("engineNativeAuditFieldsIncluded"),
            ),
            kinesis_stream_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kinesisStreamName"),
            ),
            kms_key_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyId"),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(o.extract_field("mode")),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceArn"),
            ),
        }
    }
}

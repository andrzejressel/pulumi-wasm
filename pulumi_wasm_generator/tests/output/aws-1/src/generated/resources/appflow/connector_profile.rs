/// Provides an AppFlow connector profile resource.
///
/// For information about AppFlow flows, see the [Amazon AppFlow API Reference](https://docs.aws.amazon.com/appflow/1.0/APIReference/Welcome.html).
/// For specific information about creating an AppFlow connector profile, see the
/// [CreateConnectorProfile](https://docs.aws.amazon.com/appflow/1.0/APIReference/API_CreateConnectorProfile.html) page in the Amazon AppFlow API Reference.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example_role
///       managedPolicyArns:
///         - ${test.arn}
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Sid: ""
///               Principal:
///                 Service: ec2.amazonaws.com
///   exampleBucketV2:
///     type: aws:s3:BucketV2
///     name: example
///     properties:
///       bucket: example-bucket
///   exampleCluster:
///     type: aws:redshift:Cluster
///     name: example
///     properties:
///       clusterIdentifier: example_cluster
///       databaseName: example_db
///       masterUsername: exampleuser
///       masterPassword: examplePassword123!
///       nodeType: dc1.large
///       clusterType: single-node
///   exampleConnectorProfile:
///     type: aws:appflow:ConnectorProfile
///     name: example
///     properties:
///       name: example_profile
///       connectorType: Redshift
///       connectionMode: Public
///       connectorProfileConfig:
///         connectorProfileCredentials:
///           redshift:
///             password: ${exampleCluster.masterPassword}
///             username: ${exampleCluster.masterUsername}
///         connectorProfileProperties:
///           redshift:
///             bucketName: ${exampleBucketV2.name}
///             databaseUrl: jdbc:redshift://${exampleCluster.endpoint}/${exampleCluster.databaseName}
///             roleArn: ${exampleRole.arn}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicy
///       arguments:
///         name: AmazonRedshiftAllCommandsFullAccess
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppFlow Connector Profile using the connector profile `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:appflow/connectorProfile:ConnectorProfile profile arn:aws:appflow:us-west-2:123456789012:connectorprofile/example-profile
/// ```
pub mod connector_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectorProfileArgs {
        /// Indicates the connection mode and specifies whether it is public or private. Private flows use AWS PrivateLink to route data over AWS infrastructure without exposing it to the public internet. One of: `Public`, `Private`.
        #[builder(into)]
        pub connection_mode: pulumi_wasm_rust::Output<String>,
        /// The label of the connector. The label is unique for each ConnectorRegistration in your AWS account. Only needed if calling for `CustomConnector` connector type.
        #[builder(into, default)]
        pub connector_label: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines the connector-specific configuration and credentials. See Connector Profile Config for more details.
        #[builder(into)]
        pub connector_profile_config: pulumi_wasm_rust::Output<
            super::super::types::appflow::ConnectorProfileConnectorProfileConfig,
        >,
        /// The type of connector. One of: `Amplitude`, `CustomConnector`, `CustomerProfiles`, `Datadog`, `Dynatrace`, `EventBridge`, `Googleanalytics`, `Honeycode`, `Infornexus`, `LookoutMetrics`, `Marketo`, `Redshift`, `S3`, `Salesforce`, `SAPOData`, `Servicenow`, `Singular`, `Slack`, `Snowflake`, `Trendmicro`, `Upsolver`, `Veeva`, `Zendesk`.
        #[builder(into)]
        pub connector_type: pulumi_wasm_rust::Output<String>,
        /// ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
        #[builder(into, default)]
        pub kms_arn: pulumi_wasm_rust::Output<Option<String>>,
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConnectorProfileResult {
        /// ARN of the connector profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Indicates the connection mode and specifies whether it is public or private. Private flows use AWS PrivateLink to route data over AWS infrastructure without exposing it to the public internet. One of: `Public`, `Private`.
        pub connection_mode: pulumi_wasm_rust::Output<String>,
        /// The label of the connector. The label is unique for each ConnectorRegistration in your AWS account. Only needed if calling for `CustomConnector` connector type.
        pub connector_label: pulumi_wasm_rust::Output<Option<String>>,
        /// Defines the connector-specific configuration and credentials. See Connector Profile Config for more details.
        pub connector_profile_config: pulumi_wasm_rust::Output<
            super::super::types::appflow::ConnectorProfileConnectorProfileConfig,
        >,
        /// The type of connector. One of: `Amplitude`, `CustomConnector`, `CustomerProfiles`, `Datadog`, `Dynatrace`, `EventBridge`, `Googleanalytics`, `Honeycode`, `Infornexus`, `LookoutMetrics`, `Marketo`, `Redshift`, `S3`, `Salesforce`, `SAPOData`, `Servicenow`, `Singular`, `Slack`, `Snowflake`, `Trendmicro`, `Upsolver`, `Veeva`, `Zendesk`.
        pub connector_type: pulumi_wasm_rust::Output<String>,
        /// ARN of the connector profile credentials.
        pub credentials_arn: pulumi_wasm_rust::Output<String>,
        /// ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
        pub kms_arn: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ConnectorProfileArgs) -> ConnectorProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let connection_mode_binding = args.connection_mode.get_inner();
        let connector_label_binding = args.connector_label.get_inner();
        let connector_profile_config_binding = args.connector_profile_config.get_inner();
        let connector_type_binding = args.connector_type.get_inner();
        let kms_arn_binding = args.kms_arn.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appflow/connectorProfile:ConnectorProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "connectionMode".into(),
                    value: &connection_mode_binding,
                },
                register_interface::ObjectField {
                    name: "connectorLabel".into(),
                    value: &connector_label_binding,
                },
                register_interface::ObjectField {
                    name: "connectorProfileConfig".into(),
                    value: &connector_profile_config_binding,
                },
                register_interface::ObjectField {
                    name: "connectorType".into(),
                    value: &connector_type_binding,
                },
                register_interface::ObjectField {
                    name: "kmsArn".into(),
                    value: &kms_arn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "connectionMode".into(),
                },
                register_interface::ResultField {
                    name: "connectorLabel".into(),
                },
                register_interface::ResultField {
                    name: "connectorProfileConfig".into(),
                },
                register_interface::ResultField {
                    name: "connectorType".into(),
                },
                register_interface::ResultField {
                    name: "credentialsArn".into(),
                },
                register_interface::ResultField {
                    name: "kmsArn".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ConnectorProfileResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            connection_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectionMode").unwrap(),
            ),
            connector_label: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectorLabel").unwrap(),
            ),
            connector_profile_config: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectorProfileConfig").unwrap(),
            ),
            connector_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("connectorType").unwrap(),
            ),
            credentials_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("credentialsArn").unwrap(),
            ),
            kms_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsArn").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
        }
    }
}

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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod connector_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConnectorProfileArgs {
        /// Indicates the connection mode and specifies whether it is public or private. Private flows use AWS PrivateLink to route data over AWS infrastructure without exposing it to the public internet. One of: `Public`, `Private`.
        #[builder(into)]
        pub connection_mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The label of the connector. The label is unique for each ConnectorRegistration in your AWS account. Only needed if calling for `CustomConnector` connector type.
        #[builder(into, default)]
        pub connector_label: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Defines the connector-specific configuration and credentials. See Connector Profile Config for more details.
        #[builder(into)]
        pub connector_profile_config: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::appflow::ConnectorProfileConnectorProfileConfig,
        >,
        /// The type of connector. One of: `Amplitude`, `CustomConnector`, `CustomerProfiles`, `Datadog`, `Dynatrace`, `EventBridge`, `Googleanalytics`, `Honeycode`, `Infornexus`, `LookoutMetrics`, `Marketo`, `Redshift`, `S3`, `Salesforce`, `SAPOData`, `Servicenow`, `Singular`, `Slack`, `Snowflake`, `Trendmicro`, `Upsolver`, `Veeva`, `Zendesk`.
        #[builder(into)]
        pub connector_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
        #[builder(into, default)]
        pub kms_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ConnectorProfileResult {
        /// ARN of the connector profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Indicates the connection mode and specifies whether it is public or private. Private flows use AWS PrivateLink to route data over AWS infrastructure without exposing it to the public internet. One of: `Public`, `Private`.
        pub connection_mode: pulumi_gestalt_rust::Output<String>,
        /// The label of the connector. The label is unique for each ConnectorRegistration in your AWS account. Only needed if calling for `CustomConnector` connector type.
        pub connector_label: pulumi_gestalt_rust::Output<Option<String>>,
        /// Defines the connector-specific configuration and credentials. See Connector Profile Config for more details.
        pub connector_profile_config: pulumi_gestalt_rust::Output<
            super::super::types::appflow::ConnectorProfileConnectorProfileConfig,
        >,
        /// The type of connector. One of: `Amplitude`, `CustomConnector`, `CustomerProfiles`, `Datadog`, `Dynatrace`, `EventBridge`, `Googleanalytics`, `Honeycode`, `Infornexus`, `LookoutMetrics`, `Marketo`, `Redshift`, `S3`, `Salesforce`, `SAPOData`, `Servicenow`, `Singular`, `Slack`, `Snowflake`, `Trendmicro`, `Upsolver`, `Veeva`, `Zendesk`.
        pub connector_type: pulumi_gestalt_rust::Output<String>,
        /// ARN of the connector profile credentials.
        pub credentials_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN (Amazon Resource Name) of the Key Management Service (KMS) key you provide for encryption. This is required if you do not want to use the Amazon AppFlow-managed KMS key. If you don't provide anything here, Amazon AppFlow uses the Amazon AppFlow-managed KMS key.
        pub kms_arn: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConnectorProfileArgs,
    ) -> ConnectorProfileResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let connection_mode_binding = args.connection_mode.get_output(context);
        let connector_label_binding = args.connector_label.get_output(context);
        let connector_profile_config_binding = args
            .connector_profile_config
            .get_output(context);
        let connector_type_binding = args.connector_type.get_output(context);
        let kms_arn_binding = args.kms_arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appflow/connectorProfile:ConnectorProfile".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectionMode".into(),
                    value: &connection_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectorLabel".into(),
                    value: &connector_label_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectorProfileConfig".into(),
                    value: &connector_profile_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "connectorType".into(),
                    value: &connector_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsArn".into(),
                    value: &kms_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConnectorProfileResult {
            arn: o.get_field("arn"),
            connection_mode: o.get_field("connectionMode"),
            connector_label: o.get_field("connectorLabel"),
            connector_profile_config: o.get_field("connectorProfileConfig"),
            connector_type: o.get_field("connectorType"),
            credentials_arn: o.get_field("credentialsArn"),
            kms_arn: o.get_field("kmsArn"),
            name: o.get_field("name"),
        }
    }
}

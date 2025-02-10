/// Manages an Amazon Managed Service for Prometheus (AMP) Workspace.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:amp:Workspace
///     properties:
///       alias: example
///       tags:
///         Environment: production
/// ```
///
/// ### CloudWatch Logging
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = log_group::create(
///         "example",
///         LogGroupArgs::builder().name("example").build_struct(),
///     );
///     let exampleWorkspace = workspace::create(
///         "exampleWorkspace",
///         WorkspaceArgs::builder()
///             .logging_configuration(
///                 WorkspaceLoggingConfiguration::builder()
///                     .logGroupArn("${example.arn}:*")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### AWS KMS Customer Managed Keys (CMK)
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = workspace::create(
///         "example",
///         WorkspaceArgs::builder()
///             .alias("example")
///             .kms_key_arn("${exampleKey.arn}")
///             .build_struct(),
///     );
///     let exampleKey = key::create(
///         "exampleKey",
///         KeyArgs::builder()
///             .deletion_window_in_days(7)
///             .description("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AMP Workspaces using the identifier. For example:
///
/// ```sh
/// $ pulumi import aws:amp/workspace:Workspace demo ws-C6DCB907-F2D7-4D96-957B-66691F865D8B
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod workspace {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct WorkspaceArgs {
        /// The alias of the prometheus workspace. See more [in AWS Docs](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-onboard-create-workspace.html).
        #[builder(into, default)]
        pub alias: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ARN for the KMS encryption key. If this argument is not provided, then the AWS owned encryption key will be used to encrypt the data in the workspace. See more [in AWS Docs](https://docs.aws.amazon.com/prometheus/latest/userguide/encryption-at-rest-Amazon-Service-Prometheus.html)
        #[builder(into, default)]
        pub kms_key_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Logging configuration for the workspace. See Logging Configuration below for details.
        #[builder(into, default)]
        pub logging_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::amp::WorkspaceLoggingConfiguration>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct WorkspaceResult {
        /// The alias of the prometheus workspace. See more [in AWS Docs](https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-onboard-create-workspace.html).
        pub alias: pulumi_gestalt_rust::Output<Option<String>>,
        /// Amazon Resource Name (ARN) of the workspace.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN for the KMS encryption key. If this argument is not provided, then the AWS owned encryption key will be used to encrypt the data in the workspace. See more [in AWS Docs](https://docs.aws.amazon.com/prometheus/latest/userguide/encryption-at-rest-Amazon-Service-Prometheus.html)
        pub kms_key_arn: pulumi_gestalt_rust::Output<Option<String>>,
        /// Logging configuration for the workspace. See Logging Configuration below for details.
        pub logging_configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::amp::WorkspaceLoggingConfiguration>,
        >,
        /// Prometheus endpoint available for this workspace.
        pub prometheus_endpoint: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: WorkspaceArgs,
    ) -> WorkspaceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let alias_binding = args.alias.get_output(context);
        let kms_key_arn_binding = args.kms_key_arn.get_output(context);
        let logging_configuration_binding = args
            .logging_configuration
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:amp/workspace:Workspace".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "alias".into(),
                    value: alias_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyArn".into(),
                    value: kms_key_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "loggingConfiguration".into(),
                    value: logging_configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        WorkspaceResult {
            alias: o.get_field("alias"),
            arn: o.get_field("arn"),
            kms_key_arn: o.get_field("kmsKeyArn"),
            logging_configuration: o.get_field("loggingConfiguration"),
            prometheus_endpoint: o.get_field("prometheusEndpoint"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}

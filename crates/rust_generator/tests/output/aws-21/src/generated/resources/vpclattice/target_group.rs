/// Resource for managing an AWS VPC Lattice Target Group.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = target_group::create(
///         "example",
///         TargetGroupArgs::builder()
///             .config(
///                 TargetGroupConfig::builder()
///                     .port(443)
///                     .protocol("HTTPS")
///                     .vpcIdentifier("${exampleAwsVpc.id}")
///                     .build_struct(),
///             )
///             .name("example")
///             .type_("INSTANCE")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Basic usage with Health check
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = target_group::create(
///         "example",
///         TargetGroupArgs::builder()
///             .config(
///                 TargetGroupConfig::builder()
///                     .healthCheck(
///                         TargetGroupConfigHealthCheck::builder()
///                             .enabled(true)
///                             .healthCheckIntervalSeconds(20)
///                             .healthCheckTimeoutSeconds(10)
///                             .healthyThresholdCount(7)
///                             .matcher(
///                                 TargetGroupConfigHealthCheckMatcher::builder()
///                                     .value("200-299")
///                                     .build_struct(),
///                             )
///                             .path("/instance")
///                             .port(80)
///                             .protocol("HTTP")
///                             .protocolVersion("HTTP1")
///                             .unhealthyThresholdCount(3)
///                             .build_struct(),
///                     )
///                     .ipAddressType("IPV4")
///                     .port(443)
///                     .protocol("HTTPS")
///                     .protocolVersion("HTTP1")
///                     .vpcIdentifier("${exampleAwsVpc.id}")
///                     .build_struct(),
///             )
///             .name("example")
///             .type_("IP")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### ALB
///
/// If the type is ALB, `health_check` block is not supported.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = target_group::create(
///         "example",
///         TargetGroupArgs::builder()
///             .config(
///                 TargetGroupConfig::builder()
///                     .port(443)
///                     .protocol("HTTPS")
///                     .protocolVersion("HTTP1")
///                     .vpcIdentifier("${exampleAwsVpc.id}")
///                     .build_struct(),
///             )
///             .name("example")
///             .type_("ALB")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Lambda
///
/// If the type is Lambda, `config` block is not supported.
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = target_group::create(
///         "example",
///         TargetGroupArgs::builder().name("example").type_("LAMBDA").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import VPC Lattice Target Group using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:vpclattice/targetGroup:TargetGroup example tg-0c11d4dc16ed96bdb
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod target_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TargetGroupArgs {
        /// The target group configuration.
        #[builder(into, default)]
        pub config: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::vpclattice::TargetGroupConfig>,
        >,
        /// The name of the target group. The name must be unique within the account. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The type of target group. Valid Values are `IP` | `LAMBDA` | `INSTANCE` | `ALB`
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TargetGroupResult {
        /// ARN of the target group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The target group configuration.
        pub config: pulumi_gestalt_rust::Output<
            Option<super::super::types::vpclattice::TargetGroupConfig>,
        >,
        /// The name of the target group. The name must be unique within the account. The valid characters are a-z, 0-9, and hyphens (-). You can't use a hyphen as the first or last character, or immediately after another hyphen.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Status of the target group.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The type of target group. Valid Values are `IP` | `LAMBDA` | `INSTANCE` | `ALB`
        ///
        /// The following arguments are optional:
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TargetGroupArgs,
    ) -> TargetGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let config_binding = args.config.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:vpclattice/targetGroup:TargetGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "config".into(),
                    value: &config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TargetGroupResult {
            arn: o.get_field("arn"),
            config: o.get_field("config"),
            name: o.get_field("name"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            type_: o.get_field("type"),
        }
    }
}

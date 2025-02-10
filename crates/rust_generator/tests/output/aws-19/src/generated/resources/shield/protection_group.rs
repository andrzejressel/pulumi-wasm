/// Creates a grouping of protected resources so they can be handled as a collective.
/// This resource grouping improves the accuracy of detection and reduces false positives. For more information see
/// [Managing AWS Shield Advanced protection groups](https://docs.aws.amazon.com/waf/latest/developerguide/manage-protection-group.html)
///
/// ## Example Usage
///
/// ### Create protection group for all resources
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = protection_group::create(
///         "example",
///         ProtectionGroupArgs::builder()
///             .aggregation("MAX")
///             .pattern("ALL")
///             .protection_group_id("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Create protection group for arbitrary number of resources
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:Eip
///     properties:
///       domain: vpc
///   exampleProtection:
///     type: aws:shield:Protection
///     name: example
///     properties:
///       name: example
///       resourceArn: arn:aws:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${example.id}
///   exampleProtectionGroup:
///     type: aws:shield:ProtectionGroup
///     name: example
///     properties:
///       protectionGroupId: example
///       aggregation: MEAN
///       pattern: ARBITRARY
///       members:
///         - arn:aws:ec2:${current.name}:${currentGetCallerIdentity.accountId}:eip-allocation/${example.id}
///     options:
///       dependsOn:
///         - ${exampleProtection}
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
///   currentGetCallerIdentity:
///     fn::invoke:
///       function: aws:getCallerIdentity
///       arguments: {}
/// ```
///
/// ### Create protection group for a type of resource
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = protection_group::create(
///         "example",
///         ProtectionGroupArgs::builder()
///             .aggregation("SUM")
///             .pattern("BY_RESOURCE_TYPE")
///             .protection_group_id("example")
///             .resource_type("ELASTIC_IP_ALLOCATION")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Shield protection group resources using their protection group id. For example:
///
/// ```sh
/// $ pulumi import aws:shield/protectionGroup:ProtectionGroup example example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod protection_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProtectionGroupArgs {
        /// Defines how AWS Shield combines resource data for the group in order to detect, mitigate, and report events.
        #[builder(into)]
        pub aggregation: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The Amazon Resource Names (ARNs) of the resources to include in the protection group. You must set this when you set `pattern` to ARBITRARY and you must not set it for any other `pattern` setting.
        #[builder(into, default)]
        pub members: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The criteria to use to choose the protected resources for inclusion in the group.
        #[builder(into)]
        pub pattern: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the protection group.
        #[builder(into)]
        pub protection_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The resource type to include in the protection group. You must set this when you set `pattern` to BY_RESOURCE_TYPE and you must not set it for any other `pattern` setting.
        #[builder(into, default)]
        pub resource_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProtectionGroupResult {
        /// Defines how AWS Shield combines resource data for the group in order to detect, mitigate, and report events.
        pub aggregation: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Names (ARNs) of the resources to include in the protection group. You must set this when you set `pattern` to ARBITRARY and you must not set it for any other `pattern` setting.
        pub members: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The criteria to use to choose the protected resources for inclusion in the group.
        pub pattern: pulumi_gestalt_rust::Output<String>,
        /// The ARN (Amazon Resource Name) of the protection group.
        pub protection_group_arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the protection group.
        pub protection_group_id: pulumi_gestalt_rust::Output<String>,
        /// The resource type to include in the protection group. You must set this when you set `pattern` to BY_RESOURCE_TYPE and you must not set it for any other `pattern` setting.
        pub resource_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: ProtectionGroupArgs,
    ) -> ProtectionGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let aggregation_binding = args.aggregation.get_output(context);
        let members_binding = args.members.get_output(context);
        let pattern_binding = args.pattern.get_output(context);
        let protection_group_id_binding = args.protection_group_id.get_output(context);
        let resource_type_binding = args.resource_type.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:shield/protectionGroup:ProtectionGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "aggregation".into(),
                    value: aggregation_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: members_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pattern".into(),
                    value: pattern_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "protectionGroupId".into(),
                    value: protection_group_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceType".into(),
                    value: resource_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ProtectionGroupResult {
            aggregation: o.get_field("aggregation"),
            members: o.get_field("members"),
            pattern: o.get_field("pattern"),
            protection_group_arn: o.get_field("protectionGroupArn"),
            protection_group_id: o.get_field("protectionGroupId"),
            resource_type: o.get_field("resourceType"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}

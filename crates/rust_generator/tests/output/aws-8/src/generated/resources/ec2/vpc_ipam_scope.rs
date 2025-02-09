/// Creates a scope for AWS IPAM.
///
/// ## Example Usage
///
/// Basic usage:
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:VpcIpam
///     properties:
///       operatingRegions:
///         - regionName: ${current.name}
///   exampleVpcIpamScope:
///     type: aws:ec2:VpcIpamScope
///     name: example
///     properties:
///       ipamId: ${example.id}
///       description: Another Scope
/// variables:
///   current:
///     fn::invoke:
///       function: aws:getRegion
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IPAMs using the `scope_id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/vpcIpamScope:VpcIpamScope example ipam-scope-0513c69f283d11dfb
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpc_ipam_scope {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpcIpamScopeArgs {
        /// A description for the scope you're creating.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the IPAM for which you're creating this scope.
        #[builder(into)]
        pub ipam_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct VpcIpamScopeResult {
        /// The Amazon Resource Name (ARN) of the scope.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description for the scope you're creating.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ARN of the IPAM for which you're creating this scope.
        pub ipam_arn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the IPAM for which you're creating this scope.
        pub ipam_id: pulumi_gestalt_rust::Output<String>,
        pub ipam_scope_type: pulumi_gestalt_rust::Output<String>,
        /// Defines if the scope is the default scope or not.
        pub is_default: pulumi_gestalt_rust::Output<bool>,
        /// The number of pools in the scope.
        pub pool_count: pulumi_gestalt_rust::Output<i32>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
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
        args: VpcIpamScopeArgs,
    ) -> VpcIpamScopeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let ipam_id_binding = args.ipam_id.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamScope:VpcIpamScope".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ipamId".into(),
                    value: ipam_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpcIpamScopeResult {
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            ipam_arn: o.get_field("ipamArn"),
            ipam_id: o.get_field("ipamId"),
            ipam_scope_type: o.get_field("ipamScopeType"),
            is_default: o.get_field("isDefault"),
            pool_count: o.get_field("poolCount"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}

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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: VpcIpamScopeArgs,
    ) -> VpcIpamScopeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_output(context).get_inner();
        let ipam_id_binding = args.ipam_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpcIpamScope:VpcIpamScope".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "ipamId".into(),
                    value: &ipam_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VpcIpamScopeResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            ipam_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamArn"),
            ),
            ipam_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamId"),
            ),
            ipam_scope_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ipamScopeType"),
            ),
            is_default: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isDefault"),
            ),
            pool_count: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("poolCount"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}

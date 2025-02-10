/// Provides a managed prefix list resource.
///
/// > **NOTE on Managed Prefix Lists and Managed Prefix List Entries:** The provider
/// currently provides both a standalone Managed Prefix List Entry resource (a single entry),
/// and a Managed Prefix List resource with entries defined in-line. At this time you
/// cannot use a Managed Prefix List with in-line rules in conjunction with any Managed
/// Prefix List Entry resources. Doing so will cause a conflict of entries and will overwrite entries.
///
/// > **NOTE on `max_entries`:** When you reference a Prefix List in a resource,
/// the maximum number of entries for the prefix lists counts as the same number of rules
/// or entries for the resource. For example, if you create a prefix list with a maximum
/// of 20 entries and you reference that prefix list in a security group rule, this counts
/// as 20 rules for the security group.
///
/// ## Example Usage
///
/// Basic usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:ManagedPrefixList
///     properties:
///       name: All VPC CIDR-s
///       addressFamily: IPv4
///       maxEntries: 5
///       entries:
///         - cidr: ${exampleAwsVpc.cidrBlock}
///           description: Primary
///         - cidr: ${exampleAwsVpcIpv4CidrBlockAssociation.cidrBlock}
///           description: Secondary
///       tags:
///         Env: live
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Prefix Lists using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ec2/managedPrefixList:ManagedPrefixList default pl-0570a1d2d725c16be
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod managed_prefix_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedPrefixListArgs {
        /// Address family (`IPv4` or `IPv6`) of this prefix list.
        #[builder(into)]
        pub address_family: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Configuration block for prefix list entry. Detailed below. Different entries may have overlapping CIDR blocks, but a particular CIDR should not be duplicated.
        #[builder(into, default)]
        pub entries: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::ec2::ManagedPrefixListEntry>>,
        >,
        /// Maximum number of entries that this prefix list can contain.
        #[builder(into)]
        pub max_entries: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Name of this resource. The name must not start with `com.amazonaws`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ManagedPrefixListResult {
        /// Address family (`IPv4` or `IPv6`) of this prefix list.
        pub address_family: pulumi_gestalt_rust::Output<String>,
        /// ARN of the prefix list.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for prefix list entry. Detailed below. Different entries may have overlapping CIDR blocks, but a particular CIDR should not be duplicated.
        pub entries: pulumi_gestalt_rust::Output<
            Vec<super::super::types::ec2::ManagedPrefixListEntry>,
        >,
        /// Maximum number of entries that this prefix list can contain.
        pub max_entries: pulumi_gestalt_rust::Output<i32>,
        /// Name of this resource. The name must not start with `com.amazonaws`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ID of the AWS account that owns this prefix list.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Latest version of this prefix list.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ManagedPrefixListArgs,
    ) -> ManagedPrefixListResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let address_family_binding = args.address_family.get_output(context);
        let entries_binding = args.entries.get_output(context);
        let max_entries_binding = args.max_entries.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/managedPrefixList:ManagedPrefixList".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressFamily".into(),
                    value: address_family_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "entries".into(),
                    value: entries_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maxEntries".into(),
                    value: max_entries_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ManagedPrefixListResult {
            address_family: o.get_field("addressFamily"),
            arn: o.get_field("arn"),
            entries: o.get_field("entries"),
            max_entries: o.get_field("maxEntries"),
            name: o.get_field("name"),
            owner_id: o.get_field("ownerId"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version: o.get_field("version"),
        }
    }
}

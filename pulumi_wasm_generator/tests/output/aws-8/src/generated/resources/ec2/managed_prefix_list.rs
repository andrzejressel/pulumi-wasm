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
pub mod managed_prefix_list {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedPrefixListArgs {
        /// Address family (`IPv4` or `IPv6`) of this prefix list.
        #[builder(into)]
        pub address_family: pulumi_wasm_rust::Output<String>,
        /// Configuration block for prefix list entry. Detailed below. Different entries may have overlapping CIDR blocks, but a particular CIDR should not be duplicated.
        #[builder(into, default)]
        pub entries: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::ec2::ManagedPrefixListEntry>>,
        >,
        /// Maximum number of entries that this prefix list can contain.
        #[builder(into)]
        pub max_entries: pulumi_wasm_rust::Output<i32>,
        /// Name of this resource. The name must not start with `com.amazonaws`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ManagedPrefixListResult {
        /// Address family (`IPv4` or `IPv6`) of this prefix list.
        pub address_family: pulumi_wasm_rust::Output<String>,
        /// ARN of the prefix list.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration block for prefix list entry. Detailed below. Different entries may have overlapping CIDR blocks, but a particular CIDR should not be duplicated.
        pub entries: pulumi_wasm_rust::Output<
            Vec<super::super::types::ec2::ManagedPrefixListEntry>,
        >,
        /// Maximum number of entries that this prefix list can contain.
        pub max_entries: pulumi_wasm_rust::Output<i32>,
        /// Name of this resource. The name must not start with `com.amazonaws`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// ID of the AWS account that owns this prefix list.
        pub owner_id: pulumi_wasm_rust::Output<String>,
        /// Map of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Latest version of this prefix list.
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ManagedPrefixListArgs) -> ManagedPrefixListResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_family_binding = args.address_family.get_inner();
        let entries_binding = args.entries.get_inner();
        let max_entries_binding = args.max_entries.get_inner();
        let name_binding = args.name.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/managedPrefixList:ManagedPrefixList".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addressFamily".into(),
                    value: &address_family_binding,
                },
                register_interface::ObjectField {
                    name: "entries".into(),
                    value: &entries_binding,
                },
                register_interface::ObjectField {
                    name: "maxEntries".into(),
                    value: &max_entries_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "addressFamily".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "entries".into(),
                },
                register_interface::ResultField {
                    name: "maxEntries".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ownerId".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ManagedPrefixListResult {
            address_family: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("addressFamily").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            entries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("entries").unwrap(),
            ),
            max_entries: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("maxEntries").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerId").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}

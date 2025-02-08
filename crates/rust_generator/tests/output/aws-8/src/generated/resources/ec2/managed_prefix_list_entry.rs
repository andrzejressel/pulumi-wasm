/// Use the `aws_prefix_list_entry` resource to manage a managed prefix list entry.
///
/// > **NOTE:** Pulumi currently provides two resources for managing Managed Prefix Lists and Managed Prefix List Entries. The standalone resource, Managed Prefix List Entry, is used to manage a single entry. The Managed Prefix List resource is used to manage multiple entries defined in-line. It is important to note that you cannot use a Managed Prefix List with in-line rules in conjunction with any Managed Prefix List Entry resources. This will result in a conflict of entries and will cause the entries to be overwritten.
///
/// > **NOTE:** To improve execution times on larger updates, it is recommended to use the inline `entry` block as part of the Managed Prefix List resource when creating a prefix list with more than 100 entries. You can find more information about the resource here.
///
/// ## Example Usage
///
/// Basic usage.
///
/// ```yaml
/// resources:
///   example:
///     type: aws:ec2:ManagedPrefixList
///     properties:
///       name: All VPC CIDR-s
///       addressFamily: IPv4
///       maxEntries: 5
///       tags:
///         Env: live
///   entry1:
///     type: aws:ec2:ManagedPrefixListEntry
///     name: entry_1
///     properties:
///       cidr: ${exampleAwsVpc.cidrBlock}
///       description: Primary
///       prefixListId: ${example.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import prefix list entries using `prefix_list_id` and `cidr` separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:ec2/managedPrefixListEntry:ManagedPrefixListEntry default pl-0570a1d2d725c16be,10.0.3.0/24
/// ```
#[allow(clippy::doc_lazy_continuation)]
pub mod managed_prefix_list_entry {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ManagedPrefixListEntryArgs {
        /// CIDR block of this entry.
        #[builder(into)]
        pub cidr: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Description of this entry. Please note that due to API limitations, updating only the description of an entry will require recreating the entry.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the prefix list.
        #[builder(into)]
        pub prefix_list_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ManagedPrefixListEntryResult {
        /// CIDR block of this entry.
        pub cidr: pulumi_gestalt_rust::Output<String>,
        /// Description of this entry. Please note that due to API limitations, updating only the description of an entry will require recreating the entry.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the prefix list.
        pub prefix_list_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ManagedPrefixListEntryArgs,
    ) -> ManagedPrefixListEntryResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let cidr_binding = args.cidr.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let prefix_list_id_binding = args.prefix_list_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/managedPrefixListEntry:ManagedPrefixListEntry".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "cidr".into(),
                    value: &cidr_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "prefixListId".into(),
                    value: &prefix_list_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ManagedPrefixListEntryResult {
            cidr: pulumi_gestalt_rust::__private::into_domain(o.extract_field("cidr")),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            prefix_list_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("prefixListId"),
            ),
        }
    }
}

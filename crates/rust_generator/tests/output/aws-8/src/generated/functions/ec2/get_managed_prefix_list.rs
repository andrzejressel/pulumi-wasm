#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_managed_prefix_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetManagedPrefixListArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetManagedPrefixListFilter>>,
        >,
        /// ID of the prefix list to select.
        #[builder(into, default)]
        pub id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Name of the prefix list to select.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Map of tags assigned to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetManagedPrefixListResult {
        /// Address family of the prefix list. Valid values are `IPv4` and `IPv6`.
        pub address_family: pulumi_gestalt_rust::Output<String>,
        /// ARN of the selected prefix list.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Set of entries in this prefix list. Each entry is an object with `cidr` and `description`.
        pub entries: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::ec2::GetManagedPrefixListEntry>,
        >,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetManagedPrefixListFilter>>,
        >,
        /// ID of the selected prefix list.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// When then prefix list is managed, the maximum number of entries it supports, or null otherwise.
        pub max_entries: pulumi_gestalt_rust::Output<i32>,
        /// Name of the selected prefix list.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Account ID of the owner of a customer-managed prefix list, or `AWS` otherwise.
        pub owner_id: pulumi_gestalt_rust::Output<String>,
        /// Map of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetManagedPrefixListArgs,
    ) -> GetManagedPrefixListResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let id_binding = args.id.get_output(context);
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getManagedPrefixList:getManagedPrefixList".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "id".into(),
                    value: id_binding.get_id(),
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
        let o = context.invoke_resource(request);
        GetManagedPrefixListResult {
            address_family: o.get_field("addressFamily"),
            arn: o.get_field("arn"),
            entries: o.get_field("entries"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            max_entries: o.get_field("maxEntries"),
            name: o.get_field("name"),
            owner_id: o.get_field("ownerId"),
            tags: o.get_field("tags"),
            version: o.get_field("version"),
        }
    }
}

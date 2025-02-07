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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetManagedPrefixListArgs,
    ) -> GetManagedPrefixListResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_output(context).get_inner();
        let id_binding = args.id.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getManagedPrefixList:getManagedPrefixList".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetManagedPrefixListResult {
            address_family: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("addressFamily"),
            ),
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            entries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("entries"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            max_entries: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("maxEntries"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            owner_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ownerId"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            version: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("version"),
            ),
        }
    }
}

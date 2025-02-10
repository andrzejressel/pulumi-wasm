#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_prefix_list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPrefixListArgs {
        /// Configuration block(s) for filtering. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetPrefixListFilter>>,
        >,
        /// Name of the prefix list to select.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the prefix list to select.
        #[builder(into, default)]
        pub prefix_list_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetPrefixListResult {
        /// List of CIDR blocks for the AWS service associated with the prefix list.
        pub cidr_blocks: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetPrefixListFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the selected prefix list.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub prefix_list_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetPrefixListArgs,
    ) -> GetPrefixListResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let name_binding = args.name.get_output(context);
        let prefix_list_id_binding = args.prefix_list_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getPrefixList:getPrefixList".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefixListId".into(),
                    value: prefix_list_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetPrefixListResult {
            cidr_blocks: o.get_field("cidrBlocks"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            prefix_list_id: o.get_field("prefixListId"),
        }
    }
}

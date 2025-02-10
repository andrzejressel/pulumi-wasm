#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_eips {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEipsArgs {
        /// Custom filter block as described below.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::super::types::ec2::GetEipsFilter>>,
        >,
        /// Map of tags, each pair of which must exactly match a pair on the desired Elastic IPs.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetEipsResult {
        /// List of all the allocation IDs for address for use with EC2-VPC.
        pub allocation_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        pub filters: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::super::types::ec2::GetEipsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// List of all the Elastic IP addresses.
        pub public_ips: pulumi_gestalt_rust::Output<Vec<String>>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEipsArgs,
    ) -> GetEipsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let filters_binding = args.filters.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getEips:getEips".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEipsResult {
            allocation_ids: o.get_field("allocationIds"),
            filters: o.get_field("filters"),
            id: o.get_field("id"),
            public_ips: o.get_field("publicIps"),
            tags: o.get_field("tags"),
        }
    }
}

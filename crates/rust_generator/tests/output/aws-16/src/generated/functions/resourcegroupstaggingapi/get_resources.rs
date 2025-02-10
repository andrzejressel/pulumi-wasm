#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_resources {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourcesArgs {
        /// Specifies whether to exclude resources that are compliant with the tag policy. You can use this parameter only if the `include_compliance_details` argument is also set to `true`.
        #[builder(into, default)]
        pub exclude_compliant_resources: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies whether to include details regarding the compliance with the effective tag policy.
        #[builder(into, default)]
        pub include_compliance_details: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies a list of ARNs of resources for which you want to retrieve tag data. Conflicts with `filter`.
        #[builder(into, default)]
        pub resource_arn_lists: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Constraints on the resources that you want returned. The format of each resource type is `service:resourceType`. For example, specifying a resource type of `ec2` returns all Amazon EC2 resources (which includes EC2 instances). Specifying a resource type of `ec2:instance` returns only EC2 instances.
        #[builder(into, default)]
        pub resource_type_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// Specifies a list of Tag Filters (keys and values) to restrict the output to only those resources that have the specified tag and, if included, the specified value. See Tag Filter below. Conflicts with `resource_arn_list`.
        #[builder(into, default)]
        pub tag_filters: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<
                    super::super::super::types::resourcegroupstaggingapi::GetResourcesTagFilter,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetResourcesResult {
        pub exclude_compliant_resources: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub include_compliance_details: pulumi_gestalt_rust::Output<Option<bool>>,
        pub resource_arn_lists: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of objects matching the search criteria.
        pub resource_tag_mapping_lists: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::resourcegroupstaggingapi::GetResourcesResourceTagMappingList,
            >,
        >,
        pub resource_type_filters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        pub tag_filters: pulumi_gestalt_rust::Output<
            Option<
                Vec<
                    super::super::super::types::resourcegroupstaggingapi::GetResourcesTagFilter,
                >,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetResourcesArgs,
    ) -> GetResourcesResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let exclude_compliant_resources_binding = args
            .exclude_compliant_resources
            .get_output(context);
        let include_compliance_details_binding = args
            .include_compliance_details
            .get_output(context);
        let resource_arn_lists_binding = args.resource_arn_lists.get_output(context);
        let resource_type_filters_binding = args
            .resource_type_filters
            .get_output(context);
        let tag_filters_binding = args.tag_filters.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:resourcegroupstaggingapi/getResources:getResources".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "excludeCompliantResources".into(),
                    value: exclude_compliant_resources_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeComplianceDetails".into(),
                    value: include_compliance_details_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArnLists".into(),
                    value: resource_arn_lists_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceTypeFilters".into(),
                    value: resource_type_filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tagFilters".into(),
                    value: tag_filters_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetResourcesResult {
            exclude_compliant_resources: o.get_field("excludeCompliantResources"),
            id: o.get_field("id"),
            include_compliance_details: o.get_field("includeComplianceDetails"),
            resource_arn_lists: o.get_field("resourceArnLists"),
            resource_tag_mapping_lists: o.get_field("resourceTagMappingLists"),
            resource_type_filters: o.get_field("resourceTypeFilters"),
            tag_filters: o.get_field("tagFilters"),
        }
    }
}

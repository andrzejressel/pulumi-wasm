pub mod get_resources {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetResourcesArgs {
        /// Specifies whether to exclude resources that are compliant with the tag policy. You can use this parameter only if the `include_compliance_details` argument is also set to `true`.
        #[builder(into, default)]
        pub exclude_compliant_resources: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies whether to include details regarding the compliance with the effective tag policy.
        #[builder(into, default)]
        pub include_compliance_details: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies a list of ARNs of resources for which you want to retrieve tag data. Conflicts with `filter`.
        #[builder(into, default)]
        pub resource_arn_lists: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Constraints on the resources that you want returned. The format of each resource type is `service:resourceType`. For example, specifying a resource type of `ec2` returns all Amazon EC2 resources (which includes EC2 instances). Specifying a resource type of `ec2:instance` returns only EC2 instances.
        #[builder(into, default)]
        pub resource_type_filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// Specifies a list of Tag Filters (keys and values) to restrict the output to only those resources that have the specified tag and, if included, the specified value. See Tag Filter below. Conflicts with `resource_arn_list`.
        #[builder(into, default)]
        pub tag_filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::resourcegroupstaggingapi::GetResourcesTagFilter,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetResourcesResult {
        pub exclude_compliant_resources: pulumi_wasm_rust::Output<Option<bool>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub include_compliance_details: pulumi_wasm_rust::Output<Option<bool>>,
        pub resource_arn_lists: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// List of objects matching the search criteria.
        pub resource_tag_mapping_lists: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::resourcegroupstaggingapi::GetResourcesResourceTagMappingList,
            >,
        >,
        pub resource_type_filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub tag_filters: pulumi_wasm_rust::Output<
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
    pub fn invoke(args: GetResourcesArgs) -> GetResourcesResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let exclude_compliant_resources_binding = args
            .exclude_compliant_resources
            .get_inner();
        let include_compliance_details_binding = args
            .include_compliance_details
            .get_inner();
        let resource_arn_lists_binding = args.resource_arn_lists.get_inner();
        let resource_type_filters_binding = args.resource_type_filters.get_inner();
        let tag_filters_binding = args.tag_filters.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:resourcegroupstaggingapi/getResources:getResources".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "excludeCompliantResources".into(),
                    value: &exclude_compliant_resources_binding,
                },
                register_interface::ObjectField {
                    name: "includeComplianceDetails".into(),
                    value: &include_compliance_details_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArnLists".into(),
                    value: &resource_arn_lists_binding,
                },
                register_interface::ObjectField {
                    name: "resourceTypeFilters".into(),
                    value: &resource_type_filters_binding,
                },
                register_interface::ObjectField {
                    name: "tagFilters".into(),
                    value: &tag_filters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "excludeCompliantResources".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "includeComplianceDetails".into(),
                },
                register_interface::ResultField {
                    name: "resourceArnLists".into(),
                },
                register_interface::ResultField {
                    name: "resourceTagMappingLists".into(),
                },
                register_interface::ResultField {
                    name: "resourceTypeFilters".into(),
                },
                register_interface::ResultField {
                    name: "tagFilters".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetResourcesResult {
            exclude_compliant_resources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("excludeCompliantResources").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            include_compliance_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includeComplianceDetails").unwrap(),
            ),
            resource_arn_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArnLists").unwrap(),
            ),
            resource_tag_mapping_lists: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTagMappingLists").unwrap(),
            ),
            resource_type_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceTypeFilters").unwrap(),
            ),
            tag_filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagFilters").unwrap(),
            ),
        }
    }
}

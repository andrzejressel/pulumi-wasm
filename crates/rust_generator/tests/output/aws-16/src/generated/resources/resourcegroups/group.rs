/// Provides a Resource Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = group::create(
///         "test",
///         GroupArgs::builder()
///             .name("test-group")
///             .resource_query(
///                 GroupResourceQuery::builder()
///                     .query(
///                         "{\n  \"ResourceTypeFilters\": [\n    \"AWS::EC2::Instance\"\n  ],\n  \"TagFilters\": [\n    {\n      \"Key\": \"Stage\",\n      \"Values\": [\"Test\"]\n    }\n  ]\n}",
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import resource groups using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:resourcegroups/group:Group foo resource-group-name
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// A configuration associates the resource group with an AWS service and specifies how the service can interact with the resources in the group. See below for details.
        #[builder(into, default)]
        pub configurations: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::resourcegroups::GroupConfiguration>>,
        >,
        /// A description of the resource group.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The resource group's name. A resource group name can have a maximum of 127 characters, including letters, numbers, hyphens, dots, and underscores. The name cannot start with `AWS` or `aws`.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `resource_query` block. Resource queries are documented below.
        #[builder(into, default)]
        pub resource_query: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::resourcegroups::GroupResourceQuery>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// The ARN assigned by AWS for this resource group.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A configuration associates the resource group with an AWS service and specifies how the service can interact with the resources in the group. See below for details.
        pub configurations: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::resourcegroups::GroupConfiguration>>,
        >,
        /// A description of the resource group.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The resource group's name. A resource group name can have a maximum of 127 characters, including letters, numbers, hyphens, dots, and underscores. The name cannot start with `AWS` or `aws`.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `resource_query` block. Resource queries are documented below.
        pub resource_query: pulumi_gestalt_rust::Output<
            Option<super::super::types::resourcegroups::GroupResourceQuery>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: GroupArgs,
    ) -> GroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configurations_binding = args.configurations.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_query_binding = args.resource_query.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:resourcegroups/group:Group".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurations".into(),
                    value: &configurations_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceQuery".into(),
                    value: &resource_query_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GroupResult {
            arn: o.get_field("arn"),
            configurations: o.get_field("configurations"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            resource_query: o.get_field("resourceQuery"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}

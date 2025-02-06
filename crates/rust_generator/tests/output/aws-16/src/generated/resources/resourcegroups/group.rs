/// Provides a Resource Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod group {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GroupArgs {
        /// A configuration associates the resource group with an AWS service and specifies how the service can interact with the resources in the group. See below for details.
        #[builder(into, default)]
        pub configurations: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::resourcegroups::GroupConfiguration>>,
        >,
        /// A description of the resource group.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The resource group's name. A resource group name can have a maximum of 127 characters, including letters, numbers, hyphens, dots, and underscores. The name cannot start with `AWS` or `aws`.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A `resource_query` block. Resource queries are documented below.
        #[builder(into, default)]
        pub resource_query: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::resourcegroups::GroupResourceQuery>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GroupResult {
        /// The ARN assigned by AWS for this resource group.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// A configuration associates the resource group with an AWS service and specifies how the service can interact with the resources in the group. See below for details.
        pub configurations: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::resourcegroups::GroupConfiguration>>,
        >,
        /// A description of the resource group.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The resource group's name. A resource group name can have a maximum of 127 characters, including letters, numbers, hyphens, dots, and underscores. The name cannot start with `AWS` or `aws`.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `resource_query` block. Resource queries are documented below.
        pub resource_query: pulumi_wasm_rust::Output<
            Option<super::super::types::resourcegroups::GroupResourceQuery>,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GroupArgs,
    ) -> GroupResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configurations_binding = args.configurations.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_query_binding = args.resource_query.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:resourcegroups/group:Group".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurations".into(),
                    value: &configurations_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceQuery".into(),
                    value: &resource_query_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GroupResult {
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            configurations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurations"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_query: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceQuery"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}

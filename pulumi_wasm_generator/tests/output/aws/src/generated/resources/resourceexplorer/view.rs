/// Provides a resource to manage a Resource Explorer view.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = index::create(
///         "example",
///         IndexArgs::builder().type_("LOCAL").build_struct(),
///     );
///     let exampleView = view::create(
///         "exampleView",
///         ViewArgs::builder()
///             .filters(
///                 ViewFilters::builder()
///                     .filterString("resourcetype:ec2:instance")
///                     .build_struct(),
///             )
///             .included_properties(
///                 vec![ViewIncludedProperty::builder().name("tags").build_struct(),],
///             )
///             .name("exampleview")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Resource Explorer views using the `arn`. For example:
///
/// ```sh
/// $ pulumi import aws:resourceexplorer/view:View example arn:aws:resource-explorer-2:us-west-2:123456789012:view/exampleview/e0914f6c-6c27-4b47-b5d4-6b28381a2421
/// ```
pub mod view {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ViewArgs {
        /// Specifies whether the view is the [_default view_](https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-views-about.html#manage-views-about-default) for the AWS Region. Default: `false`.
        #[builder(into, default)]
        pub default_view: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies which resources are included in the results of queries made using this view. See Filters below for more details.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<super::super::types::resourceexplorer::ViewFilters>,
        >,
        /// Optional fields to be included in search results from this view. See Included Properties below for more details.
        #[builder(into, default)]
        pub included_properties: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::resourceexplorer::ViewIncludedProperty>>,
        >,
        /// The name of the view. The name must be no more than 64 characters long, and can include letters, digits, and the dash (-) character. The name must be unique within its AWS Region.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The root ARN of the account, an organizational unit (OU), or an organization ARN. If left empty, the default is account.
        #[builder(into, default)]
        pub scope: pulumi_wasm_rust::Output<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ViewResult {
        /// Amazon Resource Name (ARN) of the Resource Explorer view.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Specifies whether the view is the [_default view_](https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-views-about.html#manage-views-about-default) for the AWS Region. Default: `false`.
        pub default_view: pulumi_wasm_rust::Output<bool>,
        /// Specifies which resources are included in the results of queries made using this view. See Filters below for more details.
        pub filters: pulumi_wasm_rust::Output<
            Option<super::super::types::resourceexplorer::ViewFilters>,
        >,
        /// Optional fields to be included in search results from this view. See Included Properties below for more details.
        pub included_properties: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::resourceexplorer::ViewIncludedProperty>>,
        >,
        /// The name of the view. The name must be no more than 64 characters long, and can include letters, digits, and the dash (-) character. The name must be unique within its AWS Region.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The root ARN of the account, an organizational unit (OU), or an organization ARN. If left empty, the default is account.
        pub scope: pulumi_wasm_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
    pub fn create(name: &str, args: ViewArgs) -> ViewResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let default_view_binding = args.default_view.get_inner();
        let filters_binding = args.filters.get_inner();
        let included_properties_binding = args.included_properties.get_inner();
        let name_binding = args.name.get_inner();
        let scope_binding = args.scope.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:resourceexplorer/view:View".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "defaultView".into(),
                    value: &default_view_binding,
                },
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "includedProperties".into(),
                    value: &included_properties_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "scope".into(),
                    value: &scope_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "defaultView".into(),
                },
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "includedProperties".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "scope".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ViewResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            default_view: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("defaultView").unwrap(),
            ),
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            included_properties: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("includedProperties").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            scope: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("scope").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
        }
    }
}
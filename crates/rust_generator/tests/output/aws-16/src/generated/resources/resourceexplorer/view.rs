/// Provides a resource to manage a Resource Explorer view.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod view {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ViewArgs {
        /// Specifies whether the view is the [_default view_](https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-views-about.html#manage-views-about-default) for the AWS Region. Default: `false`.
        #[builder(into, default)]
        pub default_view: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies which resources are included in the results of queries made using this view. See Filters below for more details.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::resourceexplorer::ViewFilters>,
        >,
        /// Optional fields to be included in search results from this view. See Included Properties below for more details.
        #[builder(into, default)]
        pub included_properties: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::resourceexplorer::ViewIncludedProperty>>,
        >,
        /// The name of the view. The name must be no more than 64 characters long, and can include letters, digits, and the dash (-) character. The name must be unique within its AWS Region.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The root ARN of the account, an organizational unit (OU), or an organization ARN. If left empty, the default is account.
        #[builder(into, default)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ViewResult {
        /// Amazon Resource Name (ARN) of the Resource Explorer view.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether the view is the [_default view_](https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-views-about.html#manage-views-about-default) for the AWS Region. Default: `false`.
        pub default_view: pulumi_gestalt_rust::Output<bool>,
        /// Specifies which resources are included in the results of queries made using this view. See Filters below for more details.
        pub filters: pulumi_gestalt_rust::Output<
            Option<super::super::types::resourceexplorer::ViewFilters>,
        >,
        /// Optional fields to be included in search results from this view. See Included Properties below for more details.
        pub included_properties: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::resourceexplorer::ViewIncludedProperty>>,
        >,
        /// The name of the view. The name must be no more than 64 characters long, and can include letters, digits, and the dash (-) character. The name must be unique within its AWS Region.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The root ARN of the account, an organizational unit (OU), or an organization ARN. If left empty, the default is account.
        pub scope: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ViewArgs,
    ) -> ViewResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let default_view_binding = args.default_view.get_output(context).get_inner();
        let filters_binding = args.filters.get_output(context).get_inner();
        let included_properties_binding = args
            .included_properties
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let scope_binding = args.scope.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:resourceexplorer/view:View".into(),
            name: name.to_string(),
            version: super::super::get_version(),
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
        };
        let o = register_interface::register(context.get_inner(), &request);
        ViewResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            default_view: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("defaultView"),
            ),
            filters: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("filters"),
            ),
            included_properties: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("includedProperties"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            scope: pulumi_gestalt_rust::__private::into_domain(o.extract_field("scope")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
        }
    }
}

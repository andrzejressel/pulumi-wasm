/// Resource for managing a QuickSight Theme.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = theme::create(
///         "example",
///         ThemeArgs::builder()
///             .base_theme_id("MIDNIGHT")
///             .configuration(
///                 ThemeConfiguration::builder()
///                     .dataColorPalette(
///                         ThemeConfigurationDataColorPalette::builder()
///                             .colors(
///                                 vec![
///                                     "#FFFFFF", "#111111", "#222222", "#333333", "#444444",
///                                     "#555555", "#666666", "#777777", "#888888", "#999999",
///                                 ],
///                             )
///                             .emptyFillColor("#FFFFFF")
///                             .minMaxGradients(vec!["#FFFFFF", "#111111",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("example")
///             .theme_id("example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import a QuickSight Theme using the AWS account ID and theme ID separated by a comma (`,`). For example:
///
/// ```sh
/// $ pulumi import aws:quicksight/theme:Theme example 123456789012,example-id
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod theme {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ThemeArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the theme that a custom theme will inherit from. All themes inherit from one of the starting themes defined by Amazon QuickSight. For a list of the starting themes, use ListThemes or choose Themes from within an analysis.
        #[builder(into)]
        pub base_theme_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The theme configuration, which contains the theme display properties. See configuration.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::quicksight::ThemeConfiguration>,
        >,
        /// Display name of the theme.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A set of resource permissions on the theme. Maximum of 64 items. See permissions.
        #[builder(into, default)]
        pub permissions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::quicksight::ThemePermission>>,
        >,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the theme.
        #[builder(into)]
        pub theme_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A description of the current theme version being created/updated.
        #[builder(into, default)]
        pub version_description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct ThemeResult {
        /// ARN of the theme.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// AWS account ID.
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the theme that a custom theme will inherit from. All themes inherit from one of the starting themes defined by Amazon QuickSight. For a list of the starting themes, use ListThemes or choose Themes from within an analysis.
        pub base_theme_id: pulumi_gestalt_rust::Output<String>,
        /// The theme configuration, which contains the theme display properties. See configuration.
        ///
        /// The following arguments are optional:
        pub configuration: pulumi_gestalt_rust::Output<
            Option<super::super::types::quicksight::ThemeConfiguration>,
        >,
        /// The time that the theme was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// The time that the theme was last updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// Display name of the theme.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A set of resource permissions on the theme. Maximum of 64 items. See permissions.
        pub permissions: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::quicksight::ThemePermission>>,
        >,
        /// The theme creation status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Identifier of the theme.
        pub theme_id: pulumi_gestalt_rust::Output<String>,
        /// A description of the current theme version being created/updated.
        pub version_description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The version number of the theme version.
        pub version_number: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ThemeArgs,
    ) -> ThemeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding_1 = args.aws_account_id.get_output(context);
        let aws_account_id_binding = aws_account_id_binding_1.get_inner();
        let base_theme_id_binding_1 = args.base_theme_id.get_output(context);
        let base_theme_id_binding = base_theme_id_binding_1.get_inner();
        let configuration_binding_1 = args.configuration.get_output(context);
        let configuration_binding = configuration_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let permissions_binding_1 = args.permissions.get_output(context);
        let permissions_binding = permissions_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let theme_id_binding_1 = args.theme_id.get_output(context);
        let theme_id_binding = theme_id_binding_1.get_inner();
        let version_description_binding_1 = args.version_description.get_output(context);
        let version_description_binding = version_description_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:quicksight/theme:Theme".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "baseThemeId".into(),
                    value: &base_theme_id_binding,
                },
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "permissions".into(),
                    value: &permissions_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "themeId".into(),
                    value: &theme_id_binding,
                },
                register_interface::ObjectField {
                    name: "versionDescription".into(),
                    value: &version_description_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ThemeResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            aws_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            base_theme_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("baseThemeId"),
            ),
            configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configuration"),
            ),
            created_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            last_updated_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("lastUpdatedTime"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            permissions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("permissions"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            tags_all: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("tagsAll"),
            ),
            theme_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("themeId"),
            ),
            version_description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionDescription"),
            ),
            version_number: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("versionNumber"),
            ),
        }
    }
}

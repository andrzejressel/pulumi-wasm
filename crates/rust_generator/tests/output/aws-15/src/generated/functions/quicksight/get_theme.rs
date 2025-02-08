#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_theme {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetThemeArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the theme.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub theme_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetThemeResult {
        /// ARN of the theme.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub aws_account_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the theme that a custom theme will inherit from. All themes inherit from one of the starting themes defined by Amazon QuickSight.
        pub base_theme_id: pulumi_gestalt_rust::Output<String>,
        /// The theme configuration, which contains the theme display properties. See configuration.
        pub configurations: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::quicksight::GetThemeConfiguration>,
        >,
        /// The time that the theme was created.
        pub created_time: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The time that the theme was last updated.
        pub last_updated_time: pulumi_gestalt_rust::Output<String>,
        /// Display name of the theme.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A set of resource permissions on the theme. See permissions.
        pub permissions: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::quicksight::GetThemePermission>,
        >,
        /// The theme creation status.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub theme_id: pulumi_gestalt_rust::Output<String>,
        /// A description of the current theme version being created/updated.
        pub version_description: pulumi_gestalt_rust::Output<String>,
        /// The version number of the theme version.
        pub version_number: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetThemeArgs,
    ) -> GetThemeResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let aws_account_id_binding = args.aws_account_id.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let theme_id_binding = args.theme_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:quicksight/getTheme:getTheme".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "awsAccountId".into(),
                    value: &aws_account_id_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "themeId".into(),
                    value: &theme_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetThemeResult {
            arn: pulumi_gestalt_rust::__private::into_domain(o.extract_field("arn")),
            aws_account_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("awsAccountId"),
            ),
            base_theme_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("baseThemeId"),
            ),
            configurations: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurations"),
            ),
            created_time: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("createdTime"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
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

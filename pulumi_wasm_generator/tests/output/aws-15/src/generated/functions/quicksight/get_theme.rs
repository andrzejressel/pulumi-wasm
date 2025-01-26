pub mod get_theme {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetThemeArgs {
        /// AWS account ID.
        #[builder(into, default)]
        pub aws_account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Identifier of the theme.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub theme_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetThemeResult {
        /// ARN of the theme.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub aws_account_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the theme that a custom theme will inherit from. All themes inherit from one of the starting themes defined by Amazon QuickSight.
        pub base_theme_id: pulumi_wasm_rust::Output<String>,
        /// The theme configuration, which contains the theme display properties. See configuration.
        pub configurations: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::quicksight::GetThemeConfiguration>,
        >,
        /// The time that the theme was created.
        pub created_time: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The time that the theme was last updated.
        pub last_updated_time: pulumi_wasm_rust::Output<String>,
        /// Display name of the theme.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A set of resource permissions on the theme. See permissions.
        pub permissions: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::quicksight::GetThemePermission>,
        >,
        /// The theme creation status.
        pub status: pulumi_wasm_rust::Output<String>,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub theme_id: pulumi_wasm_rust::Output<String>,
        /// A description of the current theme version being created/updated.
        pub version_description: pulumi_wasm_rust::Output<String>,
        /// The version number of the theme version.
        pub version_number: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetThemeArgs,
    ) -> GetThemeResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "awsAccountId".into(),
                },
                register_interface::ResultField {
                    name: "baseThemeId".into(),
                },
                register_interface::ResultField {
                    name: "configurations".into(),
                },
                register_interface::ResultField {
                    name: "createdTime".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "lastUpdatedTime".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "permissions".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "themeId".into(),
                },
                register_interface::ResultField {
                    name: "versionDescription".into(),
                },
                register_interface::ResultField {
                    name: "versionNumber".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetThemeResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            aws_account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("awsAccountId").unwrap(),
            ),
            base_theme_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("baseThemeId").unwrap(),
            ),
            configurations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurations").unwrap(),
            ),
            created_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createdTime").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            last_updated_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("lastUpdatedTime").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            permissions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("permissions").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            theme_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("themeId").unwrap(),
            ),
            version_description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionDescription").unwrap(),
            ),
            version_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("versionNumber").unwrap(),
            ),
        }
    }
}

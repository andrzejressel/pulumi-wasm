/// Provides an AppConfig Extension resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testTopic:
///     type: aws:sns:Topic
///     name: test
///     properties:
///       name: test
///   testRole:
///     type: aws:iam:Role
///     name: test
///     properties:
///       name: test
///       assumeRolePolicy: ${test.json}
///   testExtension:
///     type: aws:appconfig:Extension
///     name: test
///     properties:
///       name: test
///       description: test description
///       actionPoints:
///         - point: ON_DEPLOYMENT_COMPLETE
///           actions:
///             - name: test
///               roleArn: ${testRole.arn}
///               uri: ${testTopic.arn}
///       tags:
///         Type: AppConfig Extension
/// variables:
///   test:
///     fn::invoke:
///       Function: aws:iam:getPolicyDocument
///       Arguments:
///         statements:
///           - actions:
///               - sts:AssumeRole
///             principals:
///               - type: Service
///                 identifiers:
///                   - appconfig.amazonaws.com
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppConfig Extensions using their extension ID. For example:
///
/// ```sh
/// $ pulumi import aws:appconfig/extension:Extension example 71rxuzt
/// ```
pub mod extension {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExtensionArgs {
        /// The action points defined in the extension. Detailed below.
        #[builder(into)]
        pub action_points: pulumi_wasm_rust::Output<
            Vec<super::super::types::appconfig::ExtensionActionPoint>,
        >,
        /// Information about the extension.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// A name for the extension. Each extension name in your account must be unique. Extension versions use the same name.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The parameters accepted by the extension. You specify parameter values when you associate the extension to an AppConfig resource by using the CreateExtensionAssociation API action. For Lambda extension actions, these parameters are included in the Lambda request object. Detailed below.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::types::appconfig::ExtensionParameter>>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ExtensionResult {
        /// The action points defined in the extension. Detailed below.
        pub action_points: pulumi_wasm_rust::Output<
            Vec<super::super::types::appconfig::ExtensionActionPoint>,
        >,
        /// ARN of the AppConfig Extension.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Information about the extension.
        pub description: pulumi_wasm_rust::Output<String>,
        /// A name for the extension. Each extension name in your account must be unique. Extension versions use the same name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The parameters accepted by the extension. You specify parameter values when you associate the extension to an AppConfig resource by using the CreateExtensionAssociation API action. For Lambda extension actions, these parameters are included in the Lambda request object. Detailed below.
        pub parameters: pulumi_wasm_rust::Output<
            Vec<super::super::types::appconfig::ExtensionParameter>,
        >,
        /// Map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The version number for the extension.
        pub version: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: ExtensionArgs) -> ExtensionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_points_binding = args.action_points.get_inner();
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let tags_binding = args.tags.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appconfig/extension:Extension".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actionPoints".into(),
                    value: &action_points_binding,
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
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "actionPoints".into(),
                },
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "tagsAll".into(),
                },
                register_interface::ResultField {
                    name: "version".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExtensionResult {
            action_points: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actionPoints").unwrap(),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tags_all: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tagsAll").unwrap(),
            ),
            version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("version").unwrap(),
            ),
        }
    }
}
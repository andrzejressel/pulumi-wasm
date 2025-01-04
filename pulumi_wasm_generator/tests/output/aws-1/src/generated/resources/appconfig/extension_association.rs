/// Associates an AppConfig Extension with a Resource.
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
///   testApplication:
///     type: aws:appconfig:Application
///     name: test
///     properties:
///       name: test
///   testExtensionAssociation:
///     type: aws:appconfig:ExtensionAssociation
///     name: test
///     properties:
///       extensionArn: ${testExtension.arn}
///       resourceArn: ${testApplication.arn}
/// variables:
///   test:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
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
/// Using `pulumi import`, import AppConfig Extension Associations using their extension association ID. For example:
///
/// ```sh
/// $ pulumi import aws:appconfig/extensionAssociation:ExtensionAssociation example 71rxuzt
/// ```
pub mod extension_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExtensionAssociationArgs {
        /// The ARN of the extension defined in the association.
        #[builder(into)]
        pub extension_arn: pulumi_wasm_rust::Output<String>,
        /// The parameter names and values defined for the association.
        #[builder(into, default)]
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the application, configuration profile, or environment to associate with the extension.
        #[builder(into)]
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct ExtensionAssociationResult {
        /// ARN of the AppConfig Extension Association.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the extension defined in the association.
        pub extension_arn: pulumi_wasm_rust::Output<String>,
        /// The version number for the extension defined in the association.
        pub extension_version: pulumi_wasm_rust::Output<i32>,
        /// The parameter names and values defined for the association.
        pub parameters: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the application, configuration profile, or environment to associate with the extension.
        pub resource_arn: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: ExtensionAssociationArgs,
    ) -> ExtensionAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let extension_arn_binding = args.extension_arn.get_inner();
        let parameters_binding = args.parameters.get_inner();
        let resource_arn_binding = args.resource_arn.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appconfig/extensionAssociation:ExtensionAssociation".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "extensionArn".into(),
                    value: &extension_arn_binding,
                },
                register_interface::ObjectField {
                    name: "parameters".into(),
                    value: &parameters_binding,
                },
                register_interface::ObjectField {
                    name: "resourceArn".into(),
                    value: &resource_arn_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "extensionArn".into(),
                },
                register_interface::ResultField {
                    name: "extensionVersion".into(),
                },
                register_interface::ResultField {
                    name: "parameters".into(),
                },
                register_interface::ResultField {
                    name: "resourceArn".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        ExtensionAssociationResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            extension_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extensionArn").unwrap(),
            ),
            extension_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("extensionVersion").unwrap(),
            ),
            parameters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("parameters").unwrap(),
            ),
            resource_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceArn").unwrap(),
            ),
        }
    }
}

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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod extension_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ExtensionAssociationArgs {
        /// The ARN of the extension defined in the association.
        #[builder(into)]
        pub extension_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The parameter names and values defined for the association.
        #[builder(into, default)]
        pub parameters: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the application, configuration profile, or environment to associate with the extension.
        #[builder(into)]
        pub resource_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ExtensionAssociationResult {
        /// ARN of the AppConfig Extension Association.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the extension defined in the association.
        pub extension_arn: pulumi_gestalt_rust::Output<String>,
        /// The version number for the extension defined in the association.
        pub extension_version: pulumi_gestalt_rust::Output<i32>,
        /// The parameter names and values defined for the association.
        pub parameters: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The ARN of the application, configuration profile, or environment to associate with the extension.
        pub resource_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ExtensionAssociationArgs,
    ) -> ExtensionAssociationResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let extension_arn_binding = args.extension_arn.get_output(context);
        let parameters_binding = args.parameters.get_output(context);
        let resource_arn_binding = args.resource_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:appconfig/extensionAssociation:ExtensionAssociation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "extensionArn".into(),
                    value: extension_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameters".into(),
                    value: parameters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceArn".into(),
                    value: resource_arn_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ExtensionAssociationResult {
            arn: o.get_field("arn"),
            extension_arn: o.get_field("extensionArn"),
            extension_version: o.get_field("extensionVersion"),
            parameters: o.get_field("parameters"),
            resource_arn: o.get_field("resourceArn"),
        }
    }
}

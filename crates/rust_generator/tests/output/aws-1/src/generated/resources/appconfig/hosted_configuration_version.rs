/// Provides an AppConfig Hosted Configuration Version resource.
///
/// ## Example Usage
///
/// ### Freeform
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appconfig:HostedConfigurationVersion
///     properties:
///       applicationId: ${exampleAwsAppconfigApplication.id}
///       configurationProfileId: ${exampleAwsAppconfigConfigurationProfile.configurationProfileId}
///       description: Example Freeform Hosted Configuration Version
///       contentType: application/json
///       content:
///         fn::toJSON:
///           foo: bar
///           fruit:
///             - apple
///             - pear
///             - orange
///           isThingEnabled: true
/// ```
///
/// ### Feature Flags
///
/// ```yaml
/// resources:
///   example:
///     type: aws:appconfig:HostedConfigurationVersion
///     properties:
///       applicationId: ${exampleAwsAppconfigApplication.id}
///       configurationProfileId: ${exampleAwsAppconfigConfigurationProfile.configurationProfileId}
///       description: Example Feature Flag Configuration Version
///       contentType: application/json
///       content:
///         fn::toJSON:
///           flags:
///             foo:
///               name: foo
///               _deprecation:
///                 status: planned
///             bar:
///               name: bar
///               attributes:
///                 someAttribute:
///                   constraints:
///                     type: string
///                     required: true
///                 someOtherAttribute:
///                   constraints:
///                     type: number
///                     required: true
///           values:
///             foo:
///               enabled: 'true'
///             bar:
///               enabled: 'true'
///               someAttribute: Hello World
///               someOtherAttribute: 123
///           version: '1'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AppConfig Hosted Configuration Versions using the application ID, configuration profile ID, and version number separated by a slash (`/`). For example:
///
/// ```sh
/// $ pulumi import aws:appconfig/hostedConfigurationVersion:HostedConfigurationVersion example 71abcde/11xxxxx/2
/// ```
pub mod hosted_configuration_version {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostedConfigurationVersionArgs {
        /// Application ID.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Configuration profile ID.
        #[builder(into)]
        pub configuration_profile_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Content of the configuration or the configuration data.
        #[builder(into)]
        pub content: pulumi_wasm_rust::InputOrOutput<String>,
        /// Standard MIME type describing the format of the configuration content. For more information, see [Content-Type](https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17).
        #[builder(into)]
        pub content_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// Description of the configuration.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HostedConfigurationVersionResult {
        /// Application ID.
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the AppConfig  hosted configuration version.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Configuration profile ID.
        pub configuration_profile_id: pulumi_wasm_rust::Output<String>,
        /// Content of the configuration or the configuration data.
        pub content: pulumi_wasm_rust::Output<String>,
        /// Standard MIME type describing the format of the configuration content. For more information, see [Content-Type](https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17).
        pub content_type: pulumi_wasm_rust::Output<String>,
        /// Description of the configuration.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// Version number of the hosted configuration.
        pub version_number: pulumi_wasm_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: HostedConfigurationVersionArgs,
    ) -> HostedConfigurationVersionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let configuration_profile_id_binding = args
            .configuration_profile_id
            .get_output(context)
            .get_inner();
        let content_binding = args.content.get_output(context).get_inner();
        let content_type_binding = args.content_type.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:appconfig/hostedConfigurationVersion:HostedConfigurationVersion"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding,
                },
                register_interface::ObjectField {
                    name: "configurationProfileId".into(),
                    value: &configuration_profile_id_binding,
                },
                register_interface::ObjectField {
                    name: "content".into(),
                    value: &content_binding,
                },
                register_interface::ObjectField {
                    name: "contentType".into(),
                    value: &content_type_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        HostedConfigurationVersionResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            configuration_profile_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationProfileId"),
            ),
            content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("content"),
            ),
            content_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contentType"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            version_number: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionNumber"),
            ),
        }
    }
}

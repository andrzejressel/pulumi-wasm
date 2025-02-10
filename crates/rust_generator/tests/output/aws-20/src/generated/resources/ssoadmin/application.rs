/// Resource for managing an AWS SSO Admin Application.
///
/// > The `CreateApplication` API only supports custom OAuth 2.0 applications.
/// Creation of 3rd party SAML or OAuth 2.0 applications require setup to be done through the associated app service or AWS console.
/// See this issue for additional context.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   exampleApplication:
///     type: aws:ssoadmin:Application
///     name: example
///     properties:
///       name: example
///       applicationProviderArn: arn:aws:sso::aws:applicationProvider/custom
///       instanceArn: ${example.arns[0]}
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
/// ```
///
/// ### With Portal Options
///
/// ```yaml
/// resources:
///   exampleApplication:
///     type: aws:ssoadmin:Application
///     name: example
///     properties:
///       name: example
///       applicationProviderArn: arn:aws:sso::aws:applicationProvider/custom
///       instanceArn: ${example.arns[0]}
///       portalOptions:
///         visibility: ENABLED
///         signInOptions:
///           applicationUrl: http://example.com
///           origin: APPLICATION
/// variables:
///   example:
///     fn::invoke:
///       function: aws:ssoadmin:getInstances
///       arguments: {}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSO Admin Application using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssoadmin/application:Application example arn:aws:sso::123456789012:application/id-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApplicationArgs {
        /// ARN of the application provider.
        #[builder(into)]
        pub application_provider_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
        #[builder(into, default)]
        pub client_token: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the application.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ARN of the instance of IAM Identity Center.
        #[builder(into)]
        pub instance_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the application.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Options for the portal associated with an application. See `portal_options` below.
        #[builder(into, default)]
        pub portal_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::ssoadmin::ApplicationPortalOptions>,
        >,
        /// Status of the application. Valid values are `ENABLED` and `DISABLED`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ApplicationResult {
        /// AWS account ID.
        pub application_account: pulumi_gestalt_rust::Output<String>,
        /// ARN of the application.
        pub application_arn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the application provider.
        pub application_provider_arn: pulumi_gestalt_rust::Output<String>,
        /// A unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
        pub client_token: pulumi_gestalt_rust::Output<Option<String>>,
        /// Description of the application.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// ARN of the instance of IAM Identity Center.
        pub instance_arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the application.
        ///
        /// The following arguments are optional:
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Options for the portal associated with an application. See `portal_options` below.
        pub portal_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::ssoadmin::ApplicationPortalOptions>,
        >,
        /// Status of the application. Valid values are `ENABLED` and `DISABLED`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ApplicationArgs,
    ) -> ApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_provider_arn_binding = args
            .application_provider_arn
            .get_output(context);
        let client_token_binding = args.client_token.get_output(context);
        let description_binding = args.description.get_output(context);
        let instance_arn_binding = args.instance_arn.get_output(context);
        let name_binding = args.name.get_output(context);
        let portal_options_binding = args.portal_options.get_output(context);
        let status_binding = args.status.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssoadmin/application:Application".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationProviderArn".into(),
                    value: application_provider_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "clientToken".into(),
                    value: client_token_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instanceArn".into(),
                    value: instance_arn_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "portalOptions".into(),
                    value: portal_options_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: status_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        ApplicationResult {
            application_account: o.get_field("applicationAccount"),
            application_arn: o.get_field("applicationArn"),
            application_provider_arn: o.get_field("applicationProviderArn"),
            client_token: o.get_field("clientToken"),
            description: o.get_field("description"),
            instance_arn: o.get_field("instanceArn"),
            name: o.get_field("name"),
            portal_options: o.get_field("portalOptions"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}

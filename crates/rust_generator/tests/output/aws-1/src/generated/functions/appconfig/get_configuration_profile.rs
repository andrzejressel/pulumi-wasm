#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_configuration_profile {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationProfileArgs {
        /// ID of the AppConfig application to which this configuration profile belongs.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// ID of the Configuration Profile.
        #[builder(into)]
        pub configuration_profile_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationProfileResult {
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// ARN of the Configuration Profile.
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub configuration_profile_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the Configuration Profile.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub kms_key_identifier: pulumi_gestalt_rust::Output<String>,
        /// Location URI of the Configuration Profile.
        pub location_uri: pulumi_gestalt_rust::Output<String>,
        /// Name of the Configuration Profile.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// ARN of an IAM role with permission to access the configuration at the specified location_uri.
        pub retrieval_role_arn: pulumi_gestalt_rust::Output<String>,
        /// Map of tags for the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of validator. Valid values: JSON_SCHEMA and LAMBDA.
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// Nested list of methods for validating the configuration.
        pub validators: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::appconfig::GetConfigurationProfileValidator>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConfigurationProfileArgs,
    ) -> GetConfigurationProfileResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let configuration_profile_id_binding = args
            .configuration_profile_id
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:appconfig/getConfigurationProfile:getConfigurationProfile"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: application_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationProfileId".into(),
                    value: configuration_profile_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConfigurationProfileResult {
            application_id: o.get_field("applicationId"),
            arn: o.get_field("arn"),
            configuration_profile_id: o.get_field("configurationProfileId"),
            description: o.get_field("description"),
            id: o.get_field("id"),
            kms_key_identifier: o.get_field("kmsKeyIdentifier"),
            location_uri: o.get_field("locationUri"),
            name: o.get_field("name"),
            retrieval_role_arn: o.get_field("retrievalRoleArn"),
            tags: o.get_field("tags"),
            type_: o.get_field("type"),
            validators: o.get_field("validators"),
        }
    }
}

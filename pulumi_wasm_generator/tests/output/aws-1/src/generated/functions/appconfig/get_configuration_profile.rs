pub mod get_configuration_profile {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationProfileArgs {
        /// ID of the AppConfig application to which this configuration profile belongs.
        #[builder(into)]
        pub application_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// ID of the Configuration Profile.
        #[builder(into)]
        pub configuration_profile_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Map of tags for the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationProfileResult {
        pub application_id: pulumi_wasm_rust::Output<String>,
        /// ARN of the Configuration Profile.
        pub arn: pulumi_wasm_rust::Output<String>,
        pub configuration_profile_id: pulumi_wasm_rust::Output<String>,
        /// Description of the Configuration Profile.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub kms_key_identifier: pulumi_wasm_rust::Output<String>,
        /// Location URI of the Configuration Profile.
        pub location_uri: pulumi_wasm_rust::Output<String>,
        /// Name of the Configuration Profile.
        pub name: pulumi_wasm_rust::Output<String>,
        /// ARN of an IAM role with permission to access the configuration at the specified location_uri.
        pub retrieval_role_arn: pulumi_wasm_rust::Output<String>,
        /// Map of tags for the resource.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// Type of validator. Valid values: JSON_SCHEMA and LAMBDA.
        pub type_: pulumi_wasm_rust::Output<String>,
        /// Nested list of methods for validating the configuration.
        pub validators: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::appconfig::GetConfigurationProfileValidator>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetConfigurationProfileArgs,
    ) -> GetConfigurationProfileResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let application_id_binding = args.application_id.get_output(context).get_inner();
        let configuration_profile_id_binding = args
            .configuration_profile_id
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:appconfig/getConfigurationProfile:getConfigurationProfile"
                .into(),
            version: super::super::super::get_version(),
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
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetConfigurationProfileResult {
            application_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("applicationId"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            configuration_profile_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("configurationProfileId"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            kms_key_identifier: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("kmsKeyIdentifier"),
            ),
            location_uri: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("locationUri"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            retrieval_role_arn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("retrievalRoleArn"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            validators: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("validators"),
            ),
        }
    }
}

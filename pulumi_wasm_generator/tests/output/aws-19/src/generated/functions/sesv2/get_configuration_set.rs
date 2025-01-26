pub mod get_configuration_set {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationSetArgs {
        /// The name of the configuration set.
        #[builder(into)]
        pub configuration_set_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the container recipe.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationSetResult {
        pub arn: pulumi_wasm_rust::Output<String>,
        pub configuration_set_name: pulumi_wasm_rust::Output<String>,
        /// An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set.
        pub delivery_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetDeliveryOption>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set.
        pub reputation_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetReputationOption>,
        >,
        /// An object that defines whether or not Amazon SES can send email that you send using the configuration set.
        pub sending_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetSendingOption>,
        >,
        /// An object that contains information about the suppression list preferences for your account.
        pub suppression_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetSuppressionOption>,
        >,
        /// Key-value map of resource tags for the container recipe.
        pub tags: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        /// An object that defines the open and click tracking options for emails that you send using the configuration set.
        pub tracking_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetTrackingOption>,
        >,
        /// An object that contains information about the VDM preferences for your configuration set.
        pub vdm_options: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetVdmOption>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetConfigurationSetArgs,
    ) -> GetConfigurationSetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_set_name_binding = args
            .configuration_set_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:sesv2/getConfigurationSet:getConfigurationSet".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configurationSetName".into(),
                    value: &configuration_set_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "arn".into(),
                },
                register_interface::ResultField {
                    name: "configurationSetName".into(),
                },
                register_interface::ResultField {
                    name: "deliveryOptions".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "reputationOptions".into(),
                },
                register_interface::ResultField {
                    name: "sendingOptions".into(),
                },
                register_interface::ResultField {
                    name: "suppressionOptions".into(),
                },
                register_interface::ResultField {
                    name: "tags".into(),
                },
                register_interface::ResultField {
                    name: "trackingOptions".into(),
                },
                register_interface::ResultField {
                    name: "vdmOptions".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetConfigurationSetResult {
            arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("arn").unwrap(),
            ),
            configuration_set_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configurationSetName").unwrap(),
            ),
            delivery_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deliveryOptions").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            reputation_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("reputationOptions").unwrap(),
            ),
            sending_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sendingOptions").unwrap(),
            ),
            suppression_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("suppressionOptions").unwrap(),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tags").unwrap(),
            ),
            tracking_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("trackingOptions").unwrap(),
            ),
            vdm_options: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vdmOptions").unwrap(),
            ),
        }
    }
}

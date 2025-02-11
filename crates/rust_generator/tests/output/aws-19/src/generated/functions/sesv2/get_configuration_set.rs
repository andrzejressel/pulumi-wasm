#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_configuration_set {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetConfigurationSetArgs {
        /// The name of the configuration set.
        #[builder(into)]
        pub configuration_set_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Key-value map of resource tags for the container recipe.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetConfigurationSetResult {
        pub arn: pulumi_gestalt_rust::Output<String>,
        pub configuration_set_name: pulumi_gestalt_rust::Output<String>,
        /// An object that defines the dedicated IP pool that is used to send emails that you send using the configuration set.
        pub delivery_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetDeliveryOption>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// An object that defines whether or not Amazon SES collects reputation metrics for the emails that you send that use the configuration set.
        pub reputation_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetReputationOption>,
        >,
        /// An object that defines whether or not Amazon SES can send email that you send using the configuration set.
        pub sending_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetSendingOption>,
        >,
        /// An object that contains information about the suppression list preferences for your account.
        pub suppression_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetSuppressionOption>,
        >,
        /// Key-value map of resource tags for the container recipe.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// An object that defines the open and click tracking options for emails that you send using the configuration set.
        pub tracking_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetTrackingOption>,
        >,
        /// An object that contains information about the VDM preferences for your configuration set.
        pub vdm_options: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::sesv2::GetConfigurationSetVdmOption>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetConfigurationSetArgs,
    ) -> GetConfigurationSetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_set_name_binding = args
            .configuration_set_name
            .get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:sesv2/getConfigurationSet:getConfigurationSet".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configurationSetName".into(),
                    value: &configuration_set_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetConfigurationSetResult {
            arn: o.get_field("arn"),
            configuration_set_name: o.get_field("configurationSetName"),
            delivery_options: o.get_field("deliveryOptions"),
            id: o.get_field("id"),
            reputation_options: o.get_field("reputationOptions"),
            sending_options: o.get_field("sendingOptions"),
            suppression_options: o.get_field("suppressionOptions"),
            tags: o.get_field("tags"),
            tracking_options: o.get_field("trackingOptions"),
            vdm_options: o.get_field("vdmOptions"),
        }
    }
}

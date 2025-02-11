#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_instance_type_offerings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceTypeOfferingsArgs {
        /// Filter response by engine type.
        #[builder(into, default)]
        pub engine_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Filter response by host instance type.
        #[builder(into, default)]
        pub host_instance_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Filter response by storage type.
        #[builder(into, default)]
        pub storage_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceTypeOfferingsResult {
        /// Option for host instance type. See Broker Instance Options below.
        pub broker_instance_options: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::mq::GetInstanceTypeOfferingsBrokerInstanceOption,
            >,
        >,
        /// Broker's engine type.
        pub engine_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Broker's instance type.
        pub host_instance_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Broker's storage type.
        pub storage_type: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetInstanceTypeOfferingsArgs,
    ) -> GetInstanceTypeOfferingsResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let engine_type_binding = args.engine_type.get_output(context);
        let host_instance_type_binding = args.host_instance_type.get_output(context);
        let storage_type_binding = args.storage_type.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:mq/getInstanceTypeOfferings:getInstanceTypeOfferings".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "engineType".into(),
                    value: &engine_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostInstanceType".into(),
                    value: &host_instance_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageType".into(),
                    value: &storage_type_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetInstanceTypeOfferingsResult {
            broker_instance_options: o.get_field("brokerInstanceOptions"),
            engine_type: o.get_field("engineType"),
            host_instance_type: o.get_field("hostInstanceType"),
            id: o.get_field("id"),
            storage_type: o.get_field("storageType"),
        }
    }
}

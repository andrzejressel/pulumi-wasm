#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_contact_channel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContactChannelArgs {
        /// Amazon Resource Name (ARN) of the contact channel.
        #[builder(into)]
        pub arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetContactChannelResult {
        /// Whether the contact channel is activated.
        pub activation_status: pulumi_gestalt_rust::Output<String>,
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the AWS SSM Contact that the contact channel belongs to.
        pub contact_id: pulumi_gestalt_rust::Output<String>,
        /// Details used to engage the contact channel.
        pub delivery_addresses: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::ssmcontacts::GetContactChannelDeliveryAddress,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the contact channel.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Type of the contact channel.
        pub type_: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetContactChannelArgs,
    ) -> GetContactChannelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let arn_binding = args.arn.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ssmcontacts/getContactChannel:getContactChannel".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "arn".into(),
                    value: &arn_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetContactChannelResult {
            activation_status: o.get_field("activationStatus"),
            arn: o.get_field("arn"),
            contact_id: o.get_field("contactId"),
            delivery_addresses: o.get_field("deliveryAddresses"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            type_: o.get_field("type"),
        }
    }
}

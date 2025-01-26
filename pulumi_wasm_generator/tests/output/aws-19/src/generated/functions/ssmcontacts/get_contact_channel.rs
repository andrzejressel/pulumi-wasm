pub mod get_contact_channel {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetContactChannelArgs {
        /// Amazon Resource Name (ARN) of the contact channel.
        #[builder(into)]
        pub arn: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetContactChannelResult {
        /// Whether the contact channel is activated.
        pub activation_status: pulumi_wasm_rust::Output<String>,
        pub arn: pulumi_wasm_rust::Output<String>,
        /// Amazon Resource Name (ARN) of the AWS SSM Contact that the contact channel belongs to.
        pub contact_id: pulumi_wasm_rust::Output<String>,
        /// Details used to engage the contact channel.
        pub delivery_addresses: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::ssmcontacts::GetContactChannelDeliveryAddress,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the contact channel.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Type of the contact channel.
        pub type_: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetContactChannelArgs,
    ) -> GetContactChannelResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let arn_binding = args.arn.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ssmcontacts/getContactChannel:getContactChannel".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "arn".into(),
                    value: &arn_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetContactChannelResult {
            activation_status: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("activationStatus"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            contact_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contactId"),
            ),
            delivery_addresses: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deliveryAddresses"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
        }
    }
}

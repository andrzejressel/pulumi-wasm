#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_gateway {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetGatewayArgs {
        /// Name of the gateway to retrieve.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetGatewayResult {
        /// ASN on the Amazon side of the connection.
        pub amazon_side_asn: pulumi_gestalt_rust::Output<String>,
        /// ARN of the gateway.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// AWS Account ID of the gateway.
        pub owner_account_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetGatewayArgs,
    ) -> GetGatewayResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:directconnect/getGateway:getGateway".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetGatewayResult {
            amazon_side_asn: o.get_field("amazonSideAsn"),
            arn: o.get_field("arn"),
            id: o.get_field("id"),
            name: o.get_field("name"),
            owner_account_id: o.get_field("ownerAccountId"),
        }
    }
}

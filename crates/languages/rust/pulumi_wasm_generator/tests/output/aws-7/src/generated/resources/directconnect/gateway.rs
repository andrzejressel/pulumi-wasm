/// Provides a Direct Connect Gateway.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = gateway::create(
///         "example",
///         GatewayArgs::builder()
///             .amazon_side_asn("64512")
///             .name("tf-dxg-example")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Direct Connect Gateways using the gateway `id`. For example:
///
/// ```sh
/// $ pulumi import aws:directconnect/gateway:Gateway test abcd1234-dcba-5678-be23-cdef9876ab45
/// ```
pub mod gateway {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayArgs {
        /// The ASN to be configured on the Amazon side of the connection. The ASN must be in the private range of 64,512 to 65,534 or 4,200,000,000 to 4,294,967,294.
        #[builder(into)]
        pub amazon_side_asn: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the connection.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GatewayResult {
        /// The ASN to be configured on the Amazon side of the connection. The ASN must be in the private range of 64,512 to 65,534 or 4,200,000,000 to 4,294,967,294.
        pub amazon_side_asn: pulumi_wasm_rust::Output<String>,
        /// The ARN of the gateway.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The name of the connection.
        pub name: pulumi_wasm_rust::Output<String>,
        /// AWS Account ID of the gateway.
        pub owner_account_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: GatewayArgs,
    ) -> GatewayResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let amazon_side_asn_binding = args
            .amazon_side_asn
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:directconnect/gateway:Gateway".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "amazonSideAsn".into(),
                    value: &amazon_side_asn_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        GatewayResult {
            amazon_side_asn: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("amazonSideAsn"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            owner_account_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("ownerAccountId"),
            ),
        }
    }
}

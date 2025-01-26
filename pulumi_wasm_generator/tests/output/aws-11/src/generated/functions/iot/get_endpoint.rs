pub mod get_endpoint {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEndpointArgs {
        /// Endpoint type. Valid values: `iot:CredentialProvider`, `iot:Data`, `iot:Data-ATS`, `iot:Jobs`.
        #[builder(into, default)]
        pub endpoint_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEndpointResult {
        /// Endpoint based on `endpoint_type`:
        /// * No `endpoint_type`: Either `iot:Data` or `iot:Data-ATS` [depending on region](https://aws.amazon.com/blogs/iot/aws-iot-core-ats-endpoints/)
        /// * `iot:CredentialsProvider`: `IDENTIFIER.credentials.iot.REGION.amazonaws.com`
        /// * `iot:Data`: `IDENTIFIER.iot.REGION.amazonaws.com`
        /// * `iot:Data-ATS`: `IDENTIFIER-ats.iot.REGION.amazonaws.com`
        /// * `iot:Jobs`: `IDENTIFIER.jobs.iot.REGION.amazonaws.com`
        pub endpoint_address: pulumi_wasm_rust::Output<String>,
        pub endpoint_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetEndpointArgs,
    ) -> GetEndpointResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let endpoint_type_binding = args.endpoint_type.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:iot/getEndpoint:getEndpoint".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "endpointType".into(),
                    value: &endpoint_type_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetEndpointResult {
            endpoint_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpointAddress"),
            ),
            endpoint_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("endpointType"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
        }
    }
}

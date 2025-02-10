#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_endpoint {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEndpointArgs {
        /// Endpoint type. Valid values: `iot:CredentialProvider`, `iot:Data`, `iot:Data-ATS`, `iot:Jobs`.
        #[builder(into, default)]
        pub endpoint_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetEndpointResult {
        /// Endpoint based on `endpoint_type`:
        /// * No `endpoint_type`: Either `iot:Data` or `iot:Data-ATS` [depending on region](https://aws.amazon.com/blogs/iot/aws-iot-core-ats-endpoints/)
        /// * `iot:CredentialsProvider`: `IDENTIFIER.credentials.iot.REGION.amazonaws.com`
        /// * `iot:Data`: `IDENTIFIER.iot.REGION.amazonaws.com`
        /// * `iot:Data-ATS`: `IDENTIFIER-ats.iot.REGION.amazonaws.com`
        /// * `iot:Jobs`: `IDENTIFIER.jobs.iot.REGION.amazonaws.com`
        pub endpoint_address: pulumi_gestalt_rust::Output<String>,
        pub endpoint_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEndpointArgs,
    ) -> GetEndpointResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let endpoint_type_binding = args.endpoint_type.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:iot/getEndpoint:getEndpoint".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpointType".into(),
                    value: endpoint_type_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEndpointResult {
            endpoint_address: o.get_field("endpointAddress"),
            endpoint_type: o.get_field("endpointType"),
            id: o.get_field("id"),
        }
    }
}

pub mod get_peering_attachments {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetPeeringAttachmentsArgs {
        /// One or more configuration blocks containing name-values filters. Detailed below.
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetPeeringAttachmentsFilter,
                >,
            >,
        >,
    }
    #[allow(dead_code)]
    pub struct GetPeeringAttachmentsResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<
                Vec<
                    super::super::super::types::ec2transitgateway::GetPeeringAttachmentsFilter,
                >,
            >,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// A list of all attachments ids matching the filter. You can retrieve more information about the attachment using the [aws.ec2transitgateway.PeeringAttachment][2] data source, searching by identifier.
        pub ids: pulumi_wasm_rust::Output<Vec<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetPeeringAttachmentsArgs) -> GetPeeringAttachmentsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2transitgateway/getPeeringAttachments:getPeeringAttachments"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "filters".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ids".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetPeeringAttachmentsResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ids: pulumi_wasm_rust::__private::into_domain(hashmap.remove("ids").unwrap()),
        }
    }
}

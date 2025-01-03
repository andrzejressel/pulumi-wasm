pub mod get_provisioning_artifacts {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProvisioningArtifactsArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// Product identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub product_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetProvisioningArtifactsResult {
        pub accept_language: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub product_id: pulumi_wasm_rust::Output<String>,
        /// List with information about the provisioning artifacts. See details below.
        pub provisioning_artifact_details: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::servicecatalog::GetProvisioningArtifactsProvisioningArtifactDetail,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetProvisioningArtifactsArgs) -> GetProvisioningArtifactsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args.accept_language.get_inner();
        let product_id_binding = args.product_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:servicecatalog/getProvisioningArtifacts:getProvisioningArtifacts"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding,
                },
                register_interface::ObjectField {
                    name: "productId".into(),
                    value: &product_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "acceptLanguage".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "productId".into(),
                },
                register_interface::ResultField {
                    name: "provisioningArtifactDetails".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetProvisioningArtifactsResult {
            accept_language: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("acceptLanguage").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            product_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("productId").unwrap(),
            ),
            provisioning_artifact_details: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("provisioningArtifactDetails").unwrap(),
            ),
        }
    }
}

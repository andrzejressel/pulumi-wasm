pub mod get_detector {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDetectorArgs {
        /// ID of the detector.
        #[builder(into, default)]
        pub id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetDetectorResult {
        /// Current configuration of the detector features.
        pub features: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::guardduty::GetDetectorFeature>,
        >,
        /// The frequency of notifications sent about subsequent finding occurrences.
        pub finding_publishing_frequency: pulumi_wasm_rust::Output<String>,
        pub id: pulumi_wasm_rust::Output<String>,
        /// Service-linked role that grants GuardDuty access to the resources in the AWS account.
        pub service_role_arn: pulumi_wasm_rust::Output<String>,
        /// Current status of the detector.
        pub status: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetDetectorArgs) -> GetDetectorResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let id_binding = args.id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:guardduty/getDetector:getDetector".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "id".into(),
                    value: &id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "features".into(),
                },
                register_interface::ResultField {
                    name: "findingPublishingFrequency".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "serviceRoleArn".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetDetectorResult {
            features: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("features").unwrap(),
            ),
            finding_publishing_frequency: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("findingPublishingFrequency").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            service_role_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceRoleArn").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
        }
    }
}

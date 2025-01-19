pub mod get_finding_ids {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFindingIdsArgs {
        /// ID of the GuardDuty detector.
        #[builder(into)]
        pub detector_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetFindingIdsResult {
        pub detector_id: pulumi_wasm_rust::Output<String>,
        /// A list of finding IDs for the specified detector.
        pub finding_ids: pulumi_wasm_rust::Output<Vec<String>>,
        /// Indicates whether findings are present for the specified detector.
        pub has_findings: pulumi_wasm_rust::Output<bool>,
        pub id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetFindingIdsArgs) -> GetFindingIdsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let detector_id_binding = args.detector_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:guardduty/getFindingIds:getFindingIds".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "detectorId".into(),
                    value: &detector_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "detectorId".into(),
                },
                register_interface::ResultField {
                    name: "findingIds".into(),
                },
                register_interface::ResultField {
                    name: "hasFindings".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetFindingIdsResult {
            detector_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("detectorId").unwrap(),
            ),
            finding_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("findingIds").unwrap(),
            ),
            has_findings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("hasFindings").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
        }
    }
}

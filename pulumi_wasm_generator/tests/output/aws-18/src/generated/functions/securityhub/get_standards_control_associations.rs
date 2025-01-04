pub mod get_standards_control_associations {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetStandardsControlAssociationsArgs {
        /// The identifier of the control (identified with `SecurityControlId`, `SecurityControlArn`, or a mix of both parameters).
        #[builder(into)]
        pub security_control_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetStandardsControlAssociationsResult {
        pub id: pulumi_wasm_rust::Output<String>,
        /// ID of the security control.
        pub security_control_id: pulumi_wasm_rust::Output<String>,
        /// A list that provides the status and other details for each security control that applies to each enabled standard.
        /// See `standards_control_associations` below.
        pub standards_control_associations: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::securityhub::GetStandardsControlAssociationsStandardsControlAssociation,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        args: GetStandardsControlAssociationsArgs,
    ) -> GetStandardsControlAssociationsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let security_control_id_binding = args.security_control_id.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:securityhub/getStandardsControlAssociations:getStandardsControlAssociations"
                .into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "securityControlId".into(),
                    value: &security_control_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "securityControlId".into(),
                },
                register_interface::ResultField {
                    name: "standardsControlAssociations".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetStandardsControlAssociationsResult {
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            security_control_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("securityControlId").unwrap(),
            ),
            standards_control_associations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("standardsControlAssociations").unwrap(),
            ),
        }
    }
}

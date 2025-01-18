pub mod get_share {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetShareArgs {
        /// The ID of the Data Share account in which the Data Share is created.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The name of this Data Share.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct GetShareResult {
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// The description of the Data Share.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// The kind of the Data Share.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// The name of the snapshot schedule.
        pub name: pulumi_wasm_rust::Output<String>,
        /// A `snapshot_schedule` block as defined below.
        pub snapshot_schedules: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::datashare::GetShareSnapshotSchedule>,
        >,
        /// The terms of the Data Share.
        pub terms: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetShareArgs) -> GetShareResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_inner();
        let name_binding = args.name.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:datashare/getShare:getShare".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "snapshotSchedules".into(),
                },
                register_interface::ResultField {
                    name: "terms".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetShareResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            snapshot_schedules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotSchedules").unwrap(),
            ),
            terms: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("terms").unwrap(),
            ),
        }
    }
}

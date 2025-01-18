pub mod get_snapshot_ids {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotIdsArgs {
        /// One or more name/value pairs to filter off of. There are
        /// several valid keys, for a full reference, check out
        /// [describe-volumes in the AWS CLI reference][1].
        #[builder(into, default)]
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ebs::GetSnapshotIdsFilter>>,
        >,
        /// Returns the snapshots owned by the specified owner id. Multiple owners can be specified.
        #[builder(into, default)]
        pub owners: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// One or more AWS accounts IDs that can create volumes from the snapshot.
        #[builder(into, default)]
        pub restorable_by_user_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotIdsResult {
        pub filters: pulumi_wasm_rust::Output<
            Option<Vec<super::super::super::types::ebs::GetSnapshotIdsFilter>>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Set of EBS snapshot IDs, sorted by creation time in descending order.
        pub ids: pulumi_wasm_rust::Output<Vec<String>>,
        pub owners: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub restorable_by_user_ids: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSnapshotIdsArgs) -> GetSnapshotIdsResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filters_binding = args.filters.get_inner();
        let owners_binding = args.owners.get_inner();
        let restorable_by_user_ids_binding = args.restorable_by_user_ids.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ebs/getSnapshotIds:getSnapshotIds".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filters".into(),
                    value: &filters_binding,
                },
                register_interface::ObjectField {
                    name: "owners".into(),
                    value: &owners_binding,
                },
                register_interface::ObjectField {
                    name: "restorableByUserIds".into(),
                    value: &restorable_by_user_ids_binding,
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
                register_interface::ResultField {
                    name: "owners".into(),
                },
                register_interface::ResultField {
                    name: "restorableByUserIds".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSnapshotIdsResult {
            filters: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filters").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ids").unwrap(),
            ),
            owners: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("owners").unwrap(),
            ),
            restorable_by_user_ids: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("restorableByUserIds").unwrap(),
            ),
        }
    }
}

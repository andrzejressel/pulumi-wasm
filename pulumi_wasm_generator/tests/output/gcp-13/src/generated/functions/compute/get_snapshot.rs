pub mod get_snapshot {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetSnapshotArgs {
        /// A filter to retrieve the compute snapshot.
        /// See [API filter parameter documentation](https://cloud.google.com/compute/docs/reference/rest/v1/snapshots/list#body.QUERY_PARAMETERS.filter) for reference.
        /// If multiple compute snapshot match, either adjust the filter or specify `most_recent`. One of `name` or `filter` must be provided.
        /// If you want to use a regular expression, use the `eq` (equal) or `ne` (not equal) operator against a single un-parenthesized expression with or without quotes or against multiple parenthesized expressions. Example `sourceDisk eq '.*(.*/data-disk$).*'`. More details for golang Snapshots list call filters [here](https://pkg.go.dev/google.golang.org/api/compute/v1#SnapshotsListCall.Filter).
        #[builder(into, default)]
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// If `filter` is provided, ensures the most recent snapshot is returned when multiple compute snapshot match.
        ///
        /// - - -
        #[builder(into, default)]
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        /// The name of the compute snapshot. One of `name` or `filter` must be provided.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetSnapshotResult {
        pub chain_name: pulumi_wasm_rust::Output<String>,
        pub creation_timestamp: pulumi_wasm_rust::Output<String>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub disk_size_gb: pulumi_wasm_rust::Output<i32>,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub filter: pulumi_wasm_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub label_fingerprint: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub licenses: pulumi_wasm_rust::Output<Vec<String>>,
        pub most_recent: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub self_link: pulumi_wasm_rust::Output<String>,
        pub snapshot_encryption_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetSnapshotSnapshotEncryptionKey>,
        >,
        pub snapshot_id: pulumi_wasm_rust::Output<i32>,
        pub source_disk: pulumi_wasm_rust::Output<String>,
        pub source_disk_encryption_keys: pulumi_wasm_rust::Output<
            Vec<super::super::super::types::compute::GetSnapshotSourceDiskEncryptionKey>,
        >,
        pub storage_bytes: pulumi_wasm_rust::Output<i32>,
        pub storage_locations: pulumi_wasm_rust::Output<Vec<String>>,
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetSnapshotArgs) -> GetSnapshotResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let filter_binding = args.filter.get_inner();
        let most_recent_binding = args.most_recent.get_inner();
        let name_binding = args.name.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getSnapshot:getSnapshot".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "filter".into(),
                    value: &filter_binding,
                },
                register_interface::ObjectField {
                    name: "mostRecent".into(),
                    value: &most_recent_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "chainName".into(),
                },
                register_interface::ResultField {
                    name: "creationTimestamp".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "diskSizeGb".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "filter".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "labelFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "licenses".into(),
                },
                register_interface::ResultField {
                    name: "mostRecent".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "pulumiLabels".into(),
                },
                register_interface::ResultField {
                    name: "selfLink".into(),
                },
                register_interface::ResultField {
                    name: "snapshotEncryptionKeys".into(),
                },
                register_interface::ResultField {
                    name: "snapshotId".into(),
                },
                register_interface::ResultField {
                    name: "sourceDisk".into(),
                },
                register_interface::ResultField {
                    name: "sourceDiskEncryptionKeys".into(),
                },
                register_interface::ResultField {
                    name: "storageBytes".into(),
                },
                register_interface::ResultField {
                    name: "storageLocations".into(),
                },
                register_interface::ResultField {
                    name: "zone".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetSnapshotResult {
            chain_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("chainName").unwrap(),
            ),
            creation_timestamp: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("creationTimestamp").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            disk_size_gb: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("diskSizeGb").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            filter: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("filter").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            label_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labelFingerprint").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            licenses: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("licenses").unwrap(),
            ),
            most_recent: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mostRecent").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            pulumi_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pulumiLabels").unwrap(),
            ),
            self_link: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("selfLink").unwrap(),
            ),
            snapshot_encryption_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotEncryptionKeys").unwrap(),
            ),
            snapshot_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("snapshotId").unwrap(),
            ),
            source_disk: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDisk").unwrap(),
            ),
            source_disk_encryption_keys: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceDiskEncryptionKeys").unwrap(),
            ),
            storage_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageBytes").unwrap(),
            ),
            storage_locations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("storageLocations").unwrap(),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zone").unwrap(),
            ),
        }
    }
}

pub mod get_backup_vault {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetBackupVaultArgs {
        /// The id of Backup Vault resource.
        ///
        /// - - -
        #[builder(into)]
        pub backup_vault_id: pulumi_wasm_rust::Output<String>,
        /// The location in which the Backup Vault resource belongs.
        #[builder(into)]
        pub location: pulumi_wasm_rust::Output<String>,
        /// The project in which the resource belongs. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetBackupVaultResult {
        pub access_restriction: pulumi_wasm_rust::Output<String>,
        pub allow_missing: pulumi_wasm_rust::Output<bool>,
        pub annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub backup_count: pulumi_wasm_rust::Output<String>,
        pub backup_minimum_enforced_retention_duration: pulumi_wasm_rust::Output<String>,
        pub backup_vault_id: pulumi_wasm_rust::Output<String>,
        pub create_time: pulumi_wasm_rust::Output<String>,
        pub deletable: pulumi_wasm_rust::Output<bool>,
        pub description: pulumi_wasm_rust::Output<String>,
        pub effective_annotations: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub effective_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub effective_time: pulumi_wasm_rust::Output<String>,
        pub etag: pulumi_wasm_rust::Output<String>,
        pub force_delete: pulumi_wasm_rust::Output<bool>,
        pub force_update: pulumi_wasm_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub ignore_backup_plan_references: pulumi_wasm_rust::Output<bool>,
        pub ignore_inactive_datasources: pulumi_wasm_rust::Output<bool>,
        pub labels: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub location: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub project: pulumi_wasm_rust::Output<Option<String>>,
        pub pulumi_labels: pulumi_wasm_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub service_account: pulumi_wasm_rust::Output<String>,
        pub state: pulumi_wasm_rust::Output<String>,
        pub total_stored_bytes: pulumi_wasm_rust::Output<String>,
        pub uid: pulumi_wasm_rust::Output<String>,
        pub update_time: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetBackupVaultArgs) -> GetBackupVaultResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let backup_vault_id_binding = args.backup_vault_id.get_inner();
        let location_binding = args.location.get_inner();
        let project_binding = args.project.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:backupdisasterrecovery/getBackupVault:getBackupVault".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "backupVaultId".into(),
                    value: &backup_vault_id_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessRestriction".into(),
                },
                register_interface::ResultField {
                    name: "allowMissing".into(),
                },
                register_interface::ResultField {
                    name: "annotations".into(),
                },
                register_interface::ResultField {
                    name: "backupCount".into(),
                },
                register_interface::ResultField {
                    name: "backupMinimumEnforcedRetentionDuration".into(),
                },
                register_interface::ResultField {
                    name: "backupVaultId".into(),
                },
                register_interface::ResultField {
                    name: "createTime".into(),
                },
                register_interface::ResultField {
                    name: "deletable".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "effectiveAnnotations".into(),
                },
                register_interface::ResultField {
                    name: "effectiveLabels".into(),
                },
                register_interface::ResultField {
                    name: "effectiveTime".into(),
                },
                register_interface::ResultField {
                    name: "etag".into(),
                },
                register_interface::ResultField {
                    name: "forceDelete".into(),
                },
                register_interface::ResultField {
                    name: "forceUpdate".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "ignoreBackupPlanReferences".into(),
                },
                register_interface::ResultField {
                    name: "ignoreInactiveDatasources".into(),
                },
                register_interface::ResultField {
                    name: "labels".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
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
                    name: "serviceAccount".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "totalStoredBytes".into(),
                },
                register_interface::ResultField {
                    name: "uid".into(),
                },
                register_interface::ResultField {
                    name: "updateTime".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetBackupVaultResult {
            access_restriction: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessRestriction").unwrap(),
            ),
            allow_missing: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("allowMissing").unwrap(),
            ),
            annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("annotations").unwrap(),
            ),
            backup_count: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupCount").unwrap(),
            ),
            backup_minimum_enforced_retention_duration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupMinimumEnforcedRetentionDuration").unwrap(),
            ),
            backup_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("backupVaultId").unwrap(),
            ),
            create_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createTime").unwrap(),
            ),
            deletable: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("deletable").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            effective_annotations: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveAnnotations").unwrap(),
            ),
            effective_labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveLabels").unwrap(),
            ),
            effective_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("effectiveTime").unwrap(),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("etag").unwrap(),
            ),
            force_delete: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceDelete").unwrap(),
            ),
            force_update: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("forceUpdate").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            ignore_backup_plan_references: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ignoreBackupPlanReferences").unwrap(),
            ),
            ignore_inactive_datasources: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ignoreInactiveDatasources").unwrap(),
            ),
            labels: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("labels").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
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
            service_account: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccount").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            total_stored_bytes: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("totalStoredBytes").unwrap(),
            ),
            uid: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("uid").unwrap(),
            ),
            update_time: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updateTime").unwrap(),
            ),
        }
    }
}

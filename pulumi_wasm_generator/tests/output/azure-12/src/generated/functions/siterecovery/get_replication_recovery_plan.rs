pub mod get_replication_recovery_plan {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReplicationRecoveryPlanArgs {
        /// The name of the Replication Plan.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the vault that should be updated.
        #[builder(into)]
        pub recovery_vault_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetReplicationRecoveryPlanResult {
        pub azure_to_azure_settings: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::siterecovery::GetReplicationRecoveryPlanAzureToAzureSetting,
            >,
        >,
        pub failover_deployment_model: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        /// Name of the Action.
        pub name: pulumi_wasm_rust::Output<String>,
        /// `recovery_group` block defined as below.
        pub recovery_groups: pulumi_wasm_rust::Output<
            Vec<
                super::super::super::types::siterecovery::GetReplicationRecoveryPlanRecoveryGroup,
            >,
        >,
        pub recovery_vault_id: pulumi_wasm_rust::Output<String>,
        /// The ID of source fabric to be recovered from.
        pub source_recovery_fabric_id: pulumi_wasm_rust::Output<String>,
        /// The ID of target fabric to recover.
        pub target_recovery_fabric_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetReplicationRecoveryPlanArgs,
    ) -> GetReplicationRecoveryPlanResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let recovery_vault_id_binding = args
            .recovery_vault_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:siterecovery/getReplicationRecoveryPlan:getReplicationRecoveryPlan"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryVaultId".into(),
                    value: &recovery_vault_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "azureToAzureSettings".into(),
                },
                register_interface::ResultField {
                    name: "failoverDeploymentModel".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "recoveryGroups".into(),
                },
                register_interface::ResultField {
                    name: "recoveryVaultId".into(),
                },
                register_interface::ResultField {
                    name: "sourceRecoveryFabricId".into(),
                },
                register_interface::ResultField {
                    name: "targetRecoveryFabricId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetReplicationRecoveryPlanResult {
            azure_to_azure_settings: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("azureToAzureSettings").unwrap(),
            ),
            failover_deployment_model: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("failoverDeploymentModel").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            recovery_groups: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryGroups").unwrap(),
            ),
            recovery_vault_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("recoveryVaultId").unwrap(),
            ),
            source_recovery_fabric_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sourceRecoveryFabricId").unwrap(),
            ),
            target_recovery_fabric_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targetRecoveryFabricId").unwrap(),
            ),
        }
    }
}

#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_replication_recovery_plan {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReplicationRecoveryPlanArgs {
        /// The name of the Replication Plan.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the vault that should be updated.
        #[builder(into)]
        pub recovery_vault_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetReplicationRecoveryPlanResult {
        pub azure_to_azure_settings: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::siterecovery::GetReplicationRecoveryPlanAzureToAzureSetting,
            >,
        >,
        pub failover_deployment_model: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Name of the Action.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// `recovery_group` block defined as below.
        pub recovery_groups: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::siterecovery::GetReplicationRecoveryPlanRecoveryGroup,
            >,
        >,
        pub recovery_vault_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of source fabric to be recovered from.
        pub source_recovery_fabric_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of target fabric to recover.
        pub target_recovery_fabric_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetReplicationRecoveryPlanArgs,
    ) -> GetReplicationRecoveryPlanResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let recovery_vault_id_binding_1 = args.recovery_vault_id.get_output(context);
        let recovery_vault_id_binding = recovery_vault_id_binding_1.get_inner();
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetReplicationRecoveryPlanResult {
            azure_to_azure_settings: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("azureToAzureSettings"),
            ),
            failover_deployment_model: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("failoverDeploymentModel"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            recovery_groups: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryGroups"),
            ),
            recovery_vault_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryVaultId"),
            ),
            source_recovery_fabric_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("sourceRecoveryFabricId"),
            ),
            target_recovery_fabric_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("targetRecoveryFabricId"),
            ),
        }
    }
}

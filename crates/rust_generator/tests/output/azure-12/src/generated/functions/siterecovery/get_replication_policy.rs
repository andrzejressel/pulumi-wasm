#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_replication_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetReplicationPolicyArgs {
        /// Specifies the name of the Azure Site Recovery replication policy.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Recovery Services Vault that the Azure Site Recovery replication policy is associated witth.
        #[builder(into)]
        pub recovery_vault_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which the associated Azure Site Recovery replication policy resides.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetReplicationPolicyResult {
        /// Specifies the frequency (in minutes) at which to create application consistent recovery.
        pub application_consistent_snapshot_frequency_in_minutes: pulumi_gestalt_rust::Output<
            i32,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The duration in minutes for which the recovery points need to be stored.
        pub recovery_point_retention_in_minutes: pulumi_gestalt_rust::Output<i32>,
        pub recovery_vault_name: pulumi_gestalt_rust::Output<String>,
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetReplicationPolicyArgs,
    ) -> GetReplicationPolicyResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let recovery_vault_name_binding = args
            .recovery_vault_name
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:siterecovery/getReplicationPolicy:getReplicationPolicy".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "recoveryVaultName".into(),
                    value: &recovery_vault_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetReplicationPolicyResult {
            application_consistent_snapshot_frequency_in_minutes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("applicationConsistentSnapshotFrequencyInMinutes"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            recovery_point_retention_in_minutes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryPointRetentionInMinutes"),
            ),
            recovery_vault_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("recoveryVaultName"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}

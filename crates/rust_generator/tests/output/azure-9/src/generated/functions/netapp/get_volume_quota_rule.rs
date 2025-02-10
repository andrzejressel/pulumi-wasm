#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_volume_quota_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetVolumeQuotaRuleArgs {
        /// The name of this Volume Quota Rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The NetApp volume ID where the Volume Quota Rule is assigned to.
        #[builder(into)]
        pub volume_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetVolumeQuotaRuleResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Volume Quota Rule exists.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The quota size in kibibytes.
        pub quota_size_in_kib: pulumi_gestalt_rust::Output<i32>,
        /// The quota Target.
        pub quota_target: pulumi_gestalt_rust::Output<String>,
        /// The quota type.
        pub quota_type: pulumi_gestalt_rust::Output<String>,
        pub volume_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetVolumeQuotaRuleArgs,
    ) -> GetVolumeQuotaRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let volume_id_binding = args.volume_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:netapp/getVolumeQuotaRule:getVolumeQuotaRule".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "volumeId".into(),
                    value: volume_id_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetVolumeQuotaRuleResult {
            id: o.get_field("id"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            quota_size_in_kib: o.get_field("quotaSizeInKib"),
            quota_target: o.get_field("quotaTarget"),
            quota_type: o.get_field("quotaType"),
            volume_id: o.get_field("volumeId"),
        }
    }
}

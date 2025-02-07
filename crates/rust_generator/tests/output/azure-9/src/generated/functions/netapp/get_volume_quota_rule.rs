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
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetVolumeQuotaRuleArgs,
    ) -> GetVolumeQuotaRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding = args.name.get_output(context).get_inner();
        let volume_id_binding = args.volume_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:netapp/getVolumeQuotaRule:getVolumeQuotaRule".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "volumeId".into(),
                    value: &volume_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetVolumeQuotaRuleResult {
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            quota_size_in_kib: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("quotaSizeInKib"),
            ),
            quota_target: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("quotaTarget"),
            ),
            quota_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("quotaType"),
            ),
            volume_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("volumeId"),
            ),
        }
    }
}

#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_keys {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetKeysArgs {
        /// The name or id of the Cloud DNS managed zone.
        #[builder(into)]
        pub managed_zone: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs. If `project` is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetKeysResult {
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// A list of Key-signing key (KSK) records. Structure is documented below. Additionally, the DS record is provided:
        pub key_signing_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dns::GetKeysKeySigningKey>,
        >,
        pub managed_zone: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A list of Zone-signing key (ZSK) records. Structure is documented below.
        pub zone_signing_keys: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::dns::GetKeysZoneSigningKey>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetKeysArgs,
    ) -> GetKeysResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let managed_zone_binding = args.managed_zone.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "gcp:dns/getKeys:getKeys".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedZone".into(),
                    value: managed_zone_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: project_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetKeysResult {
            id: o.get_field("id"),
            key_signing_keys: o.get_field("keySigningKeys"),
            managed_zone: o.get_field("managedZone"),
            project: o.get_field("project"),
            zone_signing_keys: o.get_field("zoneSigningKeys"),
        }
    }
}

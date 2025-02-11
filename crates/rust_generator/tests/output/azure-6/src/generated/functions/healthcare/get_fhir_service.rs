#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_fhir_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetFhirServiceArgs {
        /// The name of the Healthcare FHIR Service.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The map of tags assigned to the Healthcare FHIR Service.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The id of the Healthcare Workspace in which the Healthcare FHIR Service exists.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetFhirServiceResult {
        /// The list of the access policies of the service instance.
        pub access_policy_object_ids: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The `authentication` block as defined below.
        pub authentications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::healthcare::GetFhirServiceAuthentication>,
        >,
        /// The name of the storage account which the operation configuration information is exported to.
        pub configuration_export_storage_account_name: pulumi_gestalt_rust::Output<
            String,
        >,
        /// The list of azure container registry settings used for convert data operation of the service instance.
        pub container_registry_login_server_urls: pulumi_gestalt_rust::Output<
            Vec<String>,
        >,
        /// The `cors` block as defined below.
        pub cors: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::healthcare::GetFhirServiceCor>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The `identity` block as defined below.
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::healthcare::GetFhirServiceIdentity>,
        >,
        /// The kind of the Healthcare FHIR Service.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The Azure Region where the Healthcare FHIR Service is located.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The map of tags assigned to the Healthcare FHIR Service.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetFhirServiceArgs,
    ) -> GetFhirServiceResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let workspace_id_binding = args.workspace_id.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:healthcare/getFhirService:getFhirService".into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding.drop_type(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetFhirServiceResult {
            access_policy_object_ids: o.get_field("accessPolicyObjectIds"),
            authentications: o.get_field("authentications"),
            configuration_export_storage_account_name: o
                .get_field("configurationExportStorageAccountName"),
            container_registry_login_server_urls: o
                .get_field("containerRegistryLoginServerUrls"),
            cors: o.get_field("cors"),
            id: o.get_field("id"),
            identities: o.get_field("identities"),
            kind: o.get_field("kind"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            tags: o.get_field("tags"),
            workspace_id: o.get_field("workspaceId"),
        }
    }
}

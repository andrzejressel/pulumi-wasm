#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_dicom_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetDicomServiceArgs {
        /// The name of the Healthcare DICOM Service
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The id of the Healthcare Workspace in which the Healthcare DICOM Service exists.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetDicomServiceResult {
        /// The `authentication` block as defined below.
        pub authentications: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::healthcare::GetDicomServiceAuthentication>,
        >,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub identities: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::healthcare::GetDicomServiceIdentity>,
        >,
        /// The Azure Region where the Healthcare DICOM Service is located.
        pub location: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        pub private_endpoints: pulumi_gestalt_rust::Output<
            Vec<super::super::super::types::healthcare::GetDicomServicePrivateEndpoint>,
        >,
        /// The url of the Healthcare DICOM Services.
        pub service_url: pulumi_gestalt_rust::Output<String>,
        /// A map of tags assigned to the Healthcare DICOM Service.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetDicomServiceArgs,
    ) -> GetDicomServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let workspace_id_binding_1 = args.workspace_id.get_output(context);
        let workspace_id_binding = workspace_id_binding_1.get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "azure:healthcare/getDicomService:getDicomService".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetDicomServiceResult {
            authentications: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authentications"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            identities: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identities"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            private_endpoints: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("privateEndpoints"),
            ),
            service_url: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("serviceUrl"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}

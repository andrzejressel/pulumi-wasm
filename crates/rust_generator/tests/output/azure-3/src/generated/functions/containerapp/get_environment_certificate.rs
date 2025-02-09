#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_environment_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetEnvironmentCertificateArgs {
        /// The ID of the Container App Environment to configure this Certificate on. Changing this forces a new resource to be created.
        #[builder(into)]
        pub container_app_environment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Container Apps Certificate. Changing this forces a new resource to be created.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetEnvironmentCertificateResult {
        pub container_app_environment_id: pulumi_gestalt_rust::Output<String>,
        /// The expiration date for the Certificate.
        pub expiration_date: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The date of issue for the Certificate.
        pub issue_date: pulumi_gestalt_rust::Output<String>,
        /// The Certificate Issuer.
        pub issuer: pulumi_gestalt_rust::Output<String>,
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Subject Name for the Certificate.
        pub subject_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags assigned to the resource.
        pub tags: pulumi_gestalt_rust::Output<std::collections::HashMap<String, String>>,
        /// The Thumbprint of the Certificate.
        pub thumbprint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
        args: GetEnvironmentCertificateArgs,
    ) -> GetEnvironmentCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let container_app_environment_id_binding = args
            .container_app_environment_id
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "azure:containerapp/getEnvironmentCertificate:getEnvironmentCertificate"
                .into(),
            version: super::super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "containerAppEnvironmentId".into(),
                    value: container_app_environment_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
            ],
        };
        let o = context.invoke_resource(request);
        GetEnvironmentCertificateResult {
            container_app_environment_id: o.get_field("containerAppEnvironmentId"),
            expiration_date: o.get_field("expirationDate"),
            id: o.get_field("id"),
            issue_date: o.get_field("issueDate"),
            issuer: o.get_field("issuer"),
            name: o.get_field("name"),
            subject_name: o.get_field("subjectName"),
            tags: o.get_field("tags"),
            thumbprint: o.get_field("thumbprint"),
        }
    }
}

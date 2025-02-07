pub mod get_provisioning_artifacts {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetProvisioningArtifactsArgs {
        /// Language code. Valid values: `en` (English), `jp` (Japanese), `zh` (Chinese). Default value is `en`.
        #[builder(into, default)]
        pub accept_language: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Product identifier.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub product_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GetProvisioningArtifactsResult {
        pub accept_language: pulumi_gestalt_rust::Output<Option<String>>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub product_id: pulumi_gestalt_rust::Output<String>,
        /// List with information about the provisioning artifacts. See details below.
        pub provisioning_artifact_details: pulumi_gestalt_rust::Output<
            Vec<
                super::super::super::types::servicecatalog::GetProvisioningArtifactsProvisioningArtifactDetail,
            >,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetProvisioningArtifactsArgs,
    ) -> GetProvisioningArtifactsResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let accept_language_binding = args
            .accept_language
            .get_output(context)
            .get_inner();
        let product_id_binding = args.product_id.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:servicecatalog/getProvisioningArtifacts:getProvisioningArtifacts"
                .into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "acceptLanguage".into(),
                    value: &accept_language_binding,
                },
                register_interface::ObjectField {
                    name: "productId".into(),
                    value: &product_id_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetProvisioningArtifactsResult {
            accept_language: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("acceptLanguage"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            product_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("productId"),
            ),
            provisioning_artifact_details: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("provisioningArtifactDetails"),
            ),
        }
    }
}

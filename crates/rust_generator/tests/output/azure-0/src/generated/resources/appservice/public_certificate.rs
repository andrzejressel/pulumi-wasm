/// Manages an App Service Public Certificate.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   examplePlan:
///     type: azure:appservice:Plan
///     name: example
///     properties:
///       name: example-app-service-plan
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         tier: Standard
///         size: S1
///   exampleAppService:
///     type: azure:appservice:AppService
///     name: example
///     properties:
///       name: example-app-service
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       appServicePlanId: ${examplePlan.id}
///   examplePublicCertificate:
///     type: azure:appservice:PublicCertificate
///     name: example
///     properties:
///       resourceGroupName: ${example.name}
///       appServiceName: ${exampleAppService.name}
///       certificateName: example-public-certificate
///       certificateLocation: Unknown
///       blob:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: app_service_public_certificate.cer
///           return: result
/// ```
///
/// ## Import
///
/// App Service Public Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:appservice/publicCertificate:PublicCertificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Web/sites/site1/publicCertificates/publicCertificate1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod public_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PublicCertificateArgs {
        /// The name of the App Service. Changing this forces a new App Service Public Certificate to be created.
        #[builder(into)]
        pub app_service_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The base64-encoded contents of the certificate. Changing this forces a new App Service Public Certificate to be created.
        #[builder(into)]
        pub blob: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location of the certificate. Possible values are `CurrentUserMy`, `LocalMachineMy` and `Unknown`. Changing this forces a new App Service Public Certificate to be created.
        #[builder(into)]
        pub certificate_location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the public certificate. Changing this forces a new App Service Public Certificate to be created.
        #[builder(into)]
        pub certificate_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Resource Group where the App Service Public Certificate should exist. Changing this forces a new App Service Public Certificate to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PublicCertificateResult {
        /// The name of the App Service. Changing this forces a new App Service Public Certificate to be created.
        pub app_service_name: pulumi_gestalt_rust::Output<String>,
        /// The base64-encoded contents of the certificate. Changing this forces a new App Service Public Certificate to be created.
        pub blob: pulumi_gestalt_rust::Output<String>,
        /// The location of the certificate. Possible values are `CurrentUserMy`, `LocalMachineMy` and `Unknown`. Changing this forces a new App Service Public Certificate to be created.
        pub certificate_location: pulumi_gestalt_rust::Output<String>,
        /// The name of the public certificate. Changing this forces a new App Service Public Certificate to be created.
        pub certificate_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the App Service Public Certificate should exist. Changing this forces a new App Service Public Certificate to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The thumbprint of the public certificate.
        pub thumbprint: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PublicCertificateArgs,
    ) -> PublicCertificateResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_service_name_binding = args.app_service_name.get_output(context);
        let blob_binding = args.blob.get_output(context);
        let certificate_location_binding = args.certificate_location.get_output(context);
        let certificate_name_binding = args.certificate_name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:appservice/publicCertificate:PublicCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appServiceName".into(),
                    value: &app_service_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "blob".into(),
                    value: &blob_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateLocation".into(),
                    value: &certificate_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificateName".into(),
                    value: &certificate_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PublicCertificateResult {
            app_service_name: o.get_field("appServiceName"),
            blob: o.get_field("blob"),
            certificate_location: o.get_field("certificateLocation"),
            certificate_name: o.get_field("certificateName"),
            resource_group_name: o.get_field("resourceGroupName"),
            thumbprint: o.get_field("thumbprint"),
        }
    }
}

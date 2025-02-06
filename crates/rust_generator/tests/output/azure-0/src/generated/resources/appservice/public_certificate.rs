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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PublicCertificateArgs,
    ) -> PublicCertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let app_service_name_binding = args
            .app_service_name
            .get_output(context)
            .get_inner();
        let blob_binding = args.blob.get_output(context).get_inner();
        let certificate_location_binding = args
            .certificate_location
            .get_output(context)
            .get_inner();
        let certificate_name_binding = args
            .certificate_name
            .get_output(context)
            .get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:appservice/publicCertificate:PublicCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "appServiceName".into(),
                    value: &app_service_name_binding,
                },
                register_interface::ObjectField {
                    name: "blob".into(),
                    value: &blob_binding,
                },
                register_interface::ObjectField {
                    name: "certificateLocation".into(),
                    value: &certificate_location_binding,
                },
                register_interface::ObjectField {
                    name: "certificateName".into(),
                    value: &certificate_name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PublicCertificateResult {
            app_service_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("appServiceName"),
            ),
            blob: pulumi_gestalt_rust::__private::into_domain(o.extract_field("blob")),
            certificate_location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateLocation"),
            ),
            certificate_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateName"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            thumbprint: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("thumbprint"),
            ),
        }
    }
}

/// Manages an IotHub Certificate.
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
///   exampleIoTHub:
///     type: azure:iot:IoTHub
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: B1
///         capacity: '1'
///   exampleCertificate:
///     type: azure:iot:Certificate
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       iothubName: ${exampleIoTHub.name}
///       isVerified: true
///       certificateContent:
///         fn::invoke:
///           function: std:filebase64
///           arguments:
///             input: example.cer
///           return: result
/// ```
///
/// ## Import
///
/// IoTHub Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/certificate:Certificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/iotHubs/example/certificates/example
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct CertificateArgs {
        /// The Base-64 representation of the X509 leaf certificate .cer file or just a .pem file content.
        #[builder(into)]
        pub certificate_content: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the IoTHub that this certificate will be attached to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iothub_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Is the certificate verified? Defaults to `false`.
        #[builder(into, default)]
        pub is_verified: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the IotHub Certificate resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group under which the IotHub Certificate resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct CertificateResult {
        /// The Base-64 representation of the X509 leaf certificate .cer file or just a .pem file content.
        pub certificate_content: pulumi_gestalt_rust::Output<String>,
        /// The name of the IoTHub that this certificate will be attached to. Changing this forces a new resource to be created.
        pub iothub_name: pulumi_gestalt_rust::Output<String>,
        /// Is the certificate verified? Defaults to `false`.
        pub is_verified: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the name of the IotHub Certificate resource. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group under which the IotHub Certificate resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: CertificateArgs,
    ) -> CertificateResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let certificate_content_binding_1 = args.certificate_content.get_output(context);
        let certificate_content_binding = certificate_content_binding_1.get_inner();
        let iothub_name_binding_1 = args.iothub_name.get_output(context);
        let iothub_name_binding = iothub_name_binding_1.get_inner();
        let is_verified_binding_1 = args.is_verified.get_output(context);
        let is_verified_binding = is_verified_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/certificate:Certificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateContent".into(),
                    value: &certificate_content_binding,
                },
                register_interface::ObjectField {
                    name: "iothubName".into(),
                    value: &iothub_name_binding,
                },
                register_interface::ObjectField {
                    name: "isVerified".into(),
                    value: &is_verified_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        CertificateResult {
            certificate_content: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("certificateContent"),
            ),
            iothub_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("iothubName"),
            ),
            is_verified: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("isVerified"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}

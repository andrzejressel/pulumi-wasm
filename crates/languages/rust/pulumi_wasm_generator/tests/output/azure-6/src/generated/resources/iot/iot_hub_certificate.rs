/// Manages an IotHub Device Provisioning Service Certificate.
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
///   exampleIotHubDps:
///     type: azure:iot:IotHubDps
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       sku:
///         name: S1
///         capacity: '1'
///   exampleIotHubCertificate:
///     type: azure:iot:IotHubCertificate
///     name: example
///     properties:
///       name: example
///       resourceGroupName: ${example.name}
///       iotDpsName: ${exampleIotHubDps.name}
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
/// IoTHub Device Provisioning Service Certificates can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:iot/iotHubCertificate:IotHubCertificate example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Devices/provisioningServices/example/certificates/example
/// ```
///
pub mod iot_hub_certificate {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IotHubCertificateArgs {
        /// The Base-64 representation of the X509 leaf certificate .cer file or just a .pem file content.
        #[builder(into)]
        pub certificate_content: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the IoT Device Provisioning Service that this certificate will be attached to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub iot_dps_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies if the certificate is created in verified state. Defaults to `false`. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub is_verified: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the name of the Iot Device Provisioning Service Certificate resource. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group under which the Iot Device Provisioning Service Certificate resource has to be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IotHubCertificateResult {
        /// The Base-64 representation of the X509 leaf certificate .cer file or just a .pem file content.
        pub certificate_content: pulumi_wasm_rust::Output<String>,
        /// The name of the IoT Device Provisioning Service that this certificate will be attached to. Changing this forces a new resource to be created.
        pub iot_dps_name: pulumi_wasm_rust::Output<String>,
        /// Specifies if the certificate is created in verified state. Defaults to `false`. Changing this forces a new resource to be created.
        pub is_verified: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the name of the Iot Device Provisioning Service Certificate resource. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group under which the Iot Device Provisioning Service Certificate resource has to be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: IotHubCertificateArgs,
    ) -> IotHubCertificateResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let certificate_content_binding = args
            .certificate_content
            .get_output(context)
            .get_inner();
        let iot_dps_name_binding = args.iot_dps_name.get_output(context).get_inner();
        let is_verified_binding = args.is_verified.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:iot/iotHubCertificate:IotHubCertificate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "certificateContent".into(),
                    value: &certificate_content_binding,
                },
                register_interface::ObjectField {
                    name: "iotDpsName".into(),
                    value: &iot_dps_name_binding,
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
        IotHubCertificateResult {
            certificate_content: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("certificateContent"),
            ),
            iot_dps_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("iotDpsName"),
            ),
            is_verified: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("isVerified"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
        }
    }
}

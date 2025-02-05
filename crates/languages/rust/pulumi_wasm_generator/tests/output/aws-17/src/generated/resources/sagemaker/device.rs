/// Provides a SageMaker Device resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = device::create(
///         "example",
///         DeviceArgs::builder()
///             .device(DeviceDevice::builder().deviceName("example").build_struct())
///             .device_fleet_name("${exampleAwsSagemakerDeviceFleet.deviceFleetName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Devices using the `device-fleet-name/device-name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/device:Device example my-fleet/my-device
/// ```
pub mod device {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceArgs {
        /// The device to register with SageMaker Edge Manager. See Device details below.
        #[builder(into)]
        pub device: pulumi_wasm_rust::InputOrOutput<
            super::super::types::sagemaker::DeviceDevice,
        >,
        /// The name of the Device Fleet.
        #[builder(into)]
        pub device_fleet_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DeviceResult {
        pub agent_version: pulumi_wasm_rust::Output<String>,
        /// The Amazon Resource Name (ARN) assigned by AWS to this Device.
        pub arn: pulumi_wasm_rust::Output<String>,
        /// The device to register with SageMaker Edge Manager. See Device details below.
        pub device: pulumi_wasm_rust::Output<
            super::super::types::sagemaker::DeviceDevice,
        >,
        /// The name of the Device Fleet.
        pub device_fleet_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DeviceArgs,
    ) -> DeviceResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let device_binding = args.device.get_output(context).get_inner();
        let device_fleet_name_binding = args
            .device_fleet_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:sagemaker/device:Device".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "device".into(),
                    value: &device_binding,
                },
                register_interface::ObjectField {
                    name: "deviceFleetName".into(),
                    value: &device_fleet_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DeviceResult {
            agent_version: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("agentVersion"),
            ),
            arn: pulumi_wasm_rust::__private::into_domain(o.extract_field("arn")),
            device: pulumi_wasm_rust::__private::into_domain(o.extract_field("device")),
            device_fleet_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("deviceFleetName"),
            ),
        }
    }
}

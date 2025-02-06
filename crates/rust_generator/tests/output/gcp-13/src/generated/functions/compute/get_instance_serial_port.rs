pub mod get_instance_serial_port {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetInstanceSerialPortArgs {
        /// The name of the Compute Instance to read output from.
        #[builder(into)]
        pub instance: pulumi_wasm_rust::InputOrOutput<String>,
        /// The number of the serial port to read output from. Possible values are 1-4.
        ///
        /// - - -
        #[builder(into)]
        pub port: pulumi_wasm_rust::InputOrOutput<i32>,
        /// The project in which the Compute Instance exists. If it
        /// is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The zone in which the Compute Instance exists.
        /// If it is not provided, the provider zone is used.
        #[builder(into, default)]
        pub zone: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetInstanceSerialPortResult {
        /// The output of the serial port. Serial port output is available only when the VM instance is running, and logs are limited to the most recent 1 MB of output per port.
        pub contents: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance: pulumi_wasm_rust::Output<String>,
        pub port: pulumi_wasm_rust::Output<i32>,
        pub project: pulumi_wasm_rust::Output<String>,
        pub zone: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_wasm_rust::PulumiContext,
        args: GetInstanceSerialPortArgs,
    ) -> GetInstanceSerialPortResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let instance_binding = args.instance.get_output(context).get_inner();
        let port_binding = args.port.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let zone_binding = args.zone.get_output(context).get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "gcp:compute/getInstanceSerialPort:getInstanceSerialPort".into(),
            version: super::super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "instance".into(),
                    value: &instance_binding,
                },
                register_interface::ObjectField {
                    name: "port".into(),
                    value: &port_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "zone".into(),
                    value: &zone_binding,
                },
            ]),
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetInstanceSerialPortResult {
            contents: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("contents"),
            ),
            id: pulumi_wasm_rust::__private::into_domain(o.extract_field("id")),
            instance: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("instance"),
            ),
            port: pulumi_wasm_rust::__private::into_domain(o.extract_field("port")),
            project: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            zone: pulumi_wasm_rust::__private::into_domain(o.extract_field("zone")),
        }
    }
}

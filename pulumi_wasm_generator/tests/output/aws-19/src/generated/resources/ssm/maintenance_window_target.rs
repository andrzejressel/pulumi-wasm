/// Provides an SSM Maintenance Window Target resource
///
/// ## Example Usage
///
/// ### Instance Target
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let target1 = maintenance_window_target::create(
///         "target1",
///         MaintenanceWindowTargetArgs::builder()
///             .description("This is a maintenance window target")
///             .name("maintenance-window-target")
///             .resource_type("INSTANCE")
///             .targets(
///                 vec![
///                     MaintenanceWindowTargetTarget::builder().key("tag:Name")
///                     .values(vec!["acceptance_test",]).build_struct(),
///                 ],
///             )
///             .window_id("${window.id}")
///             .build_struct(),
///     );
///     let window = maintenance_window::create(
///         "window",
///         MaintenanceWindowArgs::builder()
///             .cutoff(1)
///             .duration(3)
///             .name("maintenance-window-webapp")
///             .schedule("cron(0 16 ? * TUE *)")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Resource Group Target
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let target1 = maintenance_window_target::create(
///         "target1",
///         MaintenanceWindowTargetArgs::builder()
///             .description("This is a maintenance window target")
///             .name("maintenance-window-target")
///             .resource_type("RESOURCE_GROUP")
///             .targets(
///                 vec![
///                     MaintenanceWindowTargetTarget::builder()
///                     .key("resource-groups:ResourceTypeFilters")
///                     .values(vec!["AWS::EC2::Instance",]).build_struct(),
///                 ],
///             )
///             .window_id("${window.id}")
///             .build_struct(),
///     );
///     let window = maintenance_window::create(
///         "window",
///         MaintenanceWindowArgs::builder()
///             .cutoff(1)
///             .duration(3)
///             .name("maintenance-window-webapp")
///             .schedule("cron(0 16 ? * TUE *)")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SSM Maintenance Window targets using `WINDOW_ID/WINDOW_TARGET_ID`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/maintenanceWindowTarget:MaintenanceWindowTarget example mw-0c50858d01EXAMPLE/23639a0b-ddbc-4bca-9e72-78d96EXAMPLE
/// ```
pub mod maintenance_window_target {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MaintenanceWindowTargetArgs {
        /// The description of the maintenance window target.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the maintenance window target.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        /// User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window.
        #[builder(into, default)]
        pub owner_information: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of target being registered with the Maintenance Window. Possible values are `INSTANCE` and `RESOURCE_GROUP`.
        #[builder(into)]
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// The targets to register with the maintenance window. In other words, the instances to run commands on when the maintenance window runs. You can specify targets using instance IDs, resource group names, or tags that have been applied to instances. For more information about these examples formats see
        /// (https://docs.aws.amazon.com/systems-manager/latest/userguide/mw-cli-tutorial-targets-examples.html)
        #[builder(into)]
        pub targets: pulumi_wasm_rust::Output<
            Vec<super::super::types::ssm::MaintenanceWindowTargetTarget>,
        >,
        /// The Id of the maintenance window to register the target with.
        #[builder(into)]
        pub window_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct MaintenanceWindowTargetResult {
        /// The description of the maintenance window target.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the maintenance window target.
        pub name: pulumi_wasm_rust::Output<String>,
        /// User-provided value that will be included in any CloudWatch events raised while running tasks for these targets in this Maintenance Window.
        pub owner_information: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of target being registered with the Maintenance Window. Possible values are `INSTANCE` and `RESOURCE_GROUP`.
        pub resource_type: pulumi_wasm_rust::Output<String>,
        /// The targets to register with the maintenance window. In other words, the instances to run commands on when the maintenance window runs. You can specify targets using instance IDs, resource group names, or tags that have been applied to instances. For more information about these examples formats see
        /// (https://docs.aws.amazon.com/systems-manager/latest/userguide/mw-cli-tutorial-targets-examples.html)
        pub targets: pulumi_wasm_rust::Output<
            Vec<super::super::types::ssm::MaintenanceWindowTargetTarget>,
        >,
        /// The Id of the maintenance window to register the target with.
        pub window_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: MaintenanceWindowTargetArgs,
    ) -> MaintenanceWindowTargetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let description_binding = args.description.get_inner();
        let name_binding = args.name.get_inner();
        let owner_information_binding = args.owner_information.get_inner();
        let resource_type_binding = args.resource_type.get_inner();
        let targets_binding = args.targets.get_inner();
        let window_id_binding = args.window_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ssm/maintenanceWindowTarget:MaintenanceWindowTarget".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "ownerInformation".into(),
                    value: &owner_information_binding,
                },
                register_interface::ObjectField {
                    name: "resourceType".into(),
                    value: &resource_type_binding,
                },
                register_interface::ObjectField {
                    name: "targets".into(),
                    value: &targets_binding,
                },
                register_interface::ObjectField {
                    name: "windowId".into(),
                    value: &window_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "ownerInformation".into(),
                },
                register_interface::ResultField {
                    name: "resourceType".into(),
                },
                register_interface::ResultField {
                    name: "targets".into(),
                },
                register_interface::ResultField {
                    name: "windowId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MaintenanceWindowTargetResult {
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            owner_information: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("ownerInformation").unwrap(),
            ),
            resource_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("resourceType").unwrap(),
            ),
            targets: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("targets").unwrap(),
            ),
            window_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("windowId").unwrap(),
            ),
        }
    }
}

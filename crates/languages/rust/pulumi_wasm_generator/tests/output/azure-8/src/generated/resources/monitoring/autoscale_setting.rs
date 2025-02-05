/// Manages a AutoScale Setting which can be applied to Virtual Machine Scale Sets, App Services and other scalable resources.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: autoscalingTest
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: acctvn
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: acctsub
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleLinuxVirtualMachineScaleSet:
///     type: azure:compute:LinuxVirtualMachineScaleSet
///     name: example
///     properties:
///       name: exampleset
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       upgradeMode: Manual
///       sku: Standard_F2
///       instances: 2
///       adminUsername: myadmin
///       adminSshKeys:
///         - username: myadmin
///           publicKey: ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQDCsTcryUl51Q2VSEHqDRNmceUFo55ZtcIwxl2QITbN1RREti5ml/VTytC0yeBOvnZA4x4CFpdw/lCDPk0yrH9Ei5vVkXmOrExdTlT3qI7YaAzj1tUVlBd4S6LX1F7y6VLActvdHuDDuXZXzCDd/97420jrDfWZqJMlUK/EmCE5ParCeHIRIvmBxcEnGfFIsw8xQZl0HphxWOtJil8qsUWSdMyCiJYYQpMoMliO99X40AUc4/AlsyPyT5ddbKk08YrZ+rKDVHF7o29rh4vi5MmHkVgVQHKiKybWlHq+b71gIAUQk9wrJxD+dqt4igrmDSpIjfjwnd+l5UIn5fJSO5DYV4YT/4hwK7OKmuo7OFHD0WyY5YnkYEMtFgzemnRBdE8ulcT60DQpVgRMXFWHvhyCWy0L6sgj1QWDZlLpvsIvNfHsyhKFMG1frLnMt/nP0+YCcfg+v1JYeCKjeoJxB8DWcRBsjzItY0CGmzP8UYZiYKl/2u+2TgFS5r7NWH11bxoUzjKdaa1NLw+ieA8GlBFfCbfWe6YVB9ggUte4VtYFMZGxOjS2bAiYtfgTKFJv+XqORAwExG6+G2eDxIDyo80/OA9IG7Xv/jwQr7D6KDjDuULFcN/iTxuttoKrHeYz1hf5ZQlBdllwJHYx6fK2g8kha6r2JIQKocvsAXiiONqSfw== hello@world.com
///       networkInterfaces:
///         - name: TestNetworkProfile
///           primary: true
///           ipConfigurations:
///             - name: TestIPConfiguration
///               primary: true
///               subnetId: ${exampleSubnet.id}
///       osDisk:
///         caching: ReadWrite
///         storageAccountType: StandardSSD_LRS
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///   exampleAutoscaleSetting:
///     type: azure:monitoring:AutoscaleSetting
///     name: example
///     properties:
///       name: myAutoscaleSetting
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       targetResourceId: ${exampleLinuxVirtualMachineScaleSet.id}
///       profiles:
///         - name: defaultProfile
///           capacity:
///             default: 1
///             minimum: 1
///             maximum: 10
///           rules:
///             - metricTrigger:
///                 metricName: Percentage CPU
///                 metricResourceId: ${exampleLinuxVirtualMachineScaleSet.id}
///                 timeGrain: PT1M
///                 statistic: Average
///                 timeWindow: PT5M
///                 timeAggregation: Average
///                 operator: GreaterThan
///                 threshold: 75
///                 metricNamespace: microsoft.compute/virtualmachinescalesets
///                 dimensions:
///                   - name: AppName
///                     operator: Equals
///                     values:
///                       - App1
///               scaleAction:
///                 direction: Increase
///                 type: ChangeCount
///                 value: '1'
///                 cooldown: PT1M
///             - metricTrigger:
///                 metricName: Percentage CPU
///                 metricResourceId: ${exampleLinuxVirtualMachineScaleSet.id}
///                 timeGrain: PT1M
///                 statistic: Average
///                 timeWindow: PT5M
///                 timeAggregation: Average
///                 operator: LessThan
///                 threshold: 25
///               scaleAction:
///                 direction: Decrease
///                 type: ChangeCount
///                 value: '1'
///                 cooldown: PT1M
///       predictive:
///         scaleMode: Enabled
///         lookAheadTime: PT5M
///       notification:
///         email:
///           sendToSubscriptionAdministrator: true
///           sendToSubscriptionCoAdministrator: true
///           customEmails:
///             - admin@contoso.com
/// ```
///
///
/// ### Repeating On Weekends)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: autoscalingTest
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: acctvn
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: acctsub
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleLinuxVirtualMachineScaleSet:
///     type: azure:compute:LinuxVirtualMachineScaleSet
///     name: example
///     properties:
///       name: exampleset
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       upgradeMode: Manual
///       sku: Standard_F2
///       instances: 2
///       adminUsername: myadmin
///       adminSshKeys:
///         - username: myadmin
///           publicKey: ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQDCsTcryUl51Q2VSEHqDRNmceUFo55ZtcIwxl2QITbN1RREti5ml/VTytC0yeBOvnZA4x4CFpdw/lCDPk0yrH9Ei5vVkXmOrExdTlT3qI7YaAzj1tUVlBd4S6LX1F7y6VLActvdHuDDuXZXzCDd/97420jrDfWZqJMlUK/EmCE5ParCeHIRIvmBxcEnGfFIsw8xQZl0HphxWOtJil8qsUWSdMyCiJYYQpMoMliO99X40AUc4/AlsyPyT5ddbKk08YrZ+rKDVHF7o29rh4vi5MmHkVgVQHKiKybWlHq+b71gIAUQk9wrJxD+dqt4igrmDSpIjfjwnd+l5UIn5fJSO5DYV4YT/4hwK7OKmuo7OFHD0WyY5YnkYEMtFgzemnRBdE8ulcT60DQpVgRMXFWHvhyCWy0L6sgj1QWDZlLpvsIvNfHsyhKFMG1frLnMt/nP0+YCcfg+v1JYeCKjeoJxB8DWcRBsjzItY0CGmzP8UYZiYKl/2u+2TgFS5r7NWH11bxoUzjKdaa1NLw+ieA8GlBFfCbfWe6YVB9ggUte4VtYFMZGxOjS2bAiYtfgTKFJv+XqORAwExG6+G2eDxIDyo80/OA9IG7Xv/jwQr7D6KDjDuULFcN/iTxuttoKrHeYz1hf5ZQlBdllwJHYx6fK2g8kha6r2JIQKocvsAXiiONqSfw== hello@world.com
///       networkInterfaces:
///         - name: TestNetworkProfile
///           primary: true
///           ipConfigurations:
///             - name: TestIPConfiguration
///               primary: true
///               subnetId: ${exampleSubnet.id}
///       osDisk:
///         caching: ReadWrite
///         storageAccountType: StandardSSD_LRS
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///   exampleAutoscaleSetting:
///     type: azure:monitoring:AutoscaleSetting
///     name: example
///     properties:
///       name: myAutoscaleSetting
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       targetResourceId: ${exampleLinuxVirtualMachineScaleSet.id}
///       profiles:
///         - name: Weekends
///           capacity:
///             default: 1
///             minimum: 1
///             maximum: 10
///           rules:
///             - metricTrigger:
///                 metricName: Percentage CPU
///                 metricResourceId: ${exampleLinuxVirtualMachineScaleSet.id}
///                 timeGrain: PT1M
///                 statistic: Average
///                 timeWindow: PT5M
///                 timeAggregation: Average
///                 operator: GreaterThan
///                 threshold: 90
///               scaleAction:
///                 direction: Increase
///                 type: ChangeCount
///                 value: '2'
///                 cooldown: PT1M
///             - metricTrigger:
///                 metricName: Percentage CPU
///                 metricResourceId: ${exampleLinuxVirtualMachineScaleSet.id}
///                 timeGrain: PT1M
///                 statistic: Average
///                 timeWindow: PT5M
///                 timeAggregation: Average
///                 operator: LessThan
///                 threshold: 10
///               scaleAction:
///                 direction: Decrease
///                 type: ChangeCount
///                 value: '2'
///                 cooldown: PT1M
///           recurrence:
///             timezone: Pacific Standard Time
///             days:
///               - Saturday
///               - Sunday
///             hours: 12
///             minutes: 0
///       notification:
///         email:
///           sendToSubscriptionAdministrator: true
///           sendToSubscriptionCoAdministrator: true
///           customEmails:
///             - admin@contoso.com
/// ```
///
///
/// ### For Fixed Dates)
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: autoscalingTest
///       location: West Europe
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: acctvn
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: acctsub
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleLinuxVirtualMachineScaleSet:
///     type: azure:compute:LinuxVirtualMachineScaleSet
///     name: example
///     properties:
///       name: exampleset
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       upgradeMode: Manual
///       sku: Standard_F2
///       instances: 2
///       adminUsername: myadmin
///       adminSshKeys:
///         - username: myadmin
///           publicKey: ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQDCsTcryUl51Q2VSEHqDRNmceUFo55ZtcIwxl2QITbN1RREti5ml/VTytC0yeBOvnZA4x4CFpdw/lCDPk0yrH9Ei5vVkXmOrExdTlT3qI7YaAzj1tUVlBd4S6LX1F7y6VLActvdHuDDuXZXzCDd/97420jrDfWZqJMlUK/EmCE5ParCeHIRIvmBxcEnGfFIsw8xQZl0HphxWOtJil8qsUWSdMyCiJYYQpMoMliO99X40AUc4/AlsyPyT5ddbKk08YrZ+rKDVHF7o29rh4vi5MmHkVgVQHKiKybWlHq+b71gIAUQk9wrJxD+dqt4igrmDSpIjfjwnd+l5UIn5fJSO5DYV4YT/4hwK7OKmuo7OFHD0WyY5YnkYEMtFgzemnRBdE8ulcT60DQpVgRMXFWHvhyCWy0L6sgj1QWDZlLpvsIvNfHsyhKFMG1frLnMt/nP0+YCcfg+v1JYeCKjeoJxB8DWcRBsjzItY0CGmzP8UYZiYKl/2u+2TgFS5r7NWH11bxoUzjKdaa1NLw+ieA8GlBFfCbfWe6YVB9ggUte4VtYFMZGxOjS2bAiYtfgTKFJv+XqORAwExG6+G2eDxIDyo80/OA9IG7Xv/jwQr7D6KDjDuULFcN/iTxuttoKrHeYz1hf5ZQlBdllwJHYx6fK2g8kha6r2JIQKocvsAXiiONqSfw== hello@world.com
///       networkInterfaces:
///         - name: TestNetworkProfile
///           primary: true
///           ipConfigurations:
///             - name: TestIPConfiguration
///               primary: true
///               subnetId: ${exampleSubnet.id}
///       osDisk:
///         caching: ReadWrite
///         storageAccountType: StandardSSD_LRS
///       sourceImageReference:
///         publisher: Canonical
///         offer: 0001-com-ubuntu-server-jammy
///         sku: 22_04-lts
///         version: latest
///   exampleAutoscaleSetting:
///     type: azure:monitoring:AutoscaleSetting
///     name: example
///     properties:
///       name: myAutoscaleSetting
///       enabled: true
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       targetResourceId: ${exampleLinuxVirtualMachineScaleSet.id}
///       profiles:
///         - name: forJuly
///           capacity:
///             default: 1
///             minimum: 1
///             maximum: 10
///           rules:
///             - metricTrigger:
///                 metricName: Percentage CPU
///                 metricResourceId: ${exampleLinuxVirtualMachineScaleSet.id}
///                 timeGrain: PT1M
///                 statistic: Average
///                 timeWindow: PT5M
///                 timeAggregation: Average
///                 operator: GreaterThan
///                 threshold: 90
///               scaleAction:
///                 direction: Increase
///                 type: ChangeCount
///                 value: '2'
///                 cooldown: PT1M
///             - metricTrigger:
///                 metricName: Percentage CPU
///                 metricResourceId: ${exampleLinuxVirtualMachineScaleSet.id}
///                 timeGrain: PT1M
///                 statistic: Average
///                 timeWindow: PT5M
///                 timeAggregation: Average
///                 operator: LessThan
///                 threshold: 10
///               scaleAction:
///                 direction: Decrease
///                 type: ChangeCount
///                 value: '2'
///                 cooldown: PT1M
///           fixedDate:
///             timezone: Pacific Standard Time
///             start: 2020-07-01T00:00:00Z
///             end: 2020-07-31T23:59:59Z
///       notification:
///         email:
///           sendToSubscriptionAdministrator: true
///           sendToSubscriptionCoAdministrator: true
///           customEmails:
///             - admin@contoso.com
/// ```
///
/// ## Import
///
/// AutoScale Setting can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:monitoring/autoscaleSetting:AutoscaleSetting example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Insights/autoScaleSettings/setting1
/// ```
///
pub mod autoscale_setting {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AutoscaleSettingArgs {
        /// Specifies whether automatic scaling is enabled for the target resource. Defaults to `true`.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the AutoScale Setting should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the AutoScale Setting. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies a `notification` block as defined below.
        #[builder(into, default)]
        pub notification: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::monitoring::AutoscaleSettingNotification>,
        >,
        /// A `predictive` block as defined below.
        #[builder(into, default)]
        pub predictive: pulumi_wasm_rust::InputOrOutput<
            Option<super::super::types::monitoring::AutoscaleSettingPredictive>,
        >,
        /// Specifies one or more (up to 20) `profile` blocks as defined below.
        #[builder(into)]
        pub profiles: pulumi_wasm_rust::InputOrOutput<
            Vec<super::super::types::monitoring::AutoscaleSettingProfile>,
        >,
        /// The name of the Resource Group in the AutoScale Setting should be created. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the resource ID of the resource that the autoscale setting should be added to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_resource_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AutoscaleSettingResult {
        /// Specifies whether automatic scaling is enabled for the target resource. Defaults to `true`.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the AutoScale Setting should exist. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the AutoScale Setting. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies a `notification` block as defined below.
        pub notification: pulumi_wasm_rust::Output<
            Option<super::super::types::monitoring::AutoscaleSettingNotification>,
        >,
        /// A `predictive` block as defined below.
        pub predictive: pulumi_wasm_rust::Output<
            Option<super::super::types::monitoring::AutoscaleSettingPredictive>,
        >,
        /// Specifies one or more (up to 20) `profile` blocks as defined below.
        pub profiles: pulumi_wasm_rust::Output<
            Vec<super::super::types::monitoring::AutoscaleSettingProfile>,
        >,
        /// The name of the Resource Group in the AutoScale Setting should be created. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the resource ID of the resource that the autoscale setting should be added to. Changing this forces a new resource to be created.
        pub target_resource_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AutoscaleSettingArgs,
    ) -> AutoscaleSettingResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let enabled_binding = args.enabled.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let notification_binding = args.notification.get_output(context).get_inner();
        let predictive_binding = args.predictive.get_output(context).get_inner();
        let profiles_binding = args.profiles.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let target_resource_id_binding = args
            .target_resource_id
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:monitoring/autoscaleSetting:AutoscaleSetting".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "notification".into(),
                    value: &notification_binding,
                },
                register_interface::ObjectField {
                    name: "predictive".into(),
                    value: &predictive_binding,
                },
                register_interface::ObjectField {
                    name: "profiles".into(),
                    value: &profiles_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
                register_interface::ObjectField {
                    name: "targetResourceId".into(),
                    value: &target_resource_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AutoscaleSettingResult {
            enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            notification: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("notification"),
            ),
            predictive: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("predictive"),
            ),
            profiles: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("profiles"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
            target_resource_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetResourceId"),
            ),
        }
    }
}

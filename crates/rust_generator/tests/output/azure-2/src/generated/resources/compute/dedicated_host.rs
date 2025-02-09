/// Manage a Dedicated Host within a Dedicated Host Group.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleDedicatedHost = dedicated_host::create(
///         "exampleDedicatedHost",
///         DedicatedHostArgs::builder()
///             .dedicated_host_group_id("${exampleDedicatedHostGroup.id}")
///             .location("${example.location}")
///             .name("example-host")
///             .platform_fault_domain(1)
///             .sku_name("DSv3-Type3")
///             .build_struct(),
///     );
///     let exampleDedicatedHostGroup = dedicated_host_group::create(
///         "exampleDedicatedHostGroup",
///         DedicatedHostGroupArgs::builder()
///             .location("${example.location}")
///             .name("example-host-group")
///             .platform_fault_domain_count(2)
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Dedicated Hosts can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:compute/dedicatedHost:DedicatedHost example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Compute/hostGroups/group1/hosts/host1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dedicated_host {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DedicatedHostArgs {
        /// Should the Dedicated Host automatically be replaced in case of a Hardware Failure? Defaults to `true`.
        #[builder(into, default)]
        pub auto_replace_on_failure: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Specifies the ID of the Dedicated Host Group where the Dedicated Host should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub dedicated_host_group_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the software license type that will be applied to the VMs deployed on the Dedicated Host. Possible values are `None`, `Windows_Server_Hybrid` and `Windows_Server_Perpetual`. Defaults to `None`.
        #[builder(into, default)]
        pub license_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Dedicated Host. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specify the fault domain of the Dedicated Host Group in which to create the Dedicated Host. Changing this forces a new resource to be created.
        #[builder(into)]
        pub platform_fault_domain: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Specify the SKU name of the Dedicated Host. Possible values are `DADSv5-Type1`, `DASv4-Type1`, `DASv4-Type2`, `DASv5-Type1`, `DCSv2-Type1`, `DDSv4-Type1`, `DDSv4-Type2`, `DDSv5-Type1`, `DSv3-Type1`, `DSv3-Type2`, `DSv3-Type3`, `DSv3-Type4`, `DSv4-Type1`, `DSv4-Type2`, `DSv5-Type1`, `EADSv5-Type1`, `EASv4-Type1`, `EASv4-Type2`, `EASv5-Type1`, `EDSv4-Type1`, `EDSv4-Type2`, `EDSv5-Type1`, `ESv3-Type1`, `ESv3-Type2`, `ESv3-Type3`, `ESv3-Type4`, `ESv4-Type1`, `ESv4-Type2`, `ESv5-Type1`, `FSv2-Type2`, `FSv2-Type3`, `FSv2-Type4`, `FXmds-Type1`, `LSv2-Type1`, `LSv3-Type1`, `MDMSv2MedMem-Type1`, `MDSv2MedMem-Type1`, `MMSv2MedMem-Type1`, `MS-Type1`, `MSm-Type1`, `MSmv2-Type1`, `MSv2-Type1`, `MSv2MedMem-Type1`, `NVASv4-Type1` and `NVSv3-Type1`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sku_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DedicatedHostResult {
        /// Should the Dedicated Host automatically be replaced in case of a Hardware Failure? Defaults to `true`.
        pub auto_replace_on_failure: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the ID of the Dedicated Host Group where the Dedicated Host should exist. Changing this forces a new resource to be created.
        pub dedicated_host_group_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the software license type that will be applied to the VMs deployed on the Dedicated Host. Possible values are `None`, `Windows_Server_Hybrid` and `Windows_Server_Perpetual`. Defaults to `None`.
        pub license_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specify the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Dedicated Host. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specify the fault domain of the Dedicated Host Group in which to create the Dedicated Host. Changing this forces a new resource to be created.
        pub platform_fault_domain: pulumi_gestalt_rust::Output<i32>,
        /// Specify the SKU name of the Dedicated Host. Possible values are `DADSv5-Type1`, `DASv4-Type1`, `DASv4-Type2`, `DASv5-Type1`, `DCSv2-Type1`, `DDSv4-Type1`, `DDSv4-Type2`, `DDSv5-Type1`, `DSv3-Type1`, `DSv3-Type2`, `DSv3-Type3`, `DSv3-Type4`, `DSv4-Type1`, `DSv4-Type2`, `DSv5-Type1`, `EADSv5-Type1`, `EASv4-Type1`, `EASv4-Type2`, `EASv5-Type1`, `EDSv4-Type1`, `EDSv4-Type2`, `EDSv5-Type1`, `ESv3-Type1`, `ESv3-Type2`, `ESv3-Type3`, `ESv3-Type4`, `ESv4-Type1`, `ESv4-Type2`, `ESv5-Type1`, `FSv2-Type2`, `FSv2-Type3`, `FSv2-Type4`, `FXmds-Type1`, `LSv2-Type1`, `LSv3-Type1`, `MDMSv2MedMem-Type1`, `MDSv2MedMem-Type1`, `MMSv2MedMem-Type1`, `MS-Type1`, `MSm-Type1`, `MSmv2-Type1`, `MSv2-Type1`, `MSv2MedMem-Type1`, `NVASv4-Type1` and `NVSv3-Type1`. Changing this forces a new resource to be created.
        pub sku_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: DedicatedHostArgs,
    ) -> DedicatedHostResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let auto_replace_on_failure_binding_1 = args
            .auto_replace_on_failure
            .get_output(context);
        let auto_replace_on_failure_binding = auto_replace_on_failure_binding_1
            .get_inner();
        let dedicated_host_group_id_binding_1 = args
            .dedicated_host_group_id
            .get_output(context);
        let dedicated_host_group_id_binding = dedicated_host_group_id_binding_1
            .get_inner();
        let license_type_binding_1 = args.license_type.get_output(context);
        let license_type_binding = license_type_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let platform_fault_domain_binding_1 = args
            .platform_fault_domain
            .get_output(context);
        let platform_fault_domain_binding = platform_fault_domain_binding_1.get_inner();
        let sku_name_binding_1 = args.sku_name.get_output(context);
        let sku_name_binding = sku_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:compute/dedicatedHost:DedicatedHost".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autoReplaceOnFailure".into(),
                    value: &auto_replace_on_failure_binding,
                },
                register_interface::ObjectField {
                    name: "dedicatedHostGroupId".into(),
                    value: &dedicated_host_group_id_binding,
                },
                register_interface::ObjectField {
                    name: "licenseType".into(),
                    value: &license_type_binding,
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
                    name: "platformFaultDomain".into(),
                    value: &platform_fault_domain_binding,
                },
                register_interface::ObjectField {
                    name: "skuName".into(),
                    value: &sku_name_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        DedicatedHostResult {
            auto_replace_on_failure: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autoReplaceOnFailure"),
            ),
            dedicated_host_group_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("dedicatedHostGroupId"),
            ),
            license_type: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("licenseType"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            platform_fault_domain: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("platformFaultDomain"),
            ),
            sku_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("skuName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}

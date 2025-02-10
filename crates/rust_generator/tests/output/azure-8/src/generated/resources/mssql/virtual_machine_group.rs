/// Manages a Microsoft SQL Virtual Machine Group.
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
///     let exampleVirtualMachineGroup = virtual_machine_group::create(
///         "exampleVirtualMachineGroup",
///         VirtualMachineGroupArgs::builder()
///             .location("${example.location}")
///             .name("examplegroup")
///             .resource_group_name("${example.name}")
///             .sql_image_offer("SQL2017-WS2016")
///             .sql_image_sku("Developer")
///             .wsfc_domain_profile(
///                 VirtualMachineGroupWsfcDomainProfile::builder()
///                     .clusterSubnetType("SingleSubnet")
///                     .fqdn("testdomain.com")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Microsoft SQL Virtual Machine Groups can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:mssql/virtualMachineGroup:VirtualMachineGroup example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachineGroups/vmgroup1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_machine_group {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualMachineGroupArgs {
        /// The Azure Region where the Microsoft SQL Virtual Machine Group should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name which should be used for the Microsoft SQL Virtual Machine Group. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Microsoft SQL Virtual Machine Group should exist. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The offer type of the marketplace image cluster to be used by the SQL Virtual Machine Group. Changing this forces a new resource to be created.
        #[builder(into)]
        pub sql_image_offer: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The sku type of the marketplace image cluster to be used by the SQL Virtual Machine Group. Possible values are `Developer` and `Enterprise`.
        #[builder(into)]
        pub sql_image_sku: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags which should be assigned to the Microsoft SQL Virtual Machine Group.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `wsfc_domain_profile` block as defined below.
        #[builder(into)]
        pub wsfc_domain_profile: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::mssql::VirtualMachineGroupWsfcDomainProfile,
        >,
    }
    #[allow(dead_code)]
    pub struct VirtualMachineGroupResult {
        /// The Azure Region where the Microsoft SQL Virtual Machine Group should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for the Microsoft SQL Virtual Machine Group. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Microsoft SQL Virtual Machine Group should exist. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The offer type of the marketplace image cluster to be used by the SQL Virtual Machine Group. Changing this forces a new resource to be created.
        pub sql_image_offer: pulumi_gestalt_rust::Output<String>,
        /// The sku type of the marketplace image cluster to be used by the SQL Virtual Machine Group. Possible values are `Developer` and `Enterprise`.
        pub sql_image_sku: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags which should be assigned to the Microsoft SQL Virtual Machine Group.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A `wsfc_domain_profile` block as defined below.
        pub wsfc_domain_profile: pulumi_gestalt_rust::Output<
            super::super::types::mssql::VirtualMachineGroupWsfcDomainProfile,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualMachineGroupArgs,
    ) -> VirtualMachineGroupResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let sql_image_offer_binding = args.sql_image_offer.get_output(context);
        let sql_image_sku_binding = args.sql_image_sku.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let wsfc_domain_profile_binding = args.wsfc_domain_profile.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:mssql/virtualMachineGroup:VirtualMachineGroup".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlImageOffer".into(),
                    value: sql_image_offer_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "sqlImageSku".into(),
                    value: sql_image_sku_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "wsfcDomainProfile".into(),
                    value: wsfc_domain_profile_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualMachineGroupResult {
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            sql_image_offer: o.get_field("sqlImageOffer"),
            sql_image_sku: o.get_field("sqlImageSku"),
            tags: o.get_field("tags"),
            wsfc_domain_profile: o.get_field("wsfcDomainProfile"),
        }
    }
}

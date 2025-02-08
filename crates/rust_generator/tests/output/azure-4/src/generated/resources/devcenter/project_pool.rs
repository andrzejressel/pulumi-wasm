/// Manages a Dev Center Project Pool.
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
///   exampleDevCenter:
///     type: azure:devcenter:DevCenter
///     name: example
///     properties:
///       name: example-dc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       identity:
///         type: SystemAssigned
///   exampleVirtualNetwork:
///     type: azure:network:VirtualNetwork
///     name: example
///     properties:
///       name: example-vnet
///       addressSpaces:
///         - 10.0.0.0/16
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleSubnet:
///     type: azure:network:Subnet
///     name: example
///     properties:
///       name: internal
///       resourceGroupName: ${example.name}
///       virtualNetworkName: ${exampleVirtualNetwork.name}
///       addressPrefixes:
///         - 10.0.2.0/24
///   exampleNetworkConnection:
///     type: azure:devcenter:NetworkConnection
///     name: example
///     properties:
///       name: example-dcnc
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       subnetId: ${exampleSubnet.id}
///       domainJoinType: AzureADJoin
///   exampleAttachedNetwork:
///     type: azure:devcenter:AttachedNetwork
///     name: example
///     properties:
///       name: example-dcet
///       devCenterId: ${exampleDevCenter.id}
///       networkConnectionId: ${exampleNetworkConnection.id}
///   exampleProject:
///     type: azure:devcenter:Project
///     name: example
///     properties:
///       name: example-dcp
///       resourceGroupName: ${example.name}
///       location: ${example.location}
///       devCenterId: ${exampleDevCenter.id}
///   exampleDevBoxDefinition:
///     type: azure:devcenter:DevBoxDefinition
///     name: example
///     properties:
///       name: example-dcet
///       location: ${example.location}
///       devCenterId: ${exampleDevCenter.id}
///       imageReferenceId: ${exampleDevCenter.id}/galleries/default/images/microsoftvisualstudio_visualstudioplustools_vs-2022-ent-general-win10-m365-gen2
///       skuName: general_i_8c32gb256ssd_v2
///   exampleProjectPool:
///     type: azure:devcenter:ProjectPool
///     name: example
///     properties:
///       name: example-dcpl
///       location: ${example.location}
///       devCenterProjectId: ${exampleProject.id}
///       devBoxDefinitionName: ${exampleDevBoxDefinition.name}
///       localAdministratorEnabled: true
///       devCenterAttachedNetworkName: ${exampleAttachedNetwork.name}
///       stopOnDisconnectGracePeriodMinutes: 60
/// ```
///
/// ## Import
///
/// An existing Dev Center Project Pool can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:devcenter/projectPool:ProjectPool example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.DevCenter/projects/project1/pools/pool1
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod project_pool {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ProjectPoolArgs {
        /// The name of the Dev Center Dev Box Definition.
        #[builder(into)]
        pub dev_box_definition_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the Dev Center Attached Network in parent Project of the Dev Center Project Pool.
        #[builder(into)]
        pub dev_center_attached_network_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the associated Dev Center Project. Changing this forces a new resource to be created.
        #[builder(into)]
        pub dev_center_project_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies whether owners of Dev Boxes in the Dev Center Project Pool are added as local administrators on the Dev Box.
        #[builder(into)]
        pub local_administrator_enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The Azure Region where the Dev Center Project Pool should exist. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of this Dev Center Project Pool. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The specified time in minutes to wait before stopping a Dev Center Dev Box once disconnect is detected. Possible values are between `60` and `480`.
        #[builder(into, default)]
        pub stop_on_disconnect_grace_period_minutes: pulumi_gestalt_rust::InputOrOutput<
            Option<i32>,
        >,
        /// A mapping of tags which should be assigned to the Dev Center Project Pool.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ProjectPoolResult {
        /// The name of the Dev Center Dev Box Definition.
        pub dev_box_definition_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Dev Center Attached Network in parent Project of the Dev Center Project Pool.
        pub dev_center_attached_network_name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the associated Dev Center Project. Changing this forces a new resource to be created.
        pub dev_center_project_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies whether owners of Dev Boxes in the Dev Center Project Pool are added as local administrators on the Dev Box.
        pub local_administrator_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The Azure Region where the Dev Center Project Pool should exist. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Dev Center Project Pool. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The specified time in minutes to wait before stopping a Dev Center Dev Box once disconnect is detected. Possible values are between `60` and `480`.
        pub stop_on_disconnect_grace_period_minutes: pulumi_gestalt_rust::Output<
            Option<i32>,
        >,
        /// A mapping of tags which should be assigned to the Dev Center Project Pool.
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
        args: ProjectPoolArgs,
    ) -> ProjectPoolResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let dev_box_definition_name_binding = args
            .dev_box_definition_name
            .get_output(context)
            .get_inner();
        let dev_center_attached_network_name_binding = args
            .dev_center_attached_network_name
            .get_output(context)
            .get_inner();
        let dev_center_project_id_binding = args
            .dev_center_project_id
            .get_output(context)
            .get_inner();
        let local_administrator_enabled_binding = args
            .local_administrator_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let stop_on_disconnect_grace_period_minutes_binding = args
            .stop_on_disconnect_grace_period_minutes
            .get_output(context)
            .get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:devcenter/projectPool:ProjectPool".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "devBoxDefinitionName".into(),
                    value: &dev_box_definition_name_binding,
                },
                register_interface::ObjectField {
                    name: "devCenterAttachedNetworkName".into(),
                    value: &dev_center_attached_network_name_binding,
                },
                register_interface::ObjectField {
                    name: "devCenterProjectId".into(),
                    value: &dev_center_project_id_binding,
                },
                register_interface::ObjectField {
                    name: "localAdministratorEnabled".into(),
                    value: &local_administrator_enabled_binding,
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
                    name: "stopOnDisconnectGracePeriodMinutes".into(),
                    value: &stop_on_disconnect_grace_period_minutes_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ProjectPoolResult {
            dev_box_definition_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("devBoxDefinitionName"),
            ),
            dev_center_attached_network_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("devCenterAttachedNetworkName"),
            ),
            dev_center_project_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("devCenterProjectId"),
            ),
            local_administrator_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localAdministratorEnabled"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            stop_on_disconnect_grace_period_minutes: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("stopOnDisconnectGracePeriodMinutes"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}

/// Manages a Kusto Cluster Principal Assignment.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: KustoRG
///       location: West Europe
///   exampleCluster:
///     type: azure:kusto:Cluster
///     name: example
///     properties:
///       name: kustocluster
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       sku:
///         name: Standard_D13_v2
///         capacity: 2
///   exampleClusterPrincipalAssignment:
///     type: azure:kusto:ClusterPrincipalAssignment
///     name: example
///     properties:
///       name: KustoPrincipalAssignment
///       resourceGroupName: ${example.name}
///       clusterName: ${exampleCluster.name}
///       tenantId: ${current.tenantId}
///       principalId: ${current.clientId}
///       principalType: App
///       role: AllDatabasesAdmin
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Data Explorer Cluster Principal Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:kusto/clusterPrincipalAssignment:ClusterPrincipalAssignment example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Kusto/clusters/cluster1/principalAssignments/assignment1
/// ```
///
pub mod cluster_principal_assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ClusterPrincipalAssignmentArgs {
        /// The name of the cluster in which to create the resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub cluster_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Kusto cluster principal assignment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The object id of the principal. Changing this forces a new resource to be created.
        #[builder(into)]
        pub principal_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of the principal. Valid values include `App`, `Group`, `User`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub principal_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The cluster role assigned to the principal. Valid values include `AllDatabasesAdmin` and `AllDatabasesViewer`. Changing this forces a new resource to be created.
        #[builder(into)]
        pub role: pulumi_wasm_rust::InputOrOutput<String>,
        /// The tenant id in which the principal resides. Changing this forces a new resource to be created.
        #[builder(into)]
        pub tenant_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ClusterPrincipalAssignmentResult {
        /// The name of the cluster in which to create the resource. Changing this forces a new resource to be created.
        pub cluster_name: pulumi_wasm_rust::Output<String>,
        /// The name of the Kusto cluster principal assignment. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The object id of the principal. Changing this forces a new resource to be created.
        pub principal_id: pulumi_wasm_rust::Output<String>,
        /// The name of the principal.
        pub principal_name: pulumi_wasm_rust::Output<String>,
        /// The type of the principal. Valid values include `App`, `Group`, `User`. Changing this forces a new resource to be created.
        pub principal_type: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the resource. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The cluster role assigned to the principal. Valid values include `AllDatabasesAdmin` and `AllDatabasesViewer`. Changing this forces a new resource to be created.
        pub role: pulumi_wasm_rust::Output<String>,
        /// The tenant id in which the principal resides. Changing this forces a new resource to be created.
        pub tenant_id: pulumi_wasm_rust::Output<String>,
        /// The name of the tenant.
        pub tenant_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: ClusterPrincipalAssignmentArgs,
    ) -> ClusterPrincipalAssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let cluster_name_binding = args.cluster_name.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let principal_id_binding = args.principal_id.get_output(context).get_inner();
        let principal_type_binding = args.principal_type.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let tenant_id_binding = args.tenant_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:kusto/clusterPrincipalAssignment:ClusterPrincipalAssignment"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "clusterName".into(),
                    value: &cluster_name_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "principalId".into(),
                    value: &principal_id_binding,
                },
                register_interface::ObjectField {
                    name: "principalType".into(),
                    value: &principal_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
                register_interface::ObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ClusterPrincipalAssignmentResult {
            cluster_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("clusterName"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            principal_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("principalId"),
            ),
            principal_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("principalName"),
            ),
            principal_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("principalType"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            role: pulumi_wasm_rust::__private::into_domain(o.extract_field("role")),
            tenant_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenantId"),
            ),
            tenant_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("tenantName"),
            ),
        }
    }
}

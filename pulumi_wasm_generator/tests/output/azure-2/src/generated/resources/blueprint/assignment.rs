/// Manages a Blueprint Assignment resource
///
/// > **NOTE:** Azure Blueprints are in Preview and potentially subject to breaking change without notice.
///
/// > **NOTE:** Azure Blueprint Assignments can only be applied to Subscriptions.  Assignments to Management Groups is not currently supported by the service or by this provider.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleResourceGroup:
///     type: azure:core:ResourceGroup
///     name: example
///     properties:
///       name: exampleRG-bp
///       location: West Europe
///       tags:
///         Environment: example
///   exampleUserAssignedIdentity:
///     type: azure:authorization:UserAssignedIdentity
///     name: example
///     properties:
///       resourceGroupName: ${exampleResourceGroup.name}
///       location: ${exampleResourceGroup.location}
///       name: bp-user-example
///   operator:
///     type: azure:authorization:Assignment
///     properties:
///       scope: ${example.id}
///       roleDefinitionName: Blueprint Operator
///       principalId: ${exampleUserAssignedIdentity.principalId}
///   owner:
///     type: azure:authorization:Assignment
///     properties:
///       scope: ${example.id}
///       roleDefinitionName: Owner
///       principalId: ${exampleUserAssignedIdentity.principalId}
///   exampleAssignment:
///     type: azure:blueprint:Assignment
///     name: example
///     properties:
///       name: testAccBPAssignment
///       targetSubscriptionId: ${example.id}
///       versionId: ${exampleGetPublishedVersion.id}
///       location: ${exampleResourceGroup.location}
///       lockMode: AllResourcesDoNotDelete
///       lockExcludePrincipals:
///         - ${current.objectId}
///       identity:
///         type: UserAssigned
///         identityIds:
///           - ${exampleUserAssignedIdentity.id}
///       resourceGroups: |2
///             {
///               "ResourceGroup": {
///                 "name": "exampleRG-bp"
///               }
///             }
///       parameterValues: |2
///             {
///               "allowedlocationsforresourcegroups_listOfAllowedLocations": {
///                 "value": ["westus", "westus2", "eastus", "centralus", "centraluseuap", "southcentralus", "northcentralus", "westcentralus", "eastus2", "eastus2euap", "brazilsouth", "brazilus", "northeurope", "westeurope", "eastasia", "southeastasia", "japanwest", "japaneast", "koreacentral", "koreasouth", "indiasouth", "indiawest", "indiacentral", "australiaeast", "australiasoutheast", "canadacentral", "canadaeast", "uknorth", "uksouth2", "uksouth", "ukwest", "francecentral", "francesouth", "australiacentral", "australiacentral2", "uaecentral", "uaenorth", "southafricanorth", "southafricawest", "switzerlandnorth", "switzerlandwest", "germanynorth", "germanywestcentral", "norwayeast", "norwaywest"]
///               }
///             }
///     options:
///       dependsOn:
///         - ${operator}
///         - ${owner}
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
///   example:
///     fn::invoke:
///       function: azure:core:getSubscription
///       arguments: {}
///   exampleGetDefinition:
///     fn::invoke:
///       function: azure:blueprint:getDefinition
///       arguments:
///         name: exampleBlueprint
///         scopeId: ${example.id}
///   exampleGetPublishedVersion:
///     fn::invoke:
///       function: azure:blueprint:getPublishedVersion
///       arguments:
///         scopeId: ${exampleGetDefinition.scopeId}
///         blueprintName: ${exampleGetDefinition.name}
///         version: v1.0.0
/// ```
///
/// ## Import
///
/// Azure Blueprint Assignments can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:blueprint/assignment:Assignment example "/subscriptions/00000000-0000-0000-0000-000000000000/providers/Microsoft.Blueprint/blueprintAssignments/assignSimpleBlueprint"
/// ```
///
pub mod assignment {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssignmentArgs {
        /// An `identity` block as defined below.
        #[builder(into)]
        pub identity: pulumi_wasm_rust::InputOrOutput<
            super::super::types::blueprint::AssignmentIdentity,
        >,
        /// The Azure location of the Assignment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// a list of up to 200 actions that are permitted to bypass the locks applied by the Blueprint.
        #[builder(into, default)]
        pub lock_exclude_actions: pulumi_wasm_rust::InputOrOutput<Option<Vec<String>>>,
        /// a list of up to 5 Principal IDs that are permitted to bypass the locks applied by the Blueprint.
        #[builder(into, default)]
        pub lock_exclude_principals: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The locking mode of the Blueprint Assignment. One of `None` (Default), `AllResourcesReadOnly`, or `AllResourcesDoNotDelete`. Defaults to `None`.
        #[builder(into, default)]
        pub lock_mode: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the Blueprint Assignment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// a JSON string to supply Blueprint Assignment parameter values.
        ///
        /// > **NOTE:** Improperly formatted JSON, or missing values required by a Blueprint will cause the assignment to fail.
        #[builder(into, default)]
        pub parameter_values: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// a JSON string to supply the Blueprint Resource Group information.
        ///
        /// > **NOTE:** Improperly formatted JSON, or missing values required by a Blueprint will cause the assignment to fail.
        #[builder(into, default)]
        pub resource_groups: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The Subscription ID the Blueprint Published Version is to be applied to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_subscription_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// The ID of the Published Version of the blueprint to be assigned.
        #[builder(into)]
        pub version_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AssignmentResult {
        /// The name of the blueprint assigned
        pub blueprint_name: pulumi_wasm_rust::Output<String>,
        /// The Description on the Blueprint
        pub description: pulumi_wasm_rust::Output<String>,
        /// The display name of the blueprint
        pub display_name: pulumi_wasm_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_wasm_rust::Output<
            super::super::types::blueprint::AssignmentIdentity,
        >,
        /// The Azure location of the Assignment. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// a list of up to 200 actions that are permitted to bypass the locks applied by the Blueprint.
        pub lock_exclude_actions: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// a list of up to 5 Principal IDs that are permitted to bypass the locks applied by the Blueprint.
        pub lock_exclude_principals: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        /// The locking mode of the Blueprint Assignment. One of `None` (Default), `AllResourcesReadOnly`, or `AllResourcesDoNotDelete`. Defaults to `None`.
        pub lock_mode: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the Blueprint Assignment. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// a JSON string to supply Blueprint Assignment parameter values.
        ///
        /// > **NOTE:** Improperly formatted JSON, or missing values required by a Blueprint will cause the assignment to fail.
        pub parameter_values: pulumi_wasm_rust::Output<Option<String>>,
        /// a JSON string to supply the Blueprint Resource Group information.
        ///
        /// > **NOTE:** Improperly formatted JSON, or missing values required by a Blueprint will cause the assignment to fail.
        pub resource_groups: pulumi_wasm_rust::Output<Option<String>>,
        /// The Subscription ID the Blueprint Published Version is to be applied to. Changing this forces a new resource to be created.
        pub target_subscription_id: pulumi_wasm_rust::Output<String>,
        pub type_: pulumi_wasm_rust::Output<String>,
        /// The ID of the Published Version of the blueprint to be assigned.
        pub version_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AssignmentArgs,
    ) -> AssignmentResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let identity_binding = args.identity.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let lock_exclude_actions_binding = args
            .lock_exclude_actions
            .get_output(context)
            .get_inner();
        let lock_exclude_principals_binding = args
            .lock_exclude_principals
            .get_output(context)
            .get_inner();
        let lock_mode_binding = args.lock_mode.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parameter_values_binding = args
            .parameter_values
            .get_output(context)
            .get_inner();
        let resource_groups_binding = args
            .resource_groups
            .get_output(context)
            .get_inner();
        let target_subscription_id_binding = args
            .target_subscription_id
            .get_output(context)
            .get_inner();
        let version_id_binding = args.version_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:blueprint/assignment:Assignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "lockExcludeActions".into(),
                    value: &lock_exclude_actions_binding,
                },
                register_interface::ObjectField {
                    name: "lockExcludePrincipals".into(),
                    value: &lock_exclude_principals_binding,
                },
                register_interface::ObjectField {
                    name: "lockMode".into(),
                    value: &lock_mode_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parameterValues".into(),
                    value: &parameter_values_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroups".into(),
                    value: &resource_groups_binding,
                },
                register_interface::ObjectField {
                    name: "targetSubscriptionId".into(),
                    value: &target_subscription_id_binding,
                },
                register_interface::ObjectField {
                    name: "versionId".into(),
                    value: &version_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AssignmentResult {
            blueprint_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("blueprintName"),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            display_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("displayName"),
            ),
            identity: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            lock_exclude_actions: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lockExcludeActions"),
            ),
            lock_exclude_principals: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lockExcludePrincipals"),
            ),
            lock_mode: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("lockMode"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parameter_values: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parameterValues"),
            ),
            resource_groups: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroups"),
            ),
            target_subscription_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("targetSubscriptionId"),
            ),
            type_: pulumi_wasm_rust::__private::into_domain(o.extract_field("type")),
            version_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("versionId"),
            ),
        }
    }
}

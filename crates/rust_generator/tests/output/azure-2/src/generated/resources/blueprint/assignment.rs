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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod assignment {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AssignmentArgs {
        /// An `identity` block as defined below.
        #[builder(into)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::blueprint::AssignmentIdentity,
        >,
        /// The Azure location of the Assignment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// a list of up to 200 actions that are permitted to bypass the locks applied by the Blueprint.
        #[builder(into, default)]
        pub lock_exclude_actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// a list of up to 5 Principal IDs that are permitted to bypass the locks applied by the Blueprint.
        #[builder(into, default)]
        pub lock_exclude_principals: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The locking mode of the Blueprint Assignment. One of `None` (Default), `AllResourcesReadOnly`, or `AllResourcesDoNotDelete`. Defaults to `None`.
        #[builder(into, default)]
        pub lock_mode: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Blueprint Assignment. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// a JSON string to supply Blueprint Assignment parameter values.
        ///
        /// > **NOTE:** Improperly formatted JSON, or missing values required by a Blueprint will cause the assignment to fail.
        #[builder(into, default)]
        pub parameter_values: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// a JSON string to supply the Blueprint Resource Group information.
        ///
        /// > **NOTE:** Improperly formatted JSON, or missing values required by a Blueprint will cause the assignment to fail.
        #[builder(into, default)]
        pub resource_groups: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Subscription ID the Blueprint Published Version is to be applied to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub target_subscription_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the Published Version of the blueprint to be assigned.
        #[builder(into)]
        pub version_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AssignmentResult {
        /// The name of the blueprint assigned
        pub blueprint_name: pulumi_gestalt_rust::Output<String>,
        /// The Description on the Blueprint
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The display name of the blueprint
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            super::super::types::blueprint::AssignmentIdentity,
        >,
        /// The Azure location of the Assignment. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// a list of up to 200 actions that are permitted to bypass the locks applied by the Blueprint.
        pub lock_exclude_actions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// a list of up to 5 Principal IDs that are permitted to bypass the locks applied by the Blueprint.
        pub lock_exclude_principals: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The locking mode of the Blueprint Assignment. One of `None` (Default), `AllResourcesReadOnly`, or `AllResourcesDoNotDelete`. Defaults to `None`.
        pub lock_mode: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Blueprint Assignment. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// a JSON string to supply Blueprint Assignment parameter values.
        ///
        /// > **NOTE:** Improperly formatted JSON, or missing values required by a Blueprint will cause the assignment to fail.
        pub parameter_values: pulumi_gestalt_rust::Output<Option<String>>,
        /// a JSON string to supply the Blueprint Resource Group information.
        ///
        /// > **NOTE:** Improperly formatted JSON, or missing values required by a Blueprint will cause the assignment to fail.
        pub resource_groups: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Subscription ID the Blueprint Published Version is to be applied to. Changing this forces a new resource to be created.
        pub target_subscription_id: pulumi_gestalt_rust::Output<String>,
        pub type_: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Published Version of the blueprint to be assigned.
        pub version_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AssignmentArgs,
    ) -> AssignmentResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let identity_binding = args.identity.get_output(context);
        let location_binding = args.location.get_output(context);
        let lock_exclude_actions_binding = args.lock_exclude_actions.get_output(context);
        let lock_exclude_principals_binding = args
            .lock_exclude_principals
            .get_output(context);
        let lock_mode_binding = args.lock_mode.get_output(context);
        let name_binding = args.name.get_output(context);
        let parameter_values_binding = args.parameter_values.get_output(context);
        let resource_groups_binding = args.resource_groups.get_output(context);
        let target_subscription_id_binding = args
            .target_subscription_id
            .get_output(context);
        let version_id_binding = args.version_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:blueprint/assignment:Assignment".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: &identity_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lockExcludeActions".into(),
                    value: &lock_exclude_actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lockExcludePrincipals".into(),
                    value: &lock_exclude_principals_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lockMode".into(),
                    value: &lock_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parameterValues".into(),
                    value: &parameter_values_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroups".into(),
                    value: &resource_groups_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetSubscriptionId".into(),
                    value: &target_subscription_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "versionId".into(),
                    value: &version_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AssignmentResult {
            blueprint_name: o.get_field("blueprintName"),
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            identity: o.get_field("identity"),
            location: o.get_field("location"),
            lock_exclude_actions: o.get_field("lockExcludeActions"),
            lock_exclude_principals: o.get_field("lockExcludePrincipals"),
            lock_mode: o.get_field("lockMode"),
            name: o.get_field("name"),
            parameter_values: o.get_field("parameterValues"),
            resource_groups: o.get_field("resourceGroups"),
            target_subscription_id: o.get_field("targetSubscriptionId"),
            type_: o.get_field("type"),
            version_id: o.get_field("versionId"),
        }
    }
}

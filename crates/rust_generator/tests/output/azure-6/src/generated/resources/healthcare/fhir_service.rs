/// Manages a Healthcare FHIR (Fast Healthcare Interoperability Resources) Service
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example
///       location: West Europe
///   exampleWorkspace:
///     type: azure:healthcare:Workspace
///     name: example
///     properties:
///       name: example
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///   exampleFhirService:
///     type: azure:healthcare:FhirService
///     name: example
///     properties:
///       name: tfexfhir
///       location: east us
///       resourceGroupName: tfex-resource_group
///       workspaceId: ${exampleWorkspace.id}
///       kind: fhir-R4
///       authentication:
///         authority: https://login.microsoftonline.com/tenantId
///         audience: https://tfexfhir.fhir.azurehealthcareapis.com
///       accessPolicyObjectIds:
///         - ${current.objectId}
///       identity:
///         type: SystemAssigned
///       containerRegistryLoginServerUrls:
///         - tfex-container_registry_login_server
///       cors:
///         allowedOrigins:
///           - https://tfex.com:123
///           - https://tfex1.com:3389
///         allowedHeaders:
///           - '*'
///         allowedMethods:
///           - GET
///           - DELETE
///           - PUT
///         maxAgeInSeconds: 3600
///         credentialsAllowed: true
///       configurationExportStorageAccountName: storage_account_name
/// variables:
///   current:
///     fn::invoke:
///       function: azure:core:getClientConfig
///       arguments: {}
/// ```
///
/// ## Import
///
/// Healthcare FHIR Service can be imported using the resource`id`, e.g.
///
/// ```sh
/// $ pulumi import azure:healthcare/fhirService:FhirService example /subscriptions/12345678-1234-9876-4563-123456789012/resourceGroups/group1/providers/Microsoft.HealthcareApis/workspaces/workspace1/fhirServices/service1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod fhir_service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FhirServiceArgs {
        /// A list of the access policies of the service instance.
        #[builder(into, default)]
        pub access_policy_object_ids: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// An `authentication` block as defined below.
        #[builder(into)]
        pub authentication: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::healthcare::FhirServiceAuthentication,
        >,
        /// Specifies the name of the storage account which the operation configuration information is exported to.
        #[builder(into, default)]
        pub configuration_export_storage_account_name: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// A list of azure container registry settings used for convert data operation of the service instance.
        #[builder(into, default)]
        pub container_registry_login_server_urls: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// A `cors` block as defined below.
        #[builder(into, default)]
        pub cors: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::healthcare::FhirServiceCors>,
        >,
        /// An `identity` block as defined below.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::healthcare::FhirServiceIdentity>,
        >,
        /// Specifies the kind of the Healthcare FHIR Service. Possible values are: `fhir-Stu3` and `fhir-R4`. Defaults to `fhir-R4`. Changing this forces a new Healthcare FHIR Service to be created.
        #[builder(into, default)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the Azure Region where the Healthcare FHIR Service should be created. Changing this forces a new Healthcare FHIR Service to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies the name of the Healthcare FHIR Service. Changing this forces a new Healthcare FHIR Service to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// [A list](https://www.terraform.io/docs/configuration/attr-as-blocks.html) of `oci_artifact` objects as defined below to describe [OCI artifacts for export](https://learn.microsoft.com/en-gb/azure/healthcare-apis/fhir/de-identified-export).
        #[builder(into, default)]
        pub oci_artifacts: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::healthcare::FhirServiceOciArtifact>>,
        >,
        /// Specifies the name of the Resource Group in which to create the Healthcare FHIR Service. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A mapping of tags to assign to the Healthcare FHIR Service.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the id of the Healthcare Workspace where the Healthcare FHIR Service should exist. Changing this forces a new Healthcare FHIR Service to be created.
        #[builder(into)]
        pub workspace_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FhirServiceResult {
        /// A list of the access policies of the service instance.
        pub access_policy_object_ids: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// An `authentication` block as defined below.
        pub authentication: pulumi_gestalt_rust::Output<
            super::super::types::healthcare::FhirServiceAuthentication,
        >,
        /// Specifies the name of the storage account which the operation configuration information is exported to.
        pub configuration_export_storage_account_name: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// A list of azure container registry settings used for convert data operation of the service instance.
        pub container_registry_login_server_urls: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// A `cors` block as defined below.
        pub cors: pulumi_gestalt_rust::Output<
            Option<super::super::types::healthcare::FhirServiceCors>,
        >,
        /// An `identity` block as defined below.
        pub identity: pulumi_gestalt_rust::Output<
            Option<super::super::types::healthcare::FhirServiceIdentity>,
        >,
        /// Specifies the kind of the Healthcare FHIR Service. Possible values are: `fhir-Stu3` and `fhir-R4`. Defaults to `fhir-R4`. Changing this forces a new Healthcare FHIR Service to be created.
        pub kind: pulumi_gestalt_rust::Output<Option<String>>,
        /// Specifies the Azure Region where the Healthcare FHIR Service should be created. Changing this forces a new Healthcare FHIR Service to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Healthcare FHIR Service. Changing this forces a new Healthcare FHIR Service to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// [A list](https://www.terraform.io/docs/configuration/attr-as-blocks.html) of `oci_artifact` objects as defined below to describe [OCI artifacts for export](https://learn.microsoft.com/en-gb/azure/healthcare-apis/fhir/de-identified-export).
        pub oci_artifacts: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::healthcare::FhirServiceOciArtifact>>,
        >,
        /// Whether public networks access is enabled.
        pub public_network_access_enabled: pulumi_gestalt_rust::Output<bool>,
        /// Specifies the name of the Resource Group in which to create the Healthcare FHIR Service. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A mapping of tags to assign to the Healthcare FHIR Service.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Specifies the id of the Healthcare Workspace where the Healthcare FHIR Service should exist. Changing this forces a new Healthcare FHIR Service to be created.
        pub workspace_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: FhirServiceArgs,
    ) -> FhirServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let access_policy_object_ids_binding_1 = args
            .access_policy_object_ids
            .get_output(context);
        let access_policy_object_ids_binding = access_policy_object_ids_binding_1
            .get_inner();
        let authentication_binding_1 = args.authentication.get_output(context);
        let authentication_binding = authentication_binding_1.get_inner();
        let configuration_export_storage_account_name_binding_1 = args
            .configuration_export_storage_account_name
            .get_output(context);
        let configuration_export_storage_account_name_binding = configuration_export_storage_account_name_binding_1
            .get_inner();
        let container_registry_login_server_urls_binding_1 = args
            .container_registry_login_server_urls
            .get_output(context);
        let container_registry_login_server_urls_binding = container_registry_login_server_urls_binding_1
            .get_inner();
        let cors_binding_1 = args.cors.get_output(context);
        let cors_binding = cors_binding_1.get_inner();
        let identity_binding_1 = args.identity.get_output(context);
        let identity_binding = identity_binding_1.get_inner();
        let kind_binding_1 = args.kind.get_output(context);
        let kind_binding = kind_binding_1.get_inner();
        let location_binding_1 = args.location.get_output(context);
        let location_binding = location_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let oci_artifacts_binding_1 = args.oci_artifacts.get_output(context);
        let oci_artifacts_binding = oci_artifacts_binding_1.get_inner();
        let resource_group_name_binding_1 = args.resource_group_name.get_output(context);
        let resource_group_name_binding = resource_group_name_binding_1.get_inner();
        let tags_binding_1 = args.tags.get_output(context);
        let tags_binding = tags_binding_1.get_inner();
        let workspace_id_binding_1 = args.workspace_id.get_output(context);
        let workspace_id_binding = workspace_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:healthcare/fhirService:FhirService".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessPolicyObjectIds".into(),
                    value: &access_policy_object_ids_binding,
                },
                register_interface::ObjectField {
                    name: "authentication".into(),
                    value: &authentication_binding,
                },
                register_interface::ObjectField {
                    name: "configurationExportStorageAccountName".into(),
                    value: &configuration_export_storage_account_name_binding,
                },
                register_interface::ObjectField {
                    name: "containerRegistryLoginServerUrls".into(),
                    value: &container_registry_login_server_urls_binding,
                },
                register_interface::ObjectField {
                    name: "cors".into(),
                    value: &cors_binding,
                },
                register_interface::ObjectField {
                    name: "identity".into(),
                    value: &identity_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
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
                    name: "ociArtifacts".into(),
                    value: &oci_artifacts_binding,
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
                    name: "workspaceId".into(),
                    value: &workspace_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FhirServiceResult {
            access_policy_object_ids: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("accessPolicyObjectIds"),
            ),
            authentication: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("authentication"),
            ),
            configuration_export_storage_account_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configurationExportStorageAccountName"),
            ),
            container_registry_login_server_urls: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("containerRegistryLoginServerUrls"),
            ),
            cors: pulumi_gestalt_rust::__private::into_domain(o.extract_field("cors")),
            identity: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("identity"),
            ),
            kind: pulumi_gestalt_rust::__private::into_domain(o.extract_field("kind")),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            oci_artifacts: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("ociArtifacts"),
            ),
            public_network_access_enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("publicNetworkAccessEnabled"),
            ),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
            workspace_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("workspaceId"),
            ),
        }
    }
}

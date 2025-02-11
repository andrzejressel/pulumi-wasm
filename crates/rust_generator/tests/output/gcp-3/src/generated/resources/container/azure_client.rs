/// AzureClient resources hold client authentication information needed by the Anthos Multi-Cloud API to manage Azure resources on your Azure subscription.When an AzureCluster is created, an AzureClient resource needs to be provided and all operations on Azure resources associated to that cluster will authenticate to Azure services using the given client.AzureClient resources are immutable and cannot be modified upon creation.Each AzureClient resource is bound to a single Azure Active Directory Application and tenant.
///
/// For more information, see:
/// * [Multicloud overview](https://cloud.google.com/kubernetes-engine/multi-cloud/docs)
/// ## Example Usage
///
/// ### Basic_azure_client
/// A basic example of a containerazure azure client
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let primary = azure_client::create(
///         "primary",
///         AzureClientArgs::builder()
///             .application_id("12345678-1234-1234-1234-123456789111")
///             .location("us-west1")
///             .name("client-name")
///             .project("my-project-name")
///             .tenant_id("12345678-1234-1234-1234-123456789111")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Client can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/locations/{{location}}/azureClients/{{name}}`
///
/// * `{{project}}/{{location}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Client can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:container/azureClient:AzureClient default projects/{{project}}/locations/{{location}}/azureClients/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/azureClient:AzureClient default {{project}}/{{location}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:container/azureClient:AzureClient default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod azure_client {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AzureClientArgs {
        /// The Azure Active Directory Application ID.
        #[builder(into)]
        pub application_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location for the resource
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of this resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The project for the resource
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Azure Active Directory Tenant ID.
        ///
        ///
        ///
        /// - - -
        #[builder(into)]
        pub tenant_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AzureClientResult {
        /// The Azure Active Directory Application ID.
        pub application_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. The PEM encoded x509 certificate.
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// Output only. The time at which this resource was created.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// The location for the resource
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of this resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The project for the resource
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The Azure Active Directory Tenant ID.
        ///
        ///
        ///
        /// - - -
        pub tenant_id: pulumi_gestalt_rust::Output<String>,
        /// Output only. A globally unique identifier for the client.
        pub uid: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AzureClientArgs,
    ) -> AzureClientResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let application_id_binding = args.application_id.get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let tenant_id_binding = args.tenant_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:container/azureClient:AzureClient".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "applicationId".into(),
                    value: &application_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tenantId".into(),
                    value: &tenant_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AzureClientResult {
            application_id: o.get_field("applicationId"),
            certificate: o.get_field("certificate"),
            create_time: o.get_field("createTime"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            tenant_id: o.get_field("tenantId"),
            uid: o.get_field("uid"),
        }
    }
}

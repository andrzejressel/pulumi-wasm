/// Manages a Federated Identity Credential.
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
///             .name("example")
///             .build_struct(),
///     );
///     let exampleFederatedIdentityCredential = federated_identity_credential::create(
///         "exampleFederatedIdentityCredential",
///         FederatedIdentityCredentialArgs::builder()
///             .audience("foo")
///             .issuer("https://foo")
///             .name("example")
///             .parent_id("${exampleUserAssignedIdentity.id}")
///             .resource_group_name("${example.name}")
///             .subject("foo")
///             .build_struct(),
///     );
///     let exampleUserAssignedIdentity = user_assigned_identity::create(
///         "exampleUserAssignedIdentity",
///         UserAssignedIdentityArgs::builder()
///             .location("${example.location}")
///             .name("example")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// An existing Federated Identity Credential can be imported into Pulumi using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:armmsi/federatedIdentityCredential:FederatedIdentityCredential example /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{parentIdentityName}/federatedIdentityCredentials/{resourceName}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod federated_identity_credential {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FederatedIdentityCredentialArgs {
        /// Specifies the audience for this Federated Identity Credential.
        #[builder(into)]
        pub audience: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the issuer of this Federated Identity Credential.
        #[builder(into)]
        pub issuer: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of this Federated Identity Credential. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Specifies parent ID of User Assigned Identity for this Federated Identity Credential. Changing this forces a new Federated Identity Credential to be created.
        #[builder(into)]
        pub parent_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group within which this Federated Identity Credential should exist. Changing this forces a new Federated Identity Credential to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the subject for this Federated Identity Credential.
        #[builder(into)]
        pub subject: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FederatedIdentityCredentialResult {
        /// Specifies the audience for this Federated Identity Credential.
        pub audience: pulumi_gestalt_rust::Output<String>,
        /// Specifies the issuer of this Federated Identity Credential.
        pub issuer: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of this Federated Identity Credential. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Specifies parent ID of User Assigned Identity for this Federated Identity Credential. Changing this forces a new Federated Identity Credential to be created.
        pub parent_id: pulumi_gestalt_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this Federated Identity Credential should exist. Changing this forces a new Federated Identity Credential to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// Specifies the subject for this Federated Identity Credential.
        pub subject: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FederatedIdentityCredentialArgs,
    ) -> FederatedIdentityCredentialResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let audience_binding = args.audience.get_output(context);
        let issuer_binding = args.issuer.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_id_binding = args.parent_id.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let subject_binding = args.subject.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:armmsi/federatedIdentityCredential:FederatedIdentityCredential"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "audience".into(),
                    value: &audience_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "issuer".into(),
                    value: &issuer_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "subject".into(),
                    value: &subject_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FederatedIdentityCredentialResult {
            audience: o.get_field("audience"),
            issuer: o.get_field("issuer"),
            name: o.get_field("name"),
            parent_id: o.get_field("parentId"),
            resource_group_name: o.get_field("resourceGroupName"),
            subject: o.get_field("subject"),
        }
    }
}

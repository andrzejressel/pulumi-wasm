/// Manages a Federated Identity Credential.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod federated_identity_credential {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FederatedIdentityCredentialArgs {
        /// Specifies the audience for this Federated Identity Credential.
        #[builder(into)]
        pub audience: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the issuer of this Federated Identity Credential.
        #[builder(into)]
        pub issuer: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of this Federated Identity Credential. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Specifies parent ID of User Assigned Identity for this Federated Identity Credential. Changing this forces a new Federated Identity Credential to be created.
        #[builder(into)]
        pub parent_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the name of the Resource Group within which this Federated Identity Credential should exist. Changing this forces a new Federated Identity Credential to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Specifies the subject for this Federated Identity Credential.
        #[builder(into)]
        pub subject: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FederatedIdentityCredentialResult {
        /// Specifies the audience for this Federated Identity Credential.
        pub audience: pulumi_wasm_rust::Output<String>,
        /// Specifies the issuer of this Federated Identity Credential.
        pub issuer: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of this Federated Identity Credential. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Specifies parent ID of User Assigned Identity for this Federated Identity Credential. Changing this forces a new Federated Identity Credential to be created.
        pub parent_id: pulumi_wasm_rust::Output<String>,
        /// Specifies the name of the Resource Group within which this Federated Identity Credential should exist. Changing this forces a new Federated Identity Credential to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// Specifies the subject for this Federated Identity Credential.
        pub subject: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: FederatedIdentityCredentialArgs,
    ) -> FederatedIdentityCredentialResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let audience_binding = args.audience.get_output(context).get_inner();
        let issuer_binding = args.issuer.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let parent_id_binding = args.parent_id.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let subject_binding = args.subject.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:armmsi/federatedIdentityCredential:FederatedIdentityCredential"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "audience".into(),
                    value: &audience_binding,
                },
                register_interface::ObjectField {
                    name: "issuer".into(),
                    value: &issuer_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "parentId".into(),
                    value: &parent_id_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "subject".into(),
                    value: &subject_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        FederatedIdentityCredentialResult {
            audience: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("audience"),
            ),
            issuer: pulumi_wasm_rust::__private::into_domain(o.extract_field("issuer")),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            parent_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("parentId"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            subject: pulumi_wasm_rust::__private::into_domain(o.extract_field("subject")),
        }
    }
}

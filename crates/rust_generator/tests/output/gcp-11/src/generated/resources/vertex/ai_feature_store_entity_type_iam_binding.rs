///
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * {{featurestore}}/entityTypes/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Vertex AI featurestoreentitytype IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureStoreEntityTypeIamBinding:AiFeatureStoreEntityTypeIamBinding editor "{{featurestore}}/entityTypes/{{featurestore_entitytype}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureStoreEntityTypeIamBinding:AiFeatureStoreEntityTypeIamBinding editor "{{featurestore}}/entityTypes/{{featurestore_entitytype}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:vertex/aiFeatureStoreEntityTypeIamBinding:AiFeatureStoreEntityTypeIamBinding editor {{featurestore}}/entityTypes/{{featurestore_entitytype}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
pub mod ai_feature_store_entity_type_iam_binding {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AiFeatureStoreEntityTypeIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_wasm_rust::InputOrOutput<
            Option<
                super::super::types::vertex::AiFeatureStoreEntityTypeIamBindingCondition,
            >,
        >,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub entitytype: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the Featurestore to use, in the format projects/{project}/locations/{location}/featurestores/{featurestore}. Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub featurestore: pulumi_wasm_rust::InputOrOutput<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        #[builder(into)]
        pub members: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.vertex.AiFeatureStoreEntityTypeIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AiFeatureStoreEntityTypeIamBindingResult {
        pub condition: pulumi_wasm_rust::Output<
            Option<
                super::super::types::vertex::AiFeatureStoreEntityTypeIamBindingCondition,
            >,
        >,
        /// Used to find the parent resource to bind the IAM policy to
        pub entitytype: pulumi_wasm_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_wasm_rust::Output<String>,
        /// The name of the Featurestore to use, in the format projects/{project}/locations/{location}/featurestores/{featurestore}. Used to find the parent resource to bind the IAM policy to
        pub featurestore: pulumi_wasm_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        pub members: pulumi_wasm_rust::Output<Vec<String>>,
        /// The role that should be applied. Only one
        /// `gcp.vertex.AiFeatureStoreEntityTypeIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AiFeatureStoreEntityTypeIamBindingArgs,
    ) -> AiFeatureStoreEntityTypeIamBindingResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let condition_binding = args.condition.get_output(context).get_inner();
        let entitytype_binding = args.entitytype.get_output(context).get_inner();
        let featurestore_binding = args.featurestore.get_output(context).get_inner();
        let members_binding = args.members.get_output(context).get_inner();
        let role_binding = args.role.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:vertex/aiFeatureStoreEntityTypeIamBinding:AiFeatureStoreEntityTypeIamBinding"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "condition".into(),
                    value: &condition_binding,
                },
                register_interface::ObjectField {
                    name: "entitytype".into(),
                    value: &entitytype_binding,
                },
                register_interface::ObjectField {
                    name: "featurestore".into(),
                    value: &featurestore_binding,
                },
                register_interface::ObjectField {
                    name: "members".into(),
                    value: &members_binding,
                },
                register_interface::ObjectField {
                    name: "role".into(),
                    value: &role_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        AiFeatureStoreEntityTypeIamBindingResult {
            condition: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("condition"),
            ),
            entitytype: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("entitytype"),
            ),
            etag: pulumi_wasm_rust::__private::into_domain(o.extract_field("etag")),
            featurestore: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("featurestore"),
            ),
            members: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("members"),
            ),
            role: pulumi_wasm_rust::__private::into_domain(o.extract_field("role")),
        }
    }
}

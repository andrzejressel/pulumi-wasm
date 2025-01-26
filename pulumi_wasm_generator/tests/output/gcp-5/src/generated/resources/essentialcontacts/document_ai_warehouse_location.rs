/// A location is used to initialize a project.
///
///
/// To get more information about Location, see:
///
/// * [API documentation](https://cloud.google.com/document-warehouse/docs/reference/rest/v1/projects.locations)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/document-warehouse/docs/overview)
///
/// ## Example Usage
///
/// ### Document Ai Warehouse Location
///
///
/// ```yaml
/// resources:
///   example:
///     type: gcp:essentialcontacts:DocumentAiWarehouseLocation
///     properties:
///       location: us
///       projectNumber: ${project.number}
///       accessControlMode: ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_GCI
///       databaseType: DB_INFRA_SPANNER
///       kmsKey: dummy_key
///       documentCreatorDefaultRole: DOCUMENT_ADMIN
/// variables:
///   project:
///     fn::invoke:
///       function: gcp:organizations:getProject
///       arguments: {}
/// ```
///
/// ## Import
///
/// This resource does not support import.
///
pub mod document_ai_warehouse_location {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DocumentAiWarehouseLocationArgs {
        /// The access control mode for accessing the customer data.
        /// Possible values are: `ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_GCI`, `ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_BYOID`, `ACL_MODE_UNIVERSAL_ACCESS`.
        #[builder(into)]
        pub access_control_mode: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of database used to store customer data.
        /// Possible values are: `DB_INFRA_SPANNER`, `DB_CLOUD_SQL_POSTGRES`.
        #[builder(into)]
        pub database_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The default role for the person who create a document.
        /// Possible values are: `DOCUMENT_ADMIN`, `DOCUMENT_EDITOR`, `DOCUMENT_VIEWER`.
        #[builder(into, default)]
        pub document_creator_default_role: pulumi_wasm_rust::InputOrOutput<
            Option<String>,
        >,
        /// The KMS key used for CMEK encryption. It is required that
        /// the kms key is in the same region as the endpoint. The
        /// same key will be used for all provisioned resources, if
        /// encryption is available. If the kmsKey is left empty, no
        /// encryption will be enforced.
        #[builder(into, default)]
        pub kms_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The location in which the instance is to be provisioned. It takes the form projects/{projectNumber}/locations/{location}.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub location: pulumi_wasm_rust::InputOrOutput<String>,
        /// The unique identifier of the project.
        #[builder(into)]
        pub project_number: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DocumentAiWarehouseLocationResult {
        /// The access control mode for accessing the customer data.
        /// Possible values are: `ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_GCI`, `ACL_MODE_DOCUMENT_LEVEL_ACCESS_CONTROL_BYOID`, `ACL_MODE_UNIVERSAL_ACCESS`.
        pub access_control_mode: pulumi_wasm_rust::Output<String>,
        /// The type of database used to store customer data.
        /// Possible values are: `DB_INFRA_SPANNER`, `DB_CLOUD_SQL_POSTGRES`.
        pub database_type: pulumi_wasm_rust::Output<String>,
        /// The default role for the person who create a document.
        /// Possible values are: `DOCUMENT_ADMIN`, `DOCUMENT_EDITOR`, `DOCUMENT_VIEWER`.
        pub document_creator_default_role: pulumi_wasm_rust::Output<Option<String>>,
        /// The KMS key used for CMEK encryption. It is required that
        /// the kms key is in the same region as the endpoint. The
        /// same key will be used for all provisioned resources, if
        /// encryption is available. If the kmsKey is left empty, no
        /// encryption will be enforced.
        pub kms_key: pulumi_wasm_rust::Output<Option<String>>,
        /// The location in which the instance is to be provisioned. It takes the form projects/{projectNumber}/locations/{location}.
        ///
        ///
        /// - - -
        pub location: pulumi_wasm_rust::Output<String>,
        /// The unique identifier of the project.
        pub project_number: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: DocumentAiWarehouseLocationArgs,
    ) -> DocumentAiWarehouseLocationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let access_control_mode_binding = args
            .access_control_mode
            .get_output(context)
            .get_inner();
        let database_type_binding = args.database_type.get_output(context).get_inner();
        let document_creator_default_role_binding = args
            .document_creator_default_role
            .get_output(context)
            .get_inner();
        let kms_key_binding = args.kms_key.get_output(context).get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let project_number_binding = args.project_number.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:essentialcontacts/documentAiWarehouseLocation:DocumentAiWarehouseLocation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accessControlMode".into(),
                    value: &access_control_mode_binding,
                },
                register_interface::ObjectField {
                    name: "databaseType".into(),
                    value: &database_type_binding,
                },
                register_interface::ObjectField {
                    name: "documentCreatorDefaultRole".into(),
                    value: &document_creator_default_role_binding,
                },
                register_interface::ObjectField {
                    name: "kmsKey".into(),
                    value: &kms_key_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "projectNumber".into(),
                    value: &project_number_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessControlMode".into(),
                },
                register_interface::ResultField {
                    name: "databaseType".into(),
                },
                register_interface::ResultField {
                    name: "documentCreatorDefaultRole".into(),
                },
                register_interface::ResultField {
                    name: "kmsKey".into(),
                },
                register_interface::ResultField {
                    name: "location".into(),
                },
                register_interface::ResultField {
                    name: "projectNumber".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        DocumentAiWarehouseLocationResult {
            access_control_mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessControlMode").unwrap(),
            ),
            database_type: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("databaseType").unwrap(),
            ),
            document_creator_default_role: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("documentCreatorDefaultRole").unwrap(),
            ),
            kms_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kmsKey").unwrap(),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("location").unwrap(),
            ),
            project_number: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("projectNumber").unwrap(),
            ),
        }
    }
}

/// Creates a Signer Signing Profile Permission. That is, a cross-account permission for a signing profile.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   prodSp:
///     type: aws:signer:SigningProfile
///     name: prod_sp
///     properties:
///       platformId: AWSLambda-SHA384-ECDSA
///       namePrefix: prod_sp_
///       signatureValidityPeriod:
///         value: 5
///         type: YEARS
///       tags:
///         tag1: value1
///         tag2: value2
///   spPermission1:
///     type: aws:signer:SigningProfilePermission
///     name: sp_permission_1
///     properties:
///       profileName: ${prodSp.name}
///       action: signer:StartSigningJob
///       principal: ${awsAccount}
///   spPermission2:
///     type: aws:signer:SigningProfilePermission
///     name: sp_permission_2
///     properties:
///       profileName: ${prodSp.name}
///       action: signer:GetSigningProfile
///       principal: ${awsTeamRoleArn}
///       statementId: ProdAccountStartSigningJob_StatementId
///   spPermission3:
///     type: aws:signer:SigningProfilePermission
///     name: sp_permission_3
///     properties:
///       profileName: ${prodSp.name}
///       action: signer:RevokeSignature
///       principal: '123456789012'
///       profileVersion: ${prodSp.version}
///       statementIdPrefix: version-permission-
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Signer signing profile permission statements using profile_name/statement_id. For example:
///
/// ```sh
/// $ pulumi import aws:signer/signingProfilePermission:SigningProfilePermission test_signer_signing_profile_permission prod_profile_DdW3Mk1foYL88fajut4mTVFGpuwfd4ACO6ANL0D1uIj7lrn8adK/ProdAccountStartSigningJobStatementId
/// ```
pub mod signing_profile_permission {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SigningProfilePermissionArgs {
        /// An AWS Signer action permitted as part of cross-account permissions. Valid values: `signer:StartSigningJob`, `signer:GetSigningProfile`, `signer:RevokeSignature`, or `signer:SignPayload`.
        #[builder(into)]
        pub action: pulumi_wasm_rust::Output<String>,
        /// The AWS principal to be granted a cross-account permission.
        #[builder(into)]
        pub principal: pulumi_wasm_rust::Output<String>,
        /// Name of the signing profile to add the cross-account permissions.
        #[builder(into)]
        pub profile_name: pulumi_wasm_rust::Output<String>,
        /// The signing profile version that a permission applies to.
        #[builder(into, default)]
        pub profile_version: pulumi_wasm_rust::Output<Option<String>>,
        /// A unique statement identifier. By default generated by the provider.
        #[builder(into, default)]
        pub statement_id: pulumi_wasm_rust::Output<Option<String>>,
        /// A statement identifier prefix. The provider will generate a unique suffix. Conflicts with `statement_id`.
        #[builder(into, default)]
        pub statement_id_prefix: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct SigningProfilePermissionResult {
        /// An AWS Signer action permitted as part of cross-account permissions. Valid values: `signer:StartSigningJob`, `signer:GetSigningProfile`, `signer:RevokeSignature`, or `signer:SignPayload`.
        pub action: pulumi_wasm_rust::Output<String>,
        /// The AWS principal to be granted a cross-account permission.
        pub principal: pulumi_wasm_rust::Output<String>,
        /// Name of the signing profile to add the cross-account permissions.
        pub profile_name: pulumi_wasm_rust::Output<String>,
        /// The signing profile version that a permission applies to.
        pub profile_version: pulumi_wasm_rust::Output<String>,
        /// A unique statement identifier. By default generated by the provider.
        pub statement_id: pulumi_wasm_rust::Output<String>,
        /// A statement identifier prefix. The provider will generate a unique suffix. Conflicts with `statement_id`.
        pub statement_id_prefix: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SigningProfilePermissionArgs,
    ) -> SigningProfilePermissionResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let action_binding = args.action.get_inner();
        let principal_binding = args.principal.get_inner();
        let profile_name_binding = args.profile_name.get_inner();
        let profile_version_binding = args.profile_version.get_inner();
        let statement_id_binding = args.statement_id.get_inner();
        let statement_id_prefix_binding = args.statement_id_prefix.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:signer/signingProfilePermission:SigningProfilePermission".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "action".into(),
                    value: &action_binding,
                },
                register_interface::ObjectField {
                    name: "principal".into(),
                    value: &principal_binding,
                },
                register_interface::ObjectField {
                    name: "profileName".into(),
                    value: &profile_name_binding,
                },
                register_interface::ObjectField {
                    name: "profileVersion".into(),
                    value: &profile_version_binding,
                },
                register_interface::ObjectField {
                    name: "statementId".into(),
                    value: &statement_id_binding,
                },
                register_interface::ObjectField {
                    name: "statementIdPrefix".into(),
                    value: &statement_id_prefix_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "action".into(),
                },
                register_interface::ResultField {
                    name: "principal".into(),
                },
                register_interface::ResultField {
                    name: "profileName".into(),
                },
                register_interface::ResultField {
                    name: "profileVersion".into(),
                },
                register_interface::ResultField {
                    name: "statementId".into(),
                },
                register_interface::ResultField {
                    name: "statementIdPrefix".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SigningProfilePermissionResult {
            action: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("action").unwrap(),
            ),
            principal: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("principal").unwrap(),
            ),
            profile_name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileName").unwrap(),
            ),
            profile_version: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("profileVersion").unwrap(),
            ),
            statement_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statementId").unwrap(),
            ),
            statement_id_prefix: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("statementIdPrefix").unwrap(),
            ),
        }
    }
}
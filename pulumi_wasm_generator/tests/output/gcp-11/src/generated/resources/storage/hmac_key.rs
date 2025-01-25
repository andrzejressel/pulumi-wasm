/// The hmacKeys resource represents an HMAC key within Cloud Storage. The resource
/// consists of a secret and HMAC key metadata. HMAC keys can be used as credentials
/// for service accounts.
///
///
/// To get more information about HmacKey, see:
///
/// * [API documentation](https://cloud.google.com/storage/docs/json_api/v1/projects/hmacKeys)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/storage/docs/authentication/managing-hmackeys)
///
///
///
/// ## Example Usage
///
/// ### Storage Hmac Key
///
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let key = hmac_key::create(
///         "key",
///         HmacKeyArgs::builder()
///             .service_account_email("${serviceAccount.email}")
///             .build_struct(),
///     );
///     let serviceAccount = account::create(
///         "serviceAccount",
///         AccountArgs::builder().account_id("my-svc-acc").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// HmacKey can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/hmacKeys/{{access_id}}`
///
/// * `{{project}}/{{access_id}}`
///
/// * `{{access_id}}`
///
/// When using the `pulumi import` command, HmacKey can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:storage/hmacKey:HmacKey default projects/{{project}}/hmacKeys/{{access_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:storage/hmacKey:HmacKey default {{project}}/{{access_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:storage/hmacKey:HmacKey default {{access_id}}
/// ```
///
pub mod hmac_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HmacKeyArgs {
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The email address of the key's associated service account.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub service_account_email: pulumi_wasm_rust::InputOrOutput<String>,
        /// The state of the key. Can be set to one of ACTIVE, INACTIVE.
        /// Default value is `ACTIVE`.
        /// Possible values are: `ACTIVE`, `INACTIVE`.
        #[builder(into, default)]
        pub state: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct HmacKeyResult {
        /// The access ID of the HMAC Key.
        pub access_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_wasm_rust::Output<String>,
        /// HMAC secret key material.
        /// **Note**: This property is sensitive and will not be displayed in the plan.
        pub secret: pulumi_wasm_rust::Output<String>,
        /// The email address of the key's associated service account.
        ///
        ///
        /// - - -
        pub service_account_email: pulumi_wasm_rust::Output<String>,
        /// The state of the key. Can be set to one of ACTIVE, INACTIVE.
        /// Default value is `ACTIVE`.
        /// Possible values are: `ACTIVE`, `INACTIVE`.
        pub state: pulumi_wasm_rust::Output<Option<String>>,
        /// 'The creation time of the HMAC key in RFC 3339 format. '
        pub time_created: pulumi_wasm_rust::Output<String>,
        /// 'The last modification time of the HMAC key metadata in RFC 3339 format.'
        pub updated: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: HmacKeyArgs,
    ) -> HmacKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let project_binding = args.project.get_output(context).get_inner();
        let service_account_email_binding = args
            .service_account_email
            .get_output(context)
            .get_inner();
        let state_binding = args.state.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:storage/hmacKey:HmacKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "serviceAccountEmail".into(),
                    value: &service_account_email_binding,
                },
                register_interface::ObjectField {
                    name: "state".into(),
                    value: &state_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accessId".into(),
                },
                register_interface::ResultField {
                    name: "project".into(),
                },
                register_interface::ResultField {
                    name: "secret".into(),
                },
                register_interface::ResultField {
                    name: "serviceAccountEmail".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "timeCreated".into(),
                },
                register_interface::ResultField {
                    name: "updated".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        HmacKeyResult {
            access_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accessId").unwrap(),
            ),
            project: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("project").unwrap(),
            ),
            secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secret").unwrap(),
            ),
            service_account_email: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("serviceAccountEmail").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            time_created: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("timeCreated").unwrap(),
            ),
            updated: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("updated").unwrap(),
            ),
        }
    }
}

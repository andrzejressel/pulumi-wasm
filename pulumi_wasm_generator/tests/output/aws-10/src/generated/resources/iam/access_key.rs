/// Provides an IAM access key. This is a set of credentials that allow API requests to be made as an IAM user.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   lb:
///     type: aws:iam:AccessKey
///     properties:
///       user: ${lbUser.name}
///       pgpKey: keybase:some_person_that_exists
///   lbUser:
///     type: aws:iam:User
///     name: lb
///     properties:
///       name: loadbalancer
///       path: /system/
///   lbRoUserPolicy:
///     type: aws:iam:UserPolicy
///     name: lb_ro
///     properties:
///       name: test
///       user: ${lbUser.name}
///       policy: ${lbRo.json}
/// variables:
///   lbRo:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - ec2:Describe*
///             resources:
///               - '*'
/// outputs:
///   secret: ${lb.encryptedSecret}
/// ```
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = user::create(
///         "test",
///         UserArgs::builder().name("test").path("/test/").build_struct(),
///     );
///     let testAccessKey = access_key::create(
///         "testAccessKey",
///         AccessKeyArgs::builder().user("${test.name}").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import IAM Access Keys using the identifier. For example:
///
/// ```sh
/// $ pulumi import aws:iam/accessKey:AccessKey example AKIA1234567890
/// ```
/// Resource attributes such as `encrypted_secret`, `key_fingerprint`, `pgp_key`, `secret`, `ses_smtp_password_v4`, and `encrypted_ses_smtp_password_v4` are not available for imported resources as this information cannot be read from the IAM API.
///
pub mod access_key {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessKeyArgs {
        /// Either a base-64 encoded PGP public key, or a keybase username in the form `keybase:some_person_that_exists`, for use in the `encrypted_secret` output attribute. If providing a base-64 encoded PGP public key, make sure to provide the "raw" version and not the "armored" one (e.g. avoid passing the `-a` option to `gpg --export`).
        #[builder(into, default)]
        pub pgp_key: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Access key status to apply. Defaults to `Active`. Valid values are `Active` and `Inactive`.
        #[builder(into, default)]
        pub status: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// IAM user to associate with this access key.
        #[builder(into)]
        pub user: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessKeyResult {
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the access key was created.
        pub create_date: pulumi_wasm_rust::Output<String>,
        /// Encrypted secret, base64 encoded, if `pgp_key` was specified. This attribute is not available for imported resources. The encrypted secret may be decrypted using the command line.
        pub encrypted_secret: pulumi_wasm_rust::Output<String>,
        /// Encrypted SES SMTP password, base64 encoded, if `pgp_key` was specified. This attribute is not available for imported resources. The encrypted password may be decrypted using the command line.
        pub encrypted_ses_smtp_password_v4: pulumi_wasm_rust::Output<String>,
        /// Fingerprint of the PGP key used to encrypt the secret. This attribute is not available for imported resources.
        pub key_fingerprint: pulumi_wasm_rust::Output<String>,
        /// Either a base-64 encoded PGP public key, or a keybase username in the form `keybase:some_person_that_exists`, for use in the `encrypted_secret` output attribute. If providing a base-64 encoded PGP public key, make sure to provide the "raw" version and not the "armored" one (e.g. avoid passing the `-a` option to `gpg --export`).
        pub pgp_key: pulumi_wasm_rust::Output<Option<String>>,
        /// Secret access key. This attribute is not available for imported resources. Note that this will be written to the state file. If you use this, please protect your backend state file judiciously. Alternatively, you may supply a `pgp_key` instead, which will prevent the secret from being stored in plaintext, at the cost of preventing the use of the secret key in automation.
        pub secret: pulumi_wasm_rust::Output<String>,
        /// Secret access key converted into an SES SMTP password by applying [AWS's documented Sigv4 conversion algorithm](https://docs.aws.amazon.com/ses/latest/DeveloperGuide/smtp-credentials.html#smtp-credentials-convert). This attribute is not available for imported resources. As SigV4 is region specific, valid Provider regions are `ap-south-1`, `ap-southeast-2`, `eu-central-1`, `eu-west-1`, `us-east-1` and `us-west-2`. See current [AWS SES regions](https://docs.aws.amazon.com/general/latest/gr/rande.html#ses_region).
        pub ses_smtp_password_v4: pulumi_wasm_rust::Output<String>,
        /// Access key status to apply. Defaults to `Active`. Valid values are `Active` and `Inactive`.
        pub status: pulumi_wasm_rust::Output<Option<String>>,
        /// IAM user to associate with this access key.
        pub user: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: AccessKeyArgs,
    ) -> AccessKeyResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let pgp_key_binding = args.pgp_key.get_output(context).get_inner();
        let status_binding = args.status.get_output(context).get_inner();
        let user_binding = args.user.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:iam/accessKey:AccessKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "pgpKey".into(),
                    value: &pgp_key_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "user".into(),
                    value: &user_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "createDate".into(),
                },
                register_interface::ResultField {
                    name: "encryptedSecret".into(),
                },
                register_interface::ResultField {
                    name: "encryptedSesSmtpPasswordV4".into(),
                },
                register_interface::ResultField {
                    name: "keyFingerprint".into(),
                },
                register_interface::ResultField {
                    name: "pgpKey".into(),
                },
                register_interface::ResultField {
                    name: "secret".into(),
                },
                register_interface::ResultField {
                    name: "sesSmtpPasswordV4".into(),
                },
                register_interface::ResultField {
                    name: "status".into(),
                },
                register_interface::ResultField {
                    name: "user".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        AccessKeyResult {
            create_date: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("createDate").unwrap(),
            ),
            encrypted_secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptedSecret").unwrap(),
            ),
            encrypted_ses_smtp_password_v4: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("encryptedSesSmtpPasswordV4").unwrap(),
            ),
            key_fingerprint: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("keyFingerprint").unwrap(),
            ),
            pgp_key: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("pgpKey").unwrap(),
            ),
            secret: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("secret").unwrap(),
            ),
            ses_smtp_password_v4: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("sesSmtpPasswordV4").unwrap(),
            ),
            status: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("status").unwrap(),
            ),
            user: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("user").unwrap(),
            ),
        }
    }
}

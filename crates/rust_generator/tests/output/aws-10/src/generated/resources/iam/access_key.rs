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
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_key {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessKeyArgs {
        /// Either a base-64 encoded PGP public key, or a keybase username in the form `keybase:some_person_that_exists`, for use in the `encrypted_secret` output attribute. If providing a base-64 encoded PGP public key, make sure to provide the "raw" version and not the "armored" one (e.g. avoid passing the `-a` option to `gpg --export`).
        #[builder(into, default)]
        pub pgp_key: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Access key status to apply. Defaults to `Active`. Valid values are `Active` and `Inactive`.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IAM user to associate with this access key.
        #[builder(into)]
        pub user: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AccessKeyResult {
        /// Date and time in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) that the access key was created.
        pub create_date: pulumi_gestalt_rust::Output<String>,
        /// Encrypted secret, base64 encoded, if `pgp_key` was specified. This attribute is not available for imported resources. The encrypted secret may be decrypted using the command line.
        pub encrypted_secret: pulumi_gestalt_rust::Output<String>,
        /// Encrypted SES SMTP password, base64 encoded, if `pgp_key` was specified. This attribute is not available for imported resources. The encrypted password may be decrypted using the command line.
        pub encrypted_ses_smtp_password_v4: pulumi_gestalt_rust::Output<String>,
        /// Fingerprint of the PGP key used to encrypt the secret. This attribute is not available for imported resources.
        pub key_fingerprint: pulumi_gestalt_rust::Output<String>,
        /// Either a base-64 encoded PGP public key, or a keybase username in the form `keybase:some_person_that_exists`, for use in the `encrypted_secret` output attribute. If providing a base-64 encoded PGP public key, make sure to provide the "raw" version and not the "armored" one (e.g. avoid passing the `-a` option to `gpg --export`).
        pub pgp_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Secret access key. This attribute is not available for imported resources. Note that this will be written to the state file. If you use this, please protect your backend state file judiciously. Alternatively, you may supply a `pgp_key` instead, which will prevent the secret from being stored in plaintext, at the cost of preventing the use of the secret key in automation.
        pub secret: pulumi_gestalt_rust::Output<String>,
        /// Secret access key converted into an SES SMTP password by applying [AWS's documented Sigv4 conversion algorithm](https://docs.aws.amazon.com/ses/latest/DeveloperGuide/smtp-credentials.html#smtp-credentials-convert). This attribute is not available for imported resources. As SigV4 is region specific, valid Provider regions are `ap-south-1`, `ap-southeast-2`, `eu-central-1`, `eu-west-1`, `us-east-1` and `us-west-2`. See current [AWS SES regions](https://docs.aws.amazon.com/general/latest/gr/rande.html#ses_region).
        pub ses_smtp_password_v4: pulumi_gestalt_rust::Output<String>,
        /// Access key status to apply. Defaults to `Active`. Valid values are `Active` and `Inactive`.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// IAM user to associate with this access key.
        pub user: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessKeyArgs,
    ) -> AccessKeyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let pgp_key_binding = args.pgp_key.get_output(context);
        let status_binding = args.status.get_output(context);
        let user_binding = args.user.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iam/accessKey:AccessKey".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pgpKey".into(),
                    value: &pgp_key_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "user".into(),
                    value: &user_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AccessKeyResult {
            create_date: o.get_field("createDate"),
            encrypted_secret: o.get_field("encryptedSecret"),
            encrypted_ses_smtp_password_v4: o.get_field("encryptedSesSmtpPasswordV4"),
            key_fingerprint: o.get_field("keyFingerprint"),
            pgp_key: o.get_field("pgpKey"),
            secret: o.get_field("secret"),
            ses_smtp_password_v4: o.get_field("sesSmtpPasswordV4"),
            status: o.get_field("status"),
            user: o.get_field("user"),
        }
    }
}

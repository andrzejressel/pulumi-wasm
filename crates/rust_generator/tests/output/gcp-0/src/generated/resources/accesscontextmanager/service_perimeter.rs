/// ServicePerimeter describes a set of GCP resources which can freely import
/// and export data amongst themselves, but not export outside of the
/// ServicePerimeter. If a request with a source within this ServicePerimeter
/// has a target outside of the ServicePerimeter, the request will be blocked.
/// Otherwise the request is allowed. There are two types of Service Perimeter
/// - Regular and Bridge. Regular Service Perimeters cannot overlap, a single
/// GCP project can only belong to a single regular Service Perimeter. Service
/// Perimeter Bridges can contain only GCP projects as members, a single GCP
/// project may belong to multiple Service Perimeter Bridges.
///
///
/// To get more information about ServicePerimeter, see:
///
/// * [API documentation](https://cloud.google.com/access-context-manager/docs/reference/rest/v1/accessPolicies.servicePerimeters)
/// * How-to Guides
///     * [Guide to Ingress and Egress Rules](https://cloud.google.com/vpc-service-controls/docs/ingress-egress-rules)
///     * [Service Perimeter Quickstart](https://cloud.google.com/vpc-service-controls/docs/quickstart)
///
/// > **Warning:** If you are using User ADCs (Application Default Credentials) with this resource,
/// you must specify a `billing_project` and set `user_project_override` to true
/// in the provider configuration. Otherwise the ACM API will return a 403 error.
/// Your account must have the `serviceusage.services.use` permission on the
/// `billing_project` you defined.
///
/// ## Example Usage
///
/// ### Access Context Manager Service Perimeter Basic
///
///
/// ```yaml
/// resources:
///   service-perimeter:
///     type: gcp:accesscontextmanager:ServicePerimeter
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       name: accessPolicies/${["access-policy"].name}/servicePerimeters/restrict_storage
///       title: restrict_storage
///       status:
///         restrictedServices:
///           - storage.googleapis.com
///   access-level:
///     type: gcp:accesscontextmanager:AccessLevel
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       name: accessPolicies/${["access-policy"].name}/accessLevels/chromeos_no_lock
///       title: chromeos_no_lock
///       basic:
///         conditions:
///           - devicePolicy:
///               requireScreenLock: false
///               osConstraints:
///                 - osType: DESKTOP_CHROME_OS
///             regions:
///               - CH
///               - IT
///               - US
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/123456789
///       title: my policy
/// ```
/// ### Access Context Manager Service Perimeter Secure Data Exchange
///
///
/// ```yaml
/// resources:
///   secure-data-exchange:
///     type: gcp:accesscontextmanager:ServicePerimeters
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       servicePerimeters:
///         - name: accessPolicies/${["access-policy"].name}/servicePerimeters/
///           title: ""
///           status:
///             restrictedServices:
///               - storage.googleapis.com
///         - name: accessPolicies/${["access-policy"].name}/servicePerimeters/
///           title: ""
///           status:
///             restrictedServices:
///               - bigtable.googleapis.com
///             vpcAccessibleServices:
///               enableRestriction: true
///               allowedServices:
///                 - bigquery.googleapis.com
///   access-level:
///     type: gcp:accesscontextmanager:AccessLevel
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       name: accessPolicies/${["access-policy"].name}/accessLevels/secure_data_exchange
///       title: secure_data_exchange
///       basic:
///         conditions:
///           - devicePolicy:
///               requireScreenLock: false
///               osConstraints:
///                 - osType: DESKTOP_CHROME_OS
///             regions:
///               - CH
///               - IT
///               - US
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/123456789
///       title: my policy
///   test-access:
///     type: gcp:accesscontextmanager:ServicePerimeter
///     properties:
///       parent: accessPolicies/${["test-accessGoogleAccessContextManagerAccessPolicy"].name}
///       name: accessPolicies/${["test-accessGoogleAccessContextManagerAccessPolicy"].name}/servicePerimeters/%s
///       title: '%s'
///       perimeterType: PERIMETER_TYPE_REGULAR
///       status:
///         restrictedServices:
///           - bigquery.googleapis.com
///           - storage.googleapis.com
///         accessLevels:
///           - ${["access-level"].name}
///         vpcAccessibleServices:
///           enableRestriction: true
///           allowedServices:
///             - bigquery.googleapis.com
///             - storage.googleapis.com
///         ingressPolicies:
///           - ingressFrom:
///               sources:
///                 - accessLevel: ${["test-accessGoogleAccessContextManagerAccessLevel"].name}
///               identityType: ANY_IDENTITY
///             ingressTo:
///               resources:
///                 - '*'
///               operations:
///                 - serviceName: bigquery.googleapis.com
///                   methodSelectors:
///                     - method: BigQueryStorage.ReadRows
///                     - method: TableService.ListTables
///                     - permission: bigquery.jobs.get
///                 - serviceName: storage.googleapis.com
///                   methodSelectors:
///                     - method: google.storage.objects.create
///         egressPolicies:
///           - egressFrom:
///               identityType: ANY_USER_ACCOUNT
/// ```
/// ### Access Context Manager Service Perimeter Dry-Run
///
///
/// ```yaml
/// resources:
///   service-perimeter:
///     type: gcp:accesscontextmanager:ServicePerimeter
///     properties:
///       parent: accessPolicies/${["access-policy"].name}
///       name: accessPolicies/${["access-policy"].name}/servicePerimeters/restrict_bigquery_dryrun_storage
///       title: restrict_bigquery_dryrun_storage
///       status:
///         restrictedServices:
///           - bigquery.googleapis.com
///       spec:
///         restrictedServices:
///           - storage.googleapis.com
///       useExplicitDryRunSpec: true
///   access-policy:
///     type: gcp:accesscontextmanager:AccessPolicy
///     properties:
///       parent: organizations/123456789
///       title: my policy
/// ```
///
/// ## Import
///
/// ServicePerimeter can be imported using any of these accepted formats:
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, ServicePerimeter can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:accesscontextmanager/servicePerimeter:ServicePerimeter default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_perimeter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServicePerimeterArgs {
        /// Description of the ServicePerimeter and its use. Does not affect
        /// behavior.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Resource name for the ServicePerimeter. The short_name component must
        /// begin with a letter and only include alphanumeric and '_'.
        /// Format: accessPolicies/{policy_id}/servicePerimeters/{short_name}
        ///
        ///
        /// - - -
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The AccessPolicy this ServicePerimeter lives in.
        /// Format: accessPolicies/{policy_id}
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Specifies the type of the Perimeter. There are two types: regular and
        /// bridge. Regular Service Perimeter contains resources, access levels,
        /// and restricted services. Every resource can be in at most
        /// ONE regular Service Perimeter.
        /// In addition to being in a regular service perimeter, a resource can also
        /// be in zero or more perimeter bridges. A perimeter bridge only contains
        /// resources. Cross project operations are permitted if all effected
        /// resources share some perimeter (whether bridge or regular). Perimeter
        /// Bridge does not contain access levels or services: those are governed
        /// entirely by the regular perimeter that resource is in.
        /// Perimeter Bridges are typically useful when building more complex
        /// topologies with many independent perimeters that need to share some data
        /// with a common perimeter, but should not be able to share data among
        /// themselves.
        /// Default value is `PERIMETER_TYPE_REGULAR`.
        /// Possible values are: `PERIMETER_TYPE_REGULAR`, `PERIMETER_TYPE_BRIDGE`.
        #[builder(into, default)]
        pub perimeter_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Proposed (or dry run) ServicePerimeter configuration.
        /// This configuration allows to specify and test ServicePerimeter configuration
        /// without enforcing actual access restrictions. Only allowed to be set when
        /// the `useExplicitDryRunSpec` flag is set.
        /// Structure is documented below.
        #[builder(into, default)]
        pub spec: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::accesscontextmanager::ServicePerimeterSpec>,
        >,
        /// ServicePerimeter configuration. Specifies sets of resources,
        /// restricted services and access levels that determine
        /// perimeter content and boundaries.
        /// Structure is documented below.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::accesscontextmanager::ServicePerimeterStatus>,
        >,
        /// Human readable title. Must be unique within the Policy.
        #[builder(into)]
        pub title: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists
        /// for all Service Perimeters, and that spec is identical to the status for those
        /// Service Perimeters. When this flag is set, it inhibits the generation of the
        /// implicit spec, thereby allowing the user to explicitly provide a
        /// configuration ("spec") to use in a dry-run version of the Service Perimeter.
        /// This allows the user to test changes to the enforced config ("status") without
        /// actually enforcing them. This testing is done through analyzing the differences
        /// between currently enforced and suggested restrictions. useExplicitDryRunSpec must
        /// bet set to True if any of the fields in the spec are set to non-default values.
        #[builder(into, default)]
        pub use_explicit_dry_run_spec: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ServicePerimeterResult {
        /// Time the AccessPolicy was created in UTC.
        pub create_time: pulumi_gestalt_rust::Output<String>,
        /// Description of the ServicePerimeter and its use. Does not affect
        /// behavior.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Resource name for the ServicePerimeter. The short_name component must
        /// begin with a letter and only include alphanumeric and '_'.
        /// Format: accessPolicies/{policy_id}/servicePerimeters/{short_name}
        ///
        ///
        /// - - -
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The AccessPolicy this ServicePerimeter lives in.
        /// Format: accessPolicies/{policy_id}
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// Specifies the type of the Perimeter. There are two types: regular and
        /// bridge. Regular Service Perimeter contains resources, access levels,
        /// and restricted services. Every resource can be in at most
        /// ONE regular Service Perimeter.
        /// In addition to being in a regular service perimeter, a resource can also
        /// be in zero or more perimeter bridges. A perimeter bridge only contains
        /// resources. Cross project operations are permitted if all effected
        /// resources share some perimeter (whether bridge or regular). Perimeter
        /// Bridge does not contain access levels or services: those are governed
        /// entirely by the regular perimeter that resource is in.
        /// Perimeter Bridges are typically useful when building more complex
        /// topologies with many independent perimeters that need to share some data
        /// with a common perimeter, but should not be able to share data among
        /// themselves.
        /// Default value is `PERIMETER_TYPE_REGULAR`.
        /// Possible values are: `PERIMETER_TYPE_REGULAR`, `PERIMETER_TYPE_BRIDGE`.
        pub perimeter_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// Proposed (or dry run) ServicePerimeter configuration.
        /// This configuration allows to specify and test ServicePerimeter configuration
        /// without enforcing actual access restrictions. Only allowed to be set when
        /// the `useExplicitDryRunSpec` flag is set.
        /// Structure is documented below.
        pub spec: pulumi_gestalt_rust::Output<
            Option<super::super::types::accesscontextmanager::ServicePerimeterSpec>,
        >,
        /// ServicePerimeter configuration. Specifies sets of resources,
        /// restricted services and access levels that determine
        /// perimeter content and boundaries.
        /// Structure is documented below.
        pub status: pulumi_gestalt_rust::Output<
            Option<super::super::types::accesscontextmanager::ServicePerimeterStatus>,
        >,
        /// Human readable title. Must be unique within the Policy.
        pub title: pulumi_gestalt_rust::Output<String>,
        /// Time the AccessPolicy was updated in UTC.
        pub update_time: pulumi_gestalt_rust::Output<String>,
        /// Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly exists
        /// for all Service Perimeters, and that spec is identical to the status for those
        /// Service Perimeters. When this flag is set, it inhibits the generation of the
        /// implicit spec, thereby allowing the user to explicitly provide a
        /// configuration ("spec") to use in a dry-run version of the Service Perimeter.
        /// This allows the user to test changes to the enforced config ("status") without
        /// actually enforcing them. This testing is done through analyzing the differences
        /// between currently enforced and suggested restrictions. useExplicitDryRunSpec must
        /// bet set to True if any of the fields in the spec are set to non-default values.
        pub use_explicit_dry_run_spec: pulumi_gestalt_rust::Output<Option<bool>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ServicePerimeterArgs,
    ) -> ServicePerimeterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let perimeter_type_binding = args.perimeter_type.get_output(context);
        let spec_binding = args.spec.get_output(context);
        let status_binding = args.status.get_output(context);
        let title_binding = args.title.get_output(context);
        let use_explicit_dry_run_spec_binding = args
            .use_explicit_dry_run_spec
            .get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:accesscontextmanager/servicePerimeter:ServicePerimeter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "perimeterType".into(),
                    value: &perimeter_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "spec".into(),
                    value: &spec_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "title".into(),
                    value: &title_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "useExplicitDryRunSpec".into(),
                    value: &use_explicit_dry_run_spec_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ServicePerimeterResult {
            create_time: o.get_field("createTime"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            parent: o.get_field("parent"),
            perimeter_type: o.get_field("perimeterType"),
            spec: o.get_field("spec"),
            status: o.get_field("status"),
            title: o.get_field("title"),
            update_time: o.get_field("updateTime"),
            use_explicit_dry_run_spec: o.get_field("useExplicitDryRunSpec"),
        }
    }
}

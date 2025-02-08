/// A Cloud Run service has a unique endpoint and autoscales containers.
///
///
/// To get more information about Service, see:
///
/// * [API documentation](https://cloud.google.com/run/docs/reference/rest/v1/namespaces.services)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/run/docs/)
///
/// > **Warning:** We recommend using the `gcp.cloudrunv2.Service` resource which offers a better
/// developer experience and broader support of Cloud Run features.
///
/// ## Example Usage
///
/// ### Cloud Run Service Pubsub
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrun:Service
///     properties:
///       name: cloud_run_service_name
///       location: us-central1
///       template:
///         spec:
///           containers:
///             - image: gcr.io/cloudrun/hello
///       traffics:
///         - percent: 100
///           latestRevision: true
///   sa:
///     type: gcp:serviceaccount:Account
///     properties:
///       accountId: cloud-run-pubsub-invoker
///       displayName: Cloud Run Pub/Sub Invoker
///   binding:
///     type: gcp:cloudrun:IamBinding
///     properties:
///       location: ${default.location}
///       service: ${default.name}
///       role: roles/run.invoker
///       members:
///         - serviceAccount:${sa.email}
///   project:
///     type: gcp:projects:IAMBinding
///     properties:
///       role: roles/iam.serviceAccountTokenCreator
///       members:
///         - serviceAccount:${sa.email}
///   topic:
///     type: gcp:pubsub:Topic
///     properties:
///       name: pubsub_topic
///   subscription:
///     type: gcp:pubsub:Subscription
///     properties:
///       name: pubsub_subscription
///       topic: ${topic.name}
///       pushConfig:
///         pushEndpoint: ${default.statuses[0].url}
///         oidcToken:
///           serviceAccountEmail: ${sa.email}
///         attributes:
///           x-goog-version: v1
/// ```
///
/// ### Cloud Run Service Basic
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = service::create(
///         "default",
///         ServiceArgs::builder()
///             .location("us-central1")
///             .name("cloudrun-srv")
///             .template(
///                 ServiceTemplate::builder()
///                     .spec(
///                         ServiceTemplateSpec::builder()
///                             .containers(
///                                 vec![
///                                     ServiceTemplateSpecContainer::builder()
///                                     .image("us-docker.pkg.dev/cloudrun/container/hello")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .traffics(
///                 vec![
///                     ServiceTraffic::builder().latestRevision(true).percent(100)
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloud Run Service Gpu
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrun:Service
///     properties:
///       name: cloudrun-srv
///       location: us-central1
///       metadata:
///         annotations:
///           run.googleapis.com/launch-stage: BETA
///       template:
///         metadata:
///           annotations:
///             autoscaling.knative.dev/maxScale: '1'
///             run.googleapis.com/cpu-throttling: 'false'
///         spec:
///           containers:
///             - image: gcr.io/cloudrun/hello
///               resources:
///                 limits:
///                   cpu: '4'
///                   memory: 16Gi
///                   nvidia.com/gpu: '1'
///           nodeSelector:
///             run.googleapis.com/accelerator: nvidia-l4
/// ```
/// ### Cloud Run Service Sql
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrun:Service
///     properties:
///       name: cloudrun-srv
///       location: us-central1
///       template:
///         spec:
///           containers:
///             - image: us-docker.pkg.dev/cloudrun/container/hello
///         metadata:
///           annotations:
///             autoscaling.knative.dev/maxScale: '1000'
///             run.googleapis.com/cloudsql-instances: ${instance.connectionName}
///             run.googleapis.com/client-name: demo
///       autogenerateRevisionName: true
///   instance:
///     type: gcp:sql:DatabaseInstance
///     properties:
///       name: cloudrun-sql
///       region: us-east1
///       databaseVersion: MYSQL_5_7
///       settings:
///         tier: db-f1-micro
///       deletionProtection: true
/// ```
/// ### Cloud Run Service Noauth
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrun:Service
///     properties:
///       name: cloudrun-srv
///       location: us-central1
///       template:
///         spec:
///           containers:
///             - image: us-docker.pkg.dev/cloudrun/container/hello
///   noauthIamPolicy:
///     type: gcp:cloudrun:IamPolicy
///     name: noauth
///     properties:
///       location: ${default.location}
///       project: ${default.project}
///       service: ${default.name}
///       policyData: ${noauth.policyData}
/// variables:
///   noauth:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/run.invoker
///             members:
///               - allUsers
/// ```
/// ### Cloud Run Service Probes
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = service::create(
///         "default",
///         ServiceArgs::builder()
///             .location("us-central1")
///             .name("cloudrun-srv")
///             .template(
///                 ServiceTemplate::builder()
///                     .spec(
///                         ServiceTemplateSpec::builder()
///                             .containers(
///                                 vec![
///                                     ServiceTemplateSpecContainer::builder()
///                                     .image("us-docker.pkg.dev/cloudrun/container/hello")
///                                     .livenessProbe(ServiceTemplateSpecContainerLivenessProbe::builder()
///                                     .httpGet(ServiceTemplateSpecContainerLivenessProbeHttpGet::builder()
///                                     .path("/").build_struct()).build_struct())
///                                     .startupProbe(ServiceTemplateSpecContainerStartupProbe::builder()
///                                     .failureThreshold(1).initialDelaySeconds(0).periodSeconds(3)
///                                     .tcpSocket(ServiceTemplateSpecContainerStartupProbeTcpSocket::builder()
///                                     .port(8080).build_struct()).timeoutSeconds(1)
///                                     .build_struct()).build_struct(),
///                                 ],
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .traffics(
///                 vec![
///                     ServiceTraffic::builder().latestRevision(true).percent(100)
///                     .build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
/// }
/// ```
/// ### Cloud Run Service Multicontainer
///
///
/// ```yaml
/// resources:
///   default:
///     type: gcp:cloudrun:Service
///     properties:
///       name: cloudrun-srv
///       location: us-central1
///       template:
///         metadata:
///           annotations:
///             run.googleapis.com/container-dependencies:
///               fn::toJSON:
///                 hello-1:
///                   - hello-2
///         spec:
///           containers:
///             - name: hello-1
///               ports:
///                 - containerPort: 8080
///               image: us-docker.pkg.dev/cloudrun/container/hello
///               volumeMounts:
///                 - name: shared-volume
///                   mountPath: /mnt/shared
///             - name: hello-2
///               image: us-docker.pkg.dev/cloudrun/container/hello
///               envs:
///                 - name: PORT
///                   value: '8081'
///               startupProbe:
///                 httpGet:
///                   port: 8081
///               volumeMounts:
///                 - name: shared-volume
///                   mountPath: /mnt/shared
///           volumes:
///             - name: shared-volume
///               emptyDir:
///                 medium: Memory
///                 sizeLimit: 128Mi
/// ```
///
/// ## Import
///
/// Service can be imported using any of these accepted formats:
///
/// * `locations/{{location}}/namespaces/{{project}}/services/{{name}}`
///
/// * `{{location}}/{{project}}/{{name}}`
///
/// * `{{location}}/{{name}}`
///
/// When using the `pulumi import` command, Service can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:cloudrun/service:Service default locations/{{location}}/namespaces/{{project}}/services/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudrun/service:Service default {{location}}/{{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:cloudrun/service:Service default {{location}}/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation)]
pub mod service {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceArgs {
        /// If set to 'true', the revision name (template.metadata.name) will be omitted and autogenerated by Cloud Run. This cannot
        /// be set to 'true' while 'template.metadata.name' is also set. (For legacy support, if 'template.metadata.name' is unset
        /// in state while this field is set to false, the revision name will still autogenerate.)
        #[builder(into, default)]
        pub autogenerate_revision_name: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The location of the cloud run instance. eg us-central1
        #[builder(into)]
        pub location: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Metadata associated with this Service, including name, namespace, labels, and annotations.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudrun::ServiceMetadata>,
        >,
        /// Name must be unique within a Google Cloud project and region.
        /// Is required when creating resources. Name is primarily intended
        /// for creation idempotence and configuration definition. Cannot be updated.
        /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// template holds the latest specification for the Revision to be stamped out. The template references the container image,
        /// and may also include labels and annotations that should be attached to the Revision. To correlate a Revision, and/or to
        /// force a Revision to be created when the spec doesn't otherwise change, a nonce label may be provided in the template
        /// metadata. For more details, see:
        /// https://github.com/knative/serving/blob/main/docs/client-conventions.md#associate-modifications-with-revisions Cloud Run
        /// does not currently support referencing a build that is responsible for materializing the container image from source.
        #[builder(into, default)]
        pub template: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudrun::ServiceTemplate>,
        >,
        /// (Output)
        /// Traffic specifies how to distribute traffic over a collection of Knative Revisions
        /// and Configurations
        /// Structure is documented below.
        #[builder(into, default)]
        pub traffics: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cloudrun::ServiceTraffic>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ServiceResult {
        /// If set to 'true', the revision name (template.metadata.name) will be omitted and autogenerated by Cloud Run. This cannot
        /// be set to 'true' while 'template.metadata.name' is also set. (For legacy support, if 'template.metadata.name' is unset
        /// in state while this field is set to false, the revision name will still autogenerate.)
        pub autogenerate_revision_name: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The location of the cloud run instance. eg us-central1
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Metadata associated with this Service, including name, namespace, labels, and annotations.
        pub metadata: pulumi_gestalt_rust::Output<
            super::super::types::cloudrun::ServiceMetadata,
        >,
        /// Name must be unique within a Google Cloud project and region.
        /// Is required when creating resources. Name is primarily intended
        /// for creation idempotence and configuration definition. Cannot be updated.
        /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
        pub name: pulumi_gestalt_rust::Output<String>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// (Output)
        /// Status of the condition, one of True, False, Unknown.
        pub statuses: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudrun::ServiceStatus>,
        >,
        /// template holds the latest specification for the Revision to be stamped out. The template references the container image,
        /// and may also include labels and annotations that should be attached to the Revision. To correlate a Revision, and/or to
        /// force a Revision to be created when the spec doesn't otherwise change, a nonce label may be provided in the template
        /// metadata. For more details, see:
        /// https://github.com/knative/serving/blob/main/docs/client-conventions.md#associate-modifications-with-revisions Cloud Run
        /// does not currently support referencing a build that is responsible for materializing the container image from source.
        pub template: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudrun::ServiceTemplate>,
        >,
        /// (Output)
        /// Traffic specifies how to distribute traffic over a collection of Knative Revisions
        /// and Configurations
        /// Structure is documented below.
        pub traffics: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cloudrun::ServiceTraffic>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: ServiceArgs,
    ) -> ServiceResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let autogenerate_revision_name_binding = args
            .autogenerate_revision_name
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let metadata_binding = args.metadata.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let project_binding = args.project.get_output(context).get_inner();
        let template_binding = args.template.get_output(context).get_inner();
        let traffics_binding = args.traffics.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "gcp:cloudrun/service:Service".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "autogenerateRevisionName".into(),
                    value: &autogenerate_revision_name_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "project".into(),
                    value: &project_binding,
                },
                register_interface::ObjectField {
                    name: "template".into(),
                    value: &template_binding,
                },
                register_interface::ObjectField {
                    name: "traffics".into(),
                    value: &traffics_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        ServiceResult {
            autogenerate_revision_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("autogenerateRevisionName"),
            ),
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            metadata: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("metadata"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            project: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("project"),
            ),
            statuses: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("statuses"),
            ),
            template: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("template"),
            ),
            traffics: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("traffics"),
            ),
        }
    }
}

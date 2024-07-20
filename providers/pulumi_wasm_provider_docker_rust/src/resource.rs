pub mod container {

    pub struct ContainerArgs {
        pub attach: pulumi_wasm_rust::Output<Option<bool>>,
        pub capabilities: pulumi_wasm_rust::Output<Option<crate::types::ContainerCapabilities>>,
        pub cgroupns_mode: pulumi_wasm_rust::Output<Option<String>>,
        pub command: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub container_read_refresh_timeout_milliseconds: pulumi_wasm_rust::Output<Option<i32>>,
        pub cpu_set: pulumi_wasm_rust::Output<Option<String>>,
        pub cpu_shares: pulumi_wasm_rust::Output<Option<i32>>,
        pub destroy_grace_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        pub devices: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerDevice>>>,
        pub dns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub dns_opts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub dns_searches: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub domainname: pulumi_wasm_rust::Output<Option<String>>,
        pub entrypoints: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub envs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub gpus: pulumi_wasm_rust::Output<Option<String>>,
        pub group_adds: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub healthcheck: pulumi_wasm_rust::Output<Option<crate::types::ContainerHealthcheck>>,
        pub hostname: pulumi_wasm_rust::Output<Option<String>>,
        pub hosts: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerHost>>>,
        pub image: pulumi_wasm_rust::Output<String>,
        pub init: pulumi_wasm_rust::Output<Option<bool>>,
        pub ipc_mode: pulumi_wasm_rust::Output<Option<String>>,
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerLabel>>>,
        pub log_driver: pulumi_wasm_rust::Output<Option<String>>,
        pub log_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub logs: pulumi_wasm_rust::Output<Option<bool>>,
        pub max_retry_count: pulumi_wasm_rust::Output<Option<i32>>,
        pub memory: pulumi_wasm_rust::Output<Option<i32>>,
        pub memory_swap: pulumi_wasm_rust::Output<Option<i32>>,
        pub mounts: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerMount>>>,
        pub must_run: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub network_mode: pulumi_wasm_rust::Output<Option<String>>,
        pub networks_advanced:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerNetworksAdvanced>>>,
        pub pid_mode: pulumi_wasm_rust::Output<Option<String>>,
        pub ports: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerPort>>>,
        pub privileged: pulumi_wasm_rust::Output<Option<bool>>,
        pub publish_all_ports: pulumi_wasm_rust::Output<Option<bool>>,
        pub read_only: pulumi_wasm_rust::Output<Option<bool>>,
        pub remove_volumes: pulumi_wasm_rust::Output<Option<bool>>,
        pub restart: pulumi_wasm_rust::Output<Option<String>>,
        pub rm: pulumi_wasm_rust::Output<Option<bool>>,
        pub runtime: pulumi_wasm_rust::Output<Option<String>>,
        pub security_opts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub shm_size: pulumi_wasm_rust::Output<Option<i32>>,
        pub start: pulumi_wasm_rust::Output<Option<bool>>,
        pub stdin_open: pulumi_wasm_rust::Output<Option<bool>>,
        pub stop_signal: pulumi_wasm_rust::Output<Option<String>>,
        pub stop_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        pub storage_opts:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub sysctls: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub tmpfs: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub tty: pulumi_wasm_rust::Output<Option<bool>>,
        pub ulimits: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerUlimit>>>,
        pub uploads: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerUpload>>>,
        pub user: pulumi_wasm_rust::Output<Option<String>>,
        pub userns_mode: pulumi_wasm_rust::Output<Option<String>>,
        pub volumes: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerVolume>>>,
        pub wait: pulumi_wasm_rust::Output<Option<bool>>,
        pub wait_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        pub working_dir: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct ContainerResult {
        pub attach: pulumi_wasm_rust::Output<Option<bool>>,
        pub bridge: pulumi_wasm_rust::Output<String>,
        pub capabilities: pulumi_wasm_rust::Output<Option<crate::types::ContainerCapabilities>>,
        pub cgroupns_mode: pulumi_wasm_rust::Output<Option<String>>,
        pub command: pulumi_wasm_rust::Output<Vec<String>>,
        pub container_logs: pulumi_wasm_rust::Output<Option<String>>,
        pub container_read_refresh_timeout_milliseconds: pulumi_wasm_rust::Output<Option<i32>>,
        pub cpu_set: pulumi_wasm_rust::Output<Option<String>>,
        pub cpu_shares: pulumi_wasm_rust::Output<Option<i32>>,
        pub destroy_grace_seconds: pulumi_wasm_rust::Output<Option<i32>>,
        pub devices: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerDevice>>>,
        pub dns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub dns_opts: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub dns_searches: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub domainname: pulumi_wasm_rust::Output<Option<String>>,
        pub entrypoints: pulumi_wasm_rust::Output<Vec<String>>,
        pub envs: pulumi_wasm_rust::Output<Vec<String>>,
        pub exit_code: pulumi_wasm_rust::Output<i32>,
        pub gpus: pulumi_wasm_rust::Output<Option<String>>,
        pub group_adds: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub healthcheck: pulumi_wasm_rust::Output<Option<crate::types::ContainerHealthcheck>>,
        pub hostname: pulumi_wasm_rust::Output<String>,
        pub hosts: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerHost>>>,
        pub image: pulumi_wasm_rust::Output<String>,
        pub init: pulumi_wasm_rust::Output<bool>,
        pub ipc_mode: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<Vec<crate::types::ContainerLabel>>,
        pub log_driver: pulumi_wasm_rust::Output<String>,
        pub log_opts: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub logs: pulumi_wasm_rust::Output<Option<bool>>,
        pub max_retry_count: pulumi_wasm_rust::Output<Option<i32>>,
        pub memory: pulumi_wasm_rust::Output<Option<i32>>,
        pub memory_swap: pulumi_wasm_rust::Output<Option<i32>>,
        pub mounts: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerMount>>>,
        pub must_run: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub network_datas: pulumi_wasm_rust::Output<Vec<crate::types::ContainerNetworkData>>,
        pub network_mode: pulumi_wasm_rust::Output<Option<String>>,
        pub networks_advanced:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerNetworksAdvanced>>>,
        pub pid_mode: pulumi_wasm_rust::Output<Option<String>>,
        pub ports: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerPort>>>,
        pub privileged: pulumi_wasm_rust::Output<Option<bool>>,
        pub publish_all_ports: pulumi_wasm_rust::Output<Option<bool>>,
        pub read_only: pulumi_wasm_rust::Output<Option<bool>>,
        pub remove_volumes: pulumi_wasm_rust::Output<Option<bool>>,
        pub restart: pulumi_wasm_rust::Output<Option<String>>,
        pub rm: pulumi_wasm_rust::Output<Option<bool>>,
        pub runtime: pulumi_wasm_rust::Output<String>,
        pub security_opts: pulumi_wasm_rust::Output<Vec<String>>,
        pub shm_size: pulumi_wasm_rust::Output<i32>,
        pub start: pulumi_wasm_rust::Output<Option<bool>>,
        pub stdin_open: pulumi_wasm_rust::Output<Option<bool>>,
        pub stop_signal: pulumi_wasm_rust::Output<String>,
        pub stop_timeout: pulumi_wasm_rust::Output<i32>,
        pub storage_opts:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub sysctls: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub tmpfs: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub tty: pulumi_wasm_rust::Output<Option<bool>>,
        pub ulimits: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerUlimit>>>,
        pub uploads: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerUpload>>>,
        pub user: pulumi_wasm_rust::Output<Option<String>>,
        pub userns_mode: pulumi_wasm_rust::Output<Option<String>>,
        pub volumes: pulumi_wasm_rust::Output<Option<Vec<crate::types::ContainerVolume>>>,
        pub wait: pulumi_wasm_rust::Output<Option<bool>>,
        pub wait_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        pub working_dir: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub fn container(name: &str, args: ContainerArgs) -> ContainerResult {
        let result = crate::bindings::pulumi::docker::container::invoke(
            name,
            &crate::bindings::pulumi::docker::container::Args {
                attach: &crate::clone::<Option<bool>>(args.attach),
                capabilities: &crate::clone::<Option<crate::types::ContainerCapabilities>>(
                    args.capabilities,
                ),
                cgroupns_mode: &crate::clone::<Option<String>>(args.cgroupns_mode),
                command: &crate::clone::<Option<Vec<String>>>(args.command),
                container_read_refresh_timeout_milliseconds: &crate::clone::<Option<i32>>(
                    args.container_read_refresh_timeout_milliseconds,
                ),
                cpu_set: &crate::clone::<Option<String>>(args.cpu_set),
                cpu_shares: &crate::clone::<Option<i32>>(args.cpu_shares),
                destroy_grace_seconds: &crate::clone::<Option<i32>>(args.destroy_grace_seconds),
                devices: &crate::clone::<Option<Vec<crate::types::ContainerDevice>>>(args.devices),
                dns: &crate::clone::<Option<Vec<String>>>(args.dns),
                dns_opts: &crate::clone::<Option<Vec<String>>>(args.dns_opts),
                dns_searches: &crate::clone::<Option<Vec<String>>>(args.dns_searches),
                domainname: &crate::clone::<Option<String>>(args.domainname),
                entrypoints: &crate::clone::<Option<Vec<String>>>(args.entrypoints),
                envs: &crate::clone::<Option<Vec<String>>>(args.envs),
                gpus: &crate::clone::<Option<String>>(args.gpus),
                group_adds: &crate::clone::<Option<Vec<String>>>(args.group_adds),
                healthcheck: &crate::clone::<Option<crate::types::ContainerHealthcheck>>(
                    args.healthcheck,
                ),
                hostname: &crate::clone::<Option<String>>(args.hostname),
                hosts: &crate::clone::<Option<Vec<crate::types::ContainerHost>>>(args.hosts),
                image: &crate::clone::<String>(args.image),
                init: &crate::clone::<Option<bool>>(args.init),
                ipc_mode: &crate::clone::<Option<String>>(args.ipc_mode),
                labels: &crate::clone::<Option<Vec<crate::types::ContainerLabel>>>(args.labels),
                log_driver: &crate::clone::<Option<String>>(args.log_driver),
                log_opts: &crate::clone::<Option<std::collections::HashMap<String, String>>>(
                    args.log_opts,
                ),
                logs: &crate::clone::<Option<bool>>(args.logs),
                max_retry_count: &crate::clone::<Option<i32>>(args.max_retry_count),
                memory: &crate::clone::<Option<i32>>(args.memory),
                memory_swap: &crate::clone::<Option<i32>>(args.memory_swap),
                mounts: &crate::clone::<Option<Vec<crate::types::ContainerMount>>>(args.mounts),
                must_run: &crate::clone::<Option<bool>>(args.must_run),
                name: &crate::clone::<Option<String>>(args.name),
                network_mode: &crate::clone::<Option<String>>(args.network_mode),
                networks_advanced: &crate::clone::<
                    Option<Vec<crate::types::ContainerNetworksAdvanced>>,
                >(args.networks_advanced),
                pid_mode: &crate::clone::<Option<String>>(args.pid_mode),
                ports: &crate::clone::<Option<Vec<crate::types::ContainerPort>>>(args.ports),
                privileged: &crate::clone::<Option<bool>>(args.privileged),
                publish_all_ports: &crate::clone::<Option<bool>>(args.publish_all_ports),
                read_only: &crate::clone::<Option<bool>>(args.read_only),
                remove_volumes: &crate::clone::<Option<bool>>(args.remove_volumes),
                restart: &crate::clone::<Option<String>>(args.restart),
                rm: &crate::clone::<Option<bool>>(args.rm),
                runtime: &crate::clone::<Option<String>>(args.runtime),
                security_opts: &crate::clone::<Option<Vec<String>>>(args.security_opts),
                shm_size: &crate::clone::<Option<i32>>(args.shm_size),
                start: &crate::clone::<Option<bool>>(args.start),
                stdin_open: &crate::clone::<Option<bool>>(args.stdin_open),
                stop_signal: &crate::clone::<Option<String>>(args.stop_signal),
                stop_timeout: &crate::clone::<Option<i32>>(args.stop_timeout),
                storage_opts: &crate::clone::<Option<std::collections::HashMap<String, String>>>(
                    args.storage_opts,
                ),
                sysctls: &crate::clone::<Option<std::collections::HashMap<String, String>>>(
                    args.sysctls,
                ),
                tmpfs: &crate::clone::<Option<std::collections::HashMap<String, String>>>(
                    args.tmpfs,
                ),
                tty: &crate::clone::<Option<bool>>(args.tty),
                ulimits: &crate::clone::<Option<Vec<crate::types::ContainerUlimit>>>(args.ulimits),
                uploads: &crate::clone::<Option<Vec<crate::types::ContainerUpload>>>(args.uploads),
                user: &crate::clone::<Option<String>>(args.user),
                userns_mode: &crate::clone::<Option<String>>(args.userns_mode),
                volumes: &crate::clone::<Option<Vec<crate::types::ContainerVolume>>>(args.volumes),
                wait: &crate::clone::<Option<bool>>(args.wait),
                wait_timeout: &crate::clone::<Option<i32>>(args.wait_timeout),
                working_dir: &crate::clone::<Option<String>>(args.working_dir),
            },
        );

        ContainerResult {
            attach: crate::random_to_domain_mapper::<Option<bool>>(result.attach),
            bridge: crate::random_to_domain_mapper::<String>(result.bridge),
            capabilities: crate::random_to_domain_mapper::<
                Option<crate::types::ContainerCapabilities>,
            >(result.capabilities),
            cgroupns_mode: crate::random_to_domain_mapper::<Option<String>>(result.cgroupns_mode),
            command: crate::random_to_domain_mapper::<Vec<String>>(result.command),
            container_logs: crate::random_to_domain_mapper::<Option<String>>(result.container_logs),
            container_read_refresh_timeout_milliseconds: crate::random_to_domain_mapper::<
                Option<i32>,
            >(
                result.container_read_refresh_timeout_milliseconds,
            ),
            cpu_set: crate::random_to_domain_mapper::<Option<String>>(result.cpu_set),
            cpu_shares: crate::random_to_domain_mapper::<Option<i32>>(result.cpu_shares),
            destroy_grace_seconds: crate::random_to_domain_mapper::<Option<i32>>(
                result.destroy_grace_seconds,
            ),
            devices: crate::random_to_domain_mapper::<Option<Vec<crate::types::ContainerDevice>>>(
                result.devices,
            ),
            dns: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.dns),
            dns_opts: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.dns_opts),
            dns_searches: crate::random_to_domain_mapper::<Option<Vec<String>>>(
                result.dns_searches,
            ),
            domainname: crate::random_to_domain_mapper::<Option<String>>(result.domainname),
            entrypoints: crate::random_to_domain_mapper::<Vec<String>>(result.entrypoints),
            envs: crate::random_to_domain_mapper::<Vec<String>>(result.envs),
            exit_code: crate::random_to_domain_mapper::<i32>(result.exit_code),
            gpus: crate::random_to_domain_mapper::<Option<String>>(result.gpus),
            group_adds: crate::random_to_domain_mapper::<Option<Vec<String>>>(result.group_adds),
            healthcheck: crate::random_to_domain_mapper::<Option<crate::types::ContainerHealthcheck>>(
                result.healthcheck,
            ),
            hostname: crate::random_to_domain_mapper::<String>(result.hostname),
            hosts: crate::random_to_domain_mapper::<Option<Vec<crate::types::ContainerHost>>>(
                result.hosts,
            ),
            image: crate::random_to_domain_mapper::<String>(result.image),
            init: crate::random_to_domain_mapper::<bool>(result.init),
            ipc_mode: crate::random_to_domain_mapper::<String>(result.ipc_mode),
            labels: crate::random_to_domain_mapper::<Vec<crate::types::ContainerLabel>>(
                result.labels,
            ),
            log_driver: crate::random_to_domain_mapper::<String>(result.log_driver),
            log_opts: crate::random_to_domain_mapper::<
                Option<std::collections::HashMap<String, String>>,
            >(result.log_opts),
            logs: crate::random_to_domain_mapper::<Option<bool>>(result.logs),
            max_retry_count: crate::random_to_domain_mapper::<Option<i32>>(result.max_retry_count),
            memory: crate::random_to_domain_mapper::<Option<i32>>(result.memory),
            memory_swap: crate::random_to_domain_mapper::<Option<i32>>(result.memory_swap),
            mounts: crate::random_to_domain_mapper::<Option<Vec<crate::types::ContainerMount>>>(
                result.mounts,
            ),
            must_run: crate::random_to_domain_mapper::<Option<bool>>(result.must_run),
            name: crate::random_to_domain_mapper::<String>(result.name),
            network_datas: crate::random_to_domain_mapper::<Vec<crate::types::ContainerNetworkData>>(
                result.network_datas,
            ),
            network_mode: crate::random_to_domain_mapper::<Option<String>>(result.network_mode),
            networks_advanced: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::ContainerNetworksAdvanced>>,
            >(result.networks_advanced),
            pid_mode: crate::random_to_domain_mapper::<Option<String>>(result.pid_mode),
            ports: crate::random_to_domain_mapper::<Option<Vec<crate::types::ContainerPort>>>(
                result.ports,
            ),
            privileged: crate::random_to_domain_mapper::<Option<bool>>(result.privileged),
            publish_all_ports: crate::random_to_domain_mapper::<Option<bool>>(
                result.publish_all_ports,
            ),
            read_only: crate::random_to_domain_mapper::<Option<bool>>(result.read_only),
            remove_volumes: crate::random_to_domain_mapper::<Option<bool>>(result.remove_volumes),
            restart: crate::random_to_domain_mapper::<Option<String>>(result.restart),
            rm: crate::random_to_domain_mapper::<Option<bool>>(result.rm),
            runtime: crate::random_to_domain_mapper::<String>(result.runtime),
            security_opts: crate::random_to_domain_mapper::<Vec<String>>(result.security_opts),
            shm_size: crate::random_to_domain_mapper::<i32>(result.shm_size),
            start: crate::random_to_domain_mapper::<Option<bool>>(result.start),
            stdin_open: crate::random_to_domain_mapper::<Option<bool>>(result.stdin_open),
            stop_signal: crate::random_to_domain_mapper::<String>(result.stop_signal),
            stop_timeout: crate::random_to_domain_mapper::<i32>(result.stop_timeout),
            storage_opts: crate::random_to_domain_mapper::<
                Option<std::collections::HashMap<String, String>>,
            >(result.storage_opts),
            sysctls: crate::random_to_domain_mapper::<
                Option<std::collections::HashMap<String, String>>,
            >(result.sysctls),
            tmpfs: crate::random_to_domain_mapper::<
                Option<std::collections::HashMap<String, String>>,
            >(result.tmpfs),
            tty: crate::random_to_domain_mapper::<Option<bool>>(result.tty),
            ulimits: crate::random_to_domain_mapper::<Option<Vec<crate::types::ContainerUlimit>>>(
                result.ulimits,
            ),
            uploads: crate::random_to_domain_mapper::<Option<Vec<crate::types::ContainerUpload>>>(
                result.uploads,
            ),
            user: crate::random_to_domain_mapper::<Option<String>>(result.user),
            userns_mode: crate::random_to_domain_mapper::<Option<String>>(result.userns_mode),
            volumes: crate::random_to_domain_mapper::<Option<Vec<crate::types::ContainerVolume>>>(
                result.volumes,
            ),
            wait: crate::random_to_domain_mapper::<Option<bool>>(result.wait),
            wait_timeout: crate::random_to_domain_mapper::<Option<i32>>(result.wait_timeout),
            working_dir: crate::random_to_domain_mapper::<Option<String>>(result.working_dir),
        }
    }
}

pub mod image {

    pub struct ImageArgs {
        pub build: pulumi_wasm_rust::Output<Option<crate::types::DockerBuild>>,
        pub build_on_preview: pulumi_wasm_rust::Output<Option<bool>>,
        pub image_name: pulumi_wasm_rust::Output<String>,
        pub registry: pulumi_wasm_rust::Output<Option<crate::types::Registry>>,
        pub skip_push: pulumi_wasm_rust::Output<Option<bool>>,
    }

    pub struct ImageResult {
        pub base_image_name: pulumi_wasm_rust::Output<String>,
        pub context: pulumi_wasm_rust::Output<String>,
        pub dockerfile: pulumi_wasm_rust::Output<String>,
        pub image_name: pulumi_wasm_rust::Output<String>,
        pub platform: pulumi_wasm_rust::Output<Option<String>>,
        pub registry_server: pulumi_wasm_rust::Output<String>,
        pub repo_digest: pulumi_wasm_rust::Output<String>,
    }

    pub fn image(name: &str, args: ImageArgs) -> ImageResult {
        let result = crate::bindings::pulumi::docker::image::invoke(
            name,
            &crate::bindings::pulumi::docker::image::Args {
                build: &crate::clone::<Option<crate::types::DockerBuild>>(args.build),
                build_on_preview: &crate::clone::<Option<bool>>(args.build_on_preview),
                image_name: &crate::clone::<String>(args.image_name),
                registry: &crate::clone::<Option<crate::types::Registry>>(args.registry),
                skip_push: &crate::clone::<Option<bool>>(args.skip_push),
            },
        );

        ImageResult {
            base_image_name: crate::random_to_domain_mapper::<String>(result.base_image_name),
            context: crate::random_to_domain_mapper::<String>(result.context),
            dockerfile: crate::random_to_domain_mapper::<String>(result.dockerfile),
            image_name: crate::random_to_domain_mapper::<String>(result.image_name),
            platform: crate::random_to_domain_mapper::<Option<String>>(result.platform),
            registry_server: crate::random_to_domain_mapper::<String>(result.registry_server),
            repo_digest: crate::random_to_domain_mapper::<String>(result.repo_digest),
        }
    }
}

pub mod network {

    pub struct NetworkArgs {
        pub attachable: pulumi_wasm_rust::Output<Option<bool>>,
        pub check_duplicate: pulumi_wasm_rust::Output<Option<bool>>,
        pub driver: pulumi_wasm_rust::Output<Option<String>>,
        pub ingress: pulumi_wasm_rust::Output<Option<bool>>,
        pub internal: pulumi_wasm_rust::Output<Option<bool>>,
        pub ipam_configs: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkIpamConfig>>>,
        pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
        pub ipam_options:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkLabel>>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub options: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    pub struct NetworkResult {
        pub attachable: pulumi_wasm_rust::Output<Option<bool>>,
        pub check_duplicate: pulumi_wasm_rust::Output<Option<bool>>,
        pub driver: pulumi_wasm_rust::Output<String>,
        pub ingress: pulumi_wasm_rust::Output<Option<bool>>,
        pub internal: pulumi_wasm_rust::Output<bool>,
        pub ipam_configs: pulumi_wasm_rust::Output<Vec<crate::types::NetworkIpamConfig>>,
        pub ipam_driver: pulumi_wasm_rust::Output<Option<String>>,
        pub ipam_options:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub ipv6: pulumi_wasm_rust::Output<Option<bool>>,
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::NetworkLabel>>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub options: pulumi_wasm_rust::Output<std::collections::HashMap<String, String>>,
        pub scope: pulumi_wasm_rust::Output<String>,
    }

    pub fn network(name: &str, args: NetworkArgs) -> NetworkResult {
        let result = crate::bindings::pulumi::docker::network::invoke(
            name,
            &crate::bindings::pulumi::docker::network::Args {
                attachable: &crate::clone::<Option<bool>>(args.attachable),
                check_duplicate: &crate::clone::<Option<bool>>(args.check_duplicate),
                driver: &crate::clone::<Option<String>>(args.driver),
                ingress: &crate::clone::<Option<bool>>(args.ingress),
                internal: &crate::clone::<Option<bool>>(args.internal),
                ipam_configs: &crate::clone::<Option<Vec<crate::types::NetworkIpamConfig>>>(
                    args.ipam_configs,
                ),
                ipam_driver: &crate::clone::<Option<String>>(args.ipam_driver),
                ipam_options: &crate::clone::<Option<std::collections::HashMap<String, String>>>(
                    args.ipam_options,
                ),
                ipv6: &crate::clone::<Option<bool>>(args.ipv6),
                labels: &crate::clone::<Option<Vec<crate::types::NetworkLabel>>>(args.labels),
                name: &crate::clone::<Option<String>>(args.name),
                options: &crate::clone::<Option<std::collections::HashMap<String, String>>>(
                    args.options,
                ),
            },
        );

        NetworkResult {
            attachable: crate::random_to_domain_mapper::<Option<bool>>(result.attachable),
            check_duplicate: crate::random_to_domain_mapper::<Option<bool>>(result.check_duplicate),
            driver: crate::random_to_domain_mapper::<String>(result.driver),
            ingress: crate::random_to_domain_mapper::<Option<bool>>(result.ingress),
            internal: crate::random_to_domain_mapper::<bool>(result.internal),
            ipam_configs: crate::random_to_domain_mapper::<Vec<crate::types::NetworkIpamConfig>>(
                result.ipam_configs,
            ),
            ipam_driver: crate::random_to_domain_mapper::<Option<String>>(result.ipam_driver),
            ipam_options: crate::random_to_domain_mapper::<
                Option<std::collections::HashMap<String, String>>,
            >(result.ipam_options),
            ipv6: crate::random_to_domain_mapper::<Option<bool>>(result.ipv6),
            labels: crate::random_to_domain_mapper::<Option<Vec<crate::types::NetworkLabel>>>(
                result.labels,
            ),
            name: crate::random_to_domain_mapper::<String>(result.name),
            options: crate::random_to_domain_mapper::<std::collections::HashMap<String, String>>(
                result.options,
            ),
            scope: crate::random_to_domain_mapper::<String>(result.scope),
        }
    }
}

pub mod plugin {

    pub struct PluginArgs {
        pub alias: pulumi_wasm_rust::Output<Option<String>>,
        pub enable_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub envs: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        pub force_disable: pulumi_wasm_rust::Output<Option<bool>>,
        pub grant_all_permissions: pulumi_wasm_rust::Output<Option<bool>>,
        pub grant_permissions:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::PluginGrantPermission>>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct PluginResult {
        pub alias: pulumi_wasm_rust::Output<String>,
        pub enable_timeout: pulumi_wasm_rust::Output<Option<i32>>,
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        pub envs: pulumi_wasm_rust::Output<Vec<String>>,
        pub force_destroy: pulumi_wasm_rust::Output<Option<bool>>,
        pub force_disable: pulumi_wasm_rust::Output<Option<bool>>,
        pub grant_all_permissions: pulumi_wasm_rust::Output<Option<bool>>,
        pub grant_permissions:
            pulumi_wasm_rust::Output<Option<Vec<crate::types::PluginGrantPermission>>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub plugin_reference: pulumi_wasm_rust::Output<String>,
    }

    pub fn plugin(name: &str, args: PluginArgs) -> PluginResult {
        let result = crate::bindings::pulumi::docker::plugin::invoke(
            name,
            &crate::bindings::pulumi::docker::plugin::Args {
                alias: &crate::clone::<Option<String>>(args.alias),
                enable_timeout: &crate::clone::<Option<i32>>(args.enable_timeout),
                enabled: &crate::clone::<Option<bool>>(args.enabled),
                envs: &crate::clone::<Option<Vec<String>>>(args.envs),
                force_destroy: &crate::clone::<Option<bool>>(args.force_destroy),
                force_disable: &crate::clone::<Option<bool>>(args.force_disable),
                grant_all_permissions: &crate::clone::<Option<bool>>(args.grant_all_permissions),
                grant_permissions: &crate::clone::<Option<Vec<crate::types::PluginGrantPermission>>>(
                    args.grant_permissions,
                ),
                name: &crate::clone::<Option<String>>(args.name),
            },
        );

        PluginResult {
            alias: crate::random_to_domain_mapper::<String>(result.alias),
            enable_timeout: crate::random_to_domain_mapper::<Option<i32>>(result.enable_timeout),
            enabled: crate::random_to_domain_mapper::<Option<bool>>(result.enabled),
            envs: crate::random_to_domain_mapper::<Vec<String>>(result.envs),
            force_destroy: crate::random_to_domain_mapper::<Option<bool>>(result.force_destroy),
            force_disable: crate::random_to_domain_mapper::<Option<bool>>(result.force_disable),
            grant_all_permissions: crate::random_to_domain_mapper::<Option<bool>>(
                result.grant_all_permissions,
            ),
            grant_permissions: crate::random_to_domain_mapper::<
                Option<Vec<crate::types::PluginGrantPermission>>,
            >(result.grant_permissions),
            name: crate::random_to_domain_mapper::<String>(result.name),
            plugin_reference: crate::random_to_domain_mapper::<String>(result.plugin_reference),
        }
    }
}

pub mod registry_image {

    pub struct RegistryImageArgs {
        pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
        pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    pub struct RegistryImageResult {
        pub insecure_skip_verify: pulumi_wasm_rust::Output<Option<bool>>,
        pub keep_remotely: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub sha256_digest: pulumi_wasm_rust::Output<String>,
        pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    pub fn registry_image(name: &str, args: RegistryImageArgs) -> RegistryImageResult {
        let result = crate::bindings::pulumi::docker::registry_image::invoke(
            name,
            &crate::bindings::pulumi::docker::registry_image::Args {
                insecure_skip_verify: &crate::clone::<Option<bool>>(args.insecure_skip_verify),
                keep_remotely: &crate::clone::<Option<bool>>(args.keep_remotely),
                name: &crate::clone::<Option<String>>(args.name),
                triggers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(
                    args.triggers,
                ),
            },
        );

        RegistryImageResult {
            insecure_skip_verify: crate::random_to_domain_mapper::<Option<bool>>(
                result.insecure_skip_verify,
            ),
            keep_remotely: crate::random_to_domain_mapper::<Option<bool>>(result.keep_remotely),
            name: crate::random_to_domain_mapper::<String>(result.name),
            sha256_digest: crate::random_to_domain_mapper::<String>(result.sha256_digest),
            triggers: crate::random_to_domain_mapper::<
                Option<std::collections::HashMap<String, String>>,
            >(result.triggers),
        }
    }
}

pub mod remote_image {

    pub struct RemoteImageArgs {
        pub build: pulumi_wasm_rust::Output<Option<crate::types::RemoteImageBuild>>,
        pub force_remove: pulumi_wasm_rust::Output<Option<bool>>,
        pub keep_locally: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub platform: pulumi_wasm_rust::Output<Option<String>>,
        pub pull_triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    pub struct RemoteImageResult {
        pub build: pulumi_wasm_rust::Output<Option<crate::types::RemoteImageBuild>>,
        pub force_remove: pulumi_wasm_rust::Output<Option<bool>>,
        pub image_id: pulumi_wasm_rust::Output<String>,
        pub keep_locally: pulumi_wasm_rust::Output<Option<bool>>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub platform: pulumi_wasm_rust::Output<Option<String>>,
        pub pull_triggers: pulumi_wasm_rust::Output<Option<Vec<String>>>,
        pub repo_digest: pulumi_wasm_rust::Output<String>,
        pub triggers: pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
    }

    pub fn remote_image(name: &str, args: RemoteImageArgs) -> RemoteImageResult {
        let result = crate::bindings::pulumi::docker::remote_image::invoke(
            name,
            &crate::bindings::pulumi::docker::remote_image::Args {
                build: &crate::clone::<Option<crate::types::RemoteImageBuild>>(args.build),
                force_remove: &crate::clone::<Option<bool>>(args.force_remove),
                keep_locally: &crate::clone::<Option<bool>>(args.keep_locally),
                name: &crate::clone::<String>(args.name),
                platform: &crate::clone::<Option<String>>(args.platform),
                pull_triggers: &crate::clone::<Option<Vec<String>>>(args.pull_triggers),
                triggers: &crate::clone::<Option<std::collections::HashMap<String, String>>>(
                    args.triggers,
                ),
            },
        );

        RemoteImageResult {
            build: crate::random_to_domain_mapper::<Option<crate::types::RemoteImageBuild>>(
                result.build,
            ),
            force_remove: crate::random_to_domain_mapper::<Option<bool>>(result.force_remove),
            image_id: crate::random_to_domain_mapper::<String>(result.image_id),
            keep_locally: crate::random_to_domain_mapper::<Option<bool>>(result.keep_locally),
            name: crate::random_to_domain_mapper::<String>(result.name),
            platform: crate::random_to_domain_mapper::<Option<String>>(result.platform),
            pull_triggers: crate::random_to_domain_mapper::<Option<Vec<String>>>(
                result.pull_triggers,
            ),
            repo_digest: crate::random_to_domain_mapper::<String>(result.repo_digest),
            triggers: crate::random_to_domain_mapper::<
                Option<std::collections::HashMap<String, String>>,
            >(result.triggers),
        }
    }
}

pub mod secret {

    pub struct SecretArgs {
        pub data: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::SecretLabel>>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct SecretResult {
        pub data: pulumi_wasm_rust::Output<String>,
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::SecretLabel>>>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub fn secret(name: &str, args: SecretArgs) -> SecretResult {
        let result = crate::bindings::pulumi::docker::secret::invoke(
            name,
            &crate::bindings::pulumi::docker::secret::Args {
                data: &crate::clone::<String>(args.data),
                labels: &crate::clone::<Option<Vec<crate::types::SecretLabel>>>(args.labels),
                name: &crate::clone::<Option<String>>(args.name),
            },
        );

        SecretResult {
            data: crate::random_to_domain_mapper::<String>(result.data),
            labels: crate::random_to_domain_mapper::<Option<Vec<crate::types::SecretLabel>>>(
                result.labels,
            ),
            name: crate::random_to_domain_mapper::<String>(result.name),
        }
    }
}

pub mod service {

    pub struct ServiceArgs {
        pub auth: pulumi_wasm_rust::Output<Option<crate::types::ServiceAuth>>,
        pub converge_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceConvergeConfig>>,
        pub endpoint_spec: pulumi_wasm_rust::Output<Option<crate::types::ServiceEndpointSpec>>,
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::ServiceLabel>>>,
        pub mode: pulumi_wasm_rust::Output<Option<crate::types::ServiceMode>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
        pub rollback_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceRollbackConfig>>,
        pub task_spec: pulumi_wasm_rust::Output<crate::types::ServiceTaskSpec>,
        pub update_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceUpdateConfig>>,
    }

    pub struct ServiceResult {
        pub auth: pulumi_wasm_rust::Output<Option<crate::types::ServiceAuth>>,
        pub converge_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceConvergeConfig>>,
        pub endpoint_spec: pulumi_wasm_rust::Output<crate::types::ServiceEndpointSpec>,
        pub labels: pulumi_wasm_rust::Output<Vec<crate::types::ServiceLabel>>,
        pub mode: pulumi_wasm_rust::Output<crate::types::ServiceMode>,
        pub name: pulumi_wasm_rust::Output<String>,
        pub rollback_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceRollbackConfig>>,
        pub task_spec: pulumi_wasm_rust::Output<crate::types::ServiceTaskSpec>,
        pub update_config: pulumi_wasm_rust::Output<Option<crate::types::ServiceUpdateConfig>>,
    }

    pub fn service(name: &str, args: ServiceArgs) -> ServiceResult {
        let result = crate::bindings::pulumi::docker::service::invoke(
            name,
            &crate::bindings::pulumi::docker::service::Args {
                auth: &crate::clone::<Option<crate::types::ServiceAuth>>(args.auth),
                converge_config: &crate::clone::<Option<crate::types::ServiceConvergeConfig>>(
                    args.converge_config,
                ),
                endpoint_spec: &crate::clone::<Option<crate::types::ServiceEndpointSpec>>(
                    args.endpoint_spec,
                ),
                labels: &crate::clone::<Option<Vec<crate::types::ServiceLabel>>>(args.labels),
                mode: &crate::clone::<Option<crate::types::ServiceMode>>(args.mode),
                name: &crate::clone::<Option<String>>(args.name),
                rollback_config: &crate::clone::<Option<crate::types::ServiceRollbackConfig>>(
                    args.rollback_config,
                ),
                task_spec: &crate::clone::<crate::types::ServiceTaskSpec>(args.task_spec),
                update_config: &crate::clone::<Option<crate::types::ServiceUpdateConfig>>(
                    args.update_config,
                ),
            },
        );

        ServiceResult {
            auth: crate::random_to_domain_mapper::<Option<crate::types::ServiceAuth>>(result.auth),
            converge_config: crate::random_to_domain_mapper::<
                Option<crate::types::ServiceConvergeConfig>,
            >(result.converge_config),
            endpoint_spec: crate::random_to_domain_mapper::<crate::types::ServiceEndpointSpec>(
                result.endpoint_spec,
            ),
            labels: crate::random_to_domain_mapper::<Vec<crate::types::ServiceLabel>>(
                result.labels,
            ),
            mode: crate::random_to_domain_mapper::<crate::types::ServiceMode>(result.mode),
            name: crate::random_to_domain_mapper::<String>(result.name),
            rollback_config: crate::random_to_domain_mapper::<
                Option<crate::types::ServiceRollbackConfig>,
            >(result.rollback_config),
            task_spec: crate::random_to_domain_mapper::<crate::types::ServiceTaskSpec>(
                result.task_spec,
            ),
            update_config: crate::random_to_domain_mapper::<
                Option<crate::types::ServiceUpdateConfig>,
            >(result.update_config),
        }
    }
}

pub mod service_config {

    pub struct ServiceConfigArgs {
        pub data: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct ServiceConfigResult {
        pub data: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub fn service_config(name: &str, args: ServiceConfigArgs) -> ServiceConfigResult {
        let result = crate::bindings::pulumi::docker::service_config::invoke(
            name,
            &crate::bindings::pulumi::docker::service_config::Args {
                data: &crate::clone::<String>(args.data),
                name: &crate::clone::<Option<String>>(args.name),
            },
        );

        ServiceConfigResult {
            data: crate::random_to_domain_mapper::<String>(result.data),
            name: crate::random_to_domain_mapper::<String>(result.name),
        }
    }
}

pub mod tag {

    pub struct TagArgs {
        pub source_image: pulumi_wasm_rust::Output<String>,
        pub target_image: pulumi_wasm_rust::Output<String>,
    }

    pub struct TagResult {
        pub source_image: pulumi_wasm_rust::Output<String>,
        pub source_image_id: pulumi_wasm_rust::Output<String>,
        pub target_image: pulumi_wasm_rust::Output<String>,
    }

    pub fn tag(name: &str, args: TagArgs) -> TagResult {
        let result = crate::bindings::pulumi::docker::tag::invoke(
            name,
            &crate::bindings::pulumi::docker::tag::Args {
                source_image: &crate::clone::<String>(args.source_image),
                target_image: &crate::clone::<String>(args.target_image),
            },
        );

        TagResult {
            source_image: crate::random_to_domain_mapper::<String>(result.source_image),
            source_image_id: crate::random_to_domain_mapper::<String>(result.source_image_id),
            target_image: crate::random_to_domain_mapper::<String>(result.target_image),
        }
    }
}

pub mod volume {

    pub struct VolumeArgs {
        pub driver: pulumi_wasm_rust::Output<Option<String>>,
        pub driver_opts:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::VolumeLabel>>>,
        pub name: pulumi_wasm_rust::Output<Option<String>>,
    }

    pub struct VolumeResult {
        pub driver: pulumi_wasm_rust::Output<String>,
        pub driver_opts:
            pulumi_wasm_rust::Output<Option<std::collections::HashMap<String, String>>>,
        pub labels: pulumi_wasm_rust::Output<Option<Vec<crate::types::VolumeLabel>>>,
        pub mountpoint: pulumi_wasm_rust::Output<String>,
        pub name: pulumi_wasm_rust::Output<String>,
    }

    pub fn volume(name: &str, args: VolumeArgs) -> VolumeResult {
        let result = crate::bindings::pulumi::docker::volume::invoke(
            name,
            &crate::bindings::pulumi::docker::volume::Args {
                driver: &crate::clone::<Option<String>>(args.driver),
                driver_opts: &crate::clone::<Option<std::collections::HashMap<String, String>>>(
                    args.driver_opts,
                ),
                labels: &crate::clone::<Option<Vec<crate::types::VolumeLabel>>>(args.labels),
                name: &crate::clone::<Option<String>>(args.name),
            },
        );

        VolumeResult {
            driver: crate::random_to_domain_mapper::<String>(result.driver),
            driver_opts: crate::random_to_domain_mapper::<
                Option<std::collections::HashMap<String, String>>,
            >(result.driver_opts),
            labels: crate::random_to_domain_mapper::<Option<Vec<crate::types::VolumeLabel>>>(
                result.labels,
            ),
            mountpoint: crate::random_to_domain_mapper::<String>(result.mountpoint),
            name: crate::random_to_domain_mapper::<String>(result.name),
        }
    }
}

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
                attach: args.attach.get_inner(),
                capabilities: args.capabilities.get_inner(),
                cgroupns_mode: args.cgroupns_mode.get_inner(),
                command: args.command.get_inner(),
                container_read_refresh_timeout_milliseconds: args
                    .container_read_refresh_timeout_milliseconds
                    .get_inner(),
                cpu_set: args.cpu_set.get_inner(),
                cpu_shares: args.cpu_shares.get_inner(),
                destroy_grace_seconds: args.destroy_grace_seconds.get_inner(),
                devices: args.devices.get_inner(),
                dns: args.dns.get_inner(),
                dns_opts: args.dns_opts.get_inner(),
                dns_searches: args.dns_searches.get_inner(),
                domainname: args.domainname.get_inner(),
                entrypoints: args.entrypoints.get_inner(),
                envs: args.envs.get_inner(),
                gpus: args.gpus.get_inner(),
                group_adds: args.group_adds.get_inner(),
                healthcheck: args.healthcheck.get_inner(),
                hostname: args.hostname.get_inner(),
                hosts: args.hosts.get_inner(),
                image: args.image.get_inner(),
                init: args.init.get_inner(),
                ipc_mode: args.ipc_mode.get_inner(),
                labels: args.labels.get_inner(),
                log_driver: args.log_driver.get_inner(),
                log_opts: args.log_opts.get_inner(),
                logs: args.logs.get_inner(),
                max_retry_count: args.max_retry_count.get_inner(),
                memory: args.memory.get_inner(),
                memory_swap: args.memory_swap.get_inner(),
                mounts: args.mounts.get_inner(),
                must_run: args.must_run.get_inner(),
                name: args.name.get_inner(),
                network_mode: args.network_mode.get_inner(),
                networks_advanced: args.networks_advanced.get_inner(),
                pid_mode: args.pid_mode.get_inner(),
                ports: args.ports.get_inner(),
                privileged: args.privileged.get_inner(),
                publish_all_ports: args.publish_all_ports.get_inner(),
                read_only: args.read_only.get_inner(),
                remove_volumes: args.remove_volumes.get_inner(),
                restart: args.restart.get_inner(),
                rm: args.rm.get_inner(),
                runtime: args.runtime.get_inner(),
                security_opts: args.security_opts.get_inner(),
                shm_size: args.shm_size.get_inner(),
                start: args.start.get_inner(),
                stdin_open: args.stdin_open.get_inner(),
                stop_signal: args.stop_signal.get_inner(),
                stop_timeout: args.stop_timeout.get_inner(),
                storage_opts: args.storage_opts.get_inner(),
                sysctls: args.sysctls.get_inner(),
                tmpfs: args.tmpfs.get_inner(),
                tty: args.tty.get_inner(),
                ulimits: args.ulimits.get_inner(),
                uploads: args.uploads.get_inner(),
                user: args.user.get_inner(),
                userns_mode: args.userns_mode.get_inner(),
                volumes: args.volumes.get_inner(),
                wait: args.wait.get_inner(),
                wait_timeout: args.wait_timeout.get_inner(),
                working_dir: args.working_dir.get_inner(),
            },
        );

        ContainerResult {
            attach: crate::into_domain(result.attach),
            bridge: crate::into_domain(result.bridge),
            capabilities: crate::into_domain(result.capabilities),
            cgroupns_mode: crate::into_domain(result.cgroupns_mode),
            command: crate::into_domain(result.command),
            container_logs: crate::into_domain(result.container_logs),
            container_read_refresh_timeout_milliseconds: crate::into_domain(
                result.container_read_refresh_timeout_milliseconds,
            ),
            cpu_set: crate::into_domain(result.cpu_set),
            cpu_shares: crate::into_domain(result.cpu_shares),
            destroy_grace_seconds: crate::into_domain(result.destroy_grace_seconds),
            devices: crate::into_domain(result.devices),
            dns: crate::into_domain(result.dns),
            dns_opts: crate::into_domain(result.dns_opts),
            dns_searches: crate::into_domain(result.dns_searches),
            domainname: crate::into_domain(result.domainname),
            entrypoints: crate::into_domain(result.entrypoints),
            envs: crate::into_domain(result.envs),
            exit_code: crate::into_domain(result.exit_code),
            gpus: crate::into_domain(result.gpus),
            group_adds: crate::into_domain(result.group_adds),
            healthcheck: crate::into_domain(result.healthcheck),
            hostname: crate::into_domain(result.hostname),
            hosts: crate::into_domain(result.hosts),
            image: crate::into_domain(result.image),
            init: crate::into_domain(result.init),
            ipc_mode: crate::into_domain(result.ipc_mode),
            labels: crate::into_domain(result.labels),
            log_driver: crate::into_domain(result.log_driver),
            log_opts: crate::into_domain(result.log_opts),
            logs: crate::into_domain(result.logs),
            max_retry_count: crate::into_domain(result.max_retry_count),
            memory: crate::into_domain(result.memory),
            memory_swap: crate::into_domain(result.memory_swap),
            mounts: crate::into_domain(result.mounts),
            must_run: crate::into_domain(result.must_run),
            name: crate::into_domain(result.name),
            network_datas: crate::into_domain(result.network_datas),
            network_mode: crate::into_domain(result.network_mode),
            networks_advanced: crate::into_domain(result.networks_advanced),
            pid_mode: crate::into_domain(result.pid_mode),
            ports: crate::into_domain(result.ports),
            privileged: crate::into_domain(result.privileged),
            publish_all_ports: crate::into_domain(result.publish_all_ports),
            read_only: crate::into_domain(result.read_only),
            remove_volumes: crate::into_domain(result.remove_volumes),
            restart: crate::into_domain(result.restart),
            rm: crate::into_domain(result.rm),
            runtime: crate::into_domain(result.runtime),
            security_opts: crate::into_domain(result.security_opts),
            shm_size: crate::into_domain(result.shm_size),
            start: crate::into_domain(result.start),
            stdin_open: crate::into_domain(result.stdin_open),
            stop_signal: crate::into_domain(result.stop_signal),
            stop_timeout: crate::into_domain(result.stop_timeout),
            storage_opts: crate::into_domain(result.storage_opts),
            sysctls: crate::into_domain(result.sysctls),
            tmpfs: crate::into_domain(result.tmpfs),
            tty: crate::into_domain(result.tty),
            ulimits: crate::into_domain(result.ulimits),
            uploads: crate::into_domain(result.uploads),
            user: crate::into_domain(result.user),
            userns_mode: crate::into_domain(result.userns_mode),
            volumes: crate::into_domain(result.volumes),
            wait: crate::into_domain(result.wait),
            wait_timeout: crate::into_domain(result.wait_timeout),
            working_dir: crate::into_domain(result.working_dir),
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
                build: args.build.get_inner(),
                build_on_preview: args.build_on_preview.get_inner(),
                image_name: args.image_name.get_inner(),
                registry: args.registry.get_inner(),
                skip_push: args.skip_push.get_inner(),
            },
        );

        ImageResult {
            base_image_name: crate::into_domain(result.base_image_name),
            context: crate::into_domain(result.context),
            dockerfile: crate::into_domain(result.dockerfile),
            image_name: crate::into_domain(result.image_name),
            platform: crate::into_domain(result.platform),
            registry_server: crate::into_domain(result.registry_server),
            repo_digest: crate::into_domain(result.repo_digest),
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
                attachable: args.attachable.get_inner(),
                check_duplicate: args.check_duplicate.get_inner(),
                driver: args.driver.get_inner(),
                ingress: args.ingress.get_inner(),
                internal: args.internal.get_inner(),
                ipam_configs: args.ipam_configs.get_inner(),
                ipam_driver: args.ipam_driver.get_inner(),
                ipam_options: args.ipam_options.get_inner(),
                ipv6: args.ipv6.get_inner(),
                labels: args.labels.get_inner(),
                name: args.name.get_inner(),
                options: args.options.get_inner(),
            },
        );

        NetworkResult {
            attachable: crate::into_domain(result.attachable),
            check_duplicate: crate::into_domain(result.check_duplicate),
            driver: crate::into_domain(result.driver),
            ingress: crate::into_domain(result.ingress),
            internal: crate::into_domain(result.internal),
            ipam_configs: crate::into_domain(result.ipam_configs),
            ipam_driver: crate::into_domain(result.ipam_driver),
            ipam_options: crate::into_domain(result.ipam_options),
            ipv6: crate::into_domain(result.ipv6),
            labels: crate::into_domain(result.labels),
            name: crate::into_domain(result.name),
            options: crate::into_domain(result.options),
            scope: crate::into_domain(result.scope),
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
                alias: args.alias.get_inner(),
                enable_timeout: args.enable_timeout.get_inner(),
                enabled: args.enabled.get_inner(),
                envs: args.envs.get_inner(),
                force_destroy: args.force_destroy.get_inner(),
                force_disable: args.force_disable.get_inner(),
                grant_all_permissions: args.grant_all_permissions.get_inner(),
                grant_permissions: args.grant_permissions.get_inner(),
                name: args.name.get_inner(),
            },
        );

        PluginResult {
            alias: crate::into_domain(result.alias),
            enable_timeout: crate::into_domain(result.enable_timeout),
            enabled: crate::into_domain(result.enabled),
            envs: crate::into_domain(result.envs),
            force_destroy: crate::into_domain(result.force_destroy),
            force_disable: crate::into_domain(result.force_disable),
            grant_all_permissions: crate::into_domain(result.grant_all_permissions),
            grant_permissions: crate::into_domain(result.grant_permissions),
            name: crate::into_domain(result.name),
            plugin_reference: crate::into_domain(result.plugin_reference),
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
                insecure_skip_verify: args.insecure_skip_verify.get_inner(),
                keep_remotely: args.keep_remotely.get_inner(),
                name: args.name.get_inner(),
                triggers: args.triggers.get_inner(),
            },
        );

        RegistryImageResult {
            insecure_skip_verify: crate::into_domain(result.insecure_skip_verify),
            keep_remotely: crate::into_domain(result.keep_remotely),
            name: crate::into_domain(result.name),
            sha256_digest: crate::into_domain(result.sha256_digest),
            triggers: crate::into_domain(result.triggers),
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
                build: args.build.get_inner(),
                force_remove: args.force_remove.get_inner(),
                keep_locally: args.keep_locally.get_inner(),
                name: args.name.get_inner(),
                platform: args.platform.get_inner(),
                pull_triggers: args.pull_triggers.get_inner(),
                triggers: args.triggers.get_inner(),
            },
        );

        RemoteImageResult {
            build: crate::into_domain(result.build),
            force_remove: crate::into_domain(result.force_remove),
            image_id: crate::into_domain(result.image_id),
            keep_locally: crate::into_domain(result.keep_locally),
            name: crate::into_domain(result.name),
            platform: crate::into_domain(result.platform),
            pull_triggers: crate::into_domain(result.pull_triggers),
            repo_digest: crate::into_domain(result.repo_digest),
            triggers: crate::into_domain(result.triggers),
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
                data: args.data.get_inner(),
                labels: args.labels.get_inner(),
                name: args.name.get_inner(),
            },
        );

        SecretResult {
            data: crate::into_domain(result.data),
            labels: crate::into_domain(result.labels),
            name: crate::into_domain(result.name),
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
                auth: args.auth.get_inner(),
                converge_config: args.converge_config.get_inner(),
                endpoint_spec: args.endpoint_spec.get_inner(),
                labels: args.labels.get_inner(),
                mode: args.mode.get_inner(),
                name: args.name.get_inner(),
                rollback_config: args.rollback_config.get_inner(),
                task_spec: args.task_spec.get_inner(),
                update_config: args.update_config.get_inner(),
            },
        );

        ServiceResult {
            auth: crate::into_domain(result.auth),
            converge_config: crate::into_domain(result.converge_config),
            endpoint_spec: crate::into_domain(result.endpoint_spec),
            labels: crate::into_domain(result.labels),
            mode: crate::into_domain(result.mode),
            name: crate::into_domain(result.name),
            rollback_config: crate::into_domain(result.rollback_config),
            task_spec: crate::into_domain(result.task_spec),
            update_config: crate::into_domain(result.update_config),
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
                data: args.data.get_inner(),
                name: args.name.get_inner(),
            },
        );

        ServiceConfigResult {
            data: crate::into_domain(result.data),
            name: crate::into_domain(result.name),
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
                source_image: args.source_image.get_inner(),
                target_image: args.target_image.get_inner(),
            },
        );

        TagResult {
            source_image: crate::into_domain(result.source_image),
            source_image_id: crate::into_domain(result.source_image_id),
            target_image: crate::into_domain(result.target_image),
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
                driver: args.driver.get_inner(),
                driver_opts: args.driver_opts.get_inner(),
                labels: args.labels.get_inner(),
                name: args.name.get_inner(),
            },
        );

        VolumeResult {
            driver: crate::into_domain(result.driver),
            driver_opts: crate::into_domain(result.driver_opts),
            labels: crate::into_domain(result.labels),
            mountpoint: crate::into_domain(result.mountpoint),
            name: crate::into_domain(result.name),
        }
    }
}

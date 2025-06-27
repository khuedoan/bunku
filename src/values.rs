use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Values {
    #[serde(default)]
    pub global: GlobalConfig,

    #[serde(default = "default_pod_options_strategy")]
    pub default_pod_options_strategy: String,

    #[serde(default)]
    pub default_pod_options: PodOptions,

    #[serde(default)]
    pub controllers: IndexMap<String, Controller>,

    #[serde(default)]
    pub service_account: IndexMap<String, ServiceAccount>,

    #[serde(default)]
    pub config_maps: IndexMap<String, ConfigMap>,

    #[serde(default)]
    pub secrets: IndexMap<String, Secret>,

    #[serde(default)]
    pub ingress: IndexMap<String, Ingress>,

    #[serde(default)]
    pub route: IndexMap<String, Route>,

    #[serde(default)]
    pub service: IndexMap<String, Service>,

    #[serde(default)]
    pub service_monitor: IndexMap<String, ServiceMonitor>,

    #[serde(default)]
    pub networkpolicies: IndexMap<String, NetworkPolicy>,

    #[serde(default)]
    pub persistence: IndexMap<String, PersistenceItem>,

    #[serde(default)]
    pub rbac: RbacConfig,

    #[serde(default)]
    pub raw_resources: IndexMap<String, RawResource>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlobalConfig {
    #[serde(default)]
    pub propagate_global_metadata_to_pods: bool,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PodOptions {
    #[serde(default)]
    pub affinity: Option<serde_json::Value>,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub automount_service_account_token: bool,
    #[serde(default)]
    pub dns_config: Option<serde_json::Value>,
    pub dns_policy: Option<String>,
    #[serde(default)]
    pub enable_service_links: bool,
    pub hostname: Option<String>,
    #[serde(default)]
    pub host_aliases: Vec<serde_json::Value>,
    #[serde(default)]
    pub host_ipc: bool,
    #[serde(default)]
    pub host_network: bool,
    #[serde(default)]
    pub host_pid: bool,
    pub host_users: Option<bool>,
    #[serde(default)]
    pub image_pull_secrets: Vec<String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub node_selector: HashMap<String, String>,
    pub priority_class_name: Option<String>,
    pub restart_policy: Option<String>,
    pub runtime_class_name: Option<String>,
    pub scheduler_name: Option<String>,
    #[serde(default)]
    pub security_context: Option<serde_json::Value>,
    pub share_process_namespace: Option<bool>,
    pub termination_grace_period_seconds: Option<i64>,
    #[serde(default)]
    pub tolerations: Vec<serde_json::Value>,
    #[serde(default)]
    pub topology_spread_constraints: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Controller {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_controller_type")]
    pub r#type: String,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    pub replicas: Option<i32>,
    pub strategy: Option<String>,
    pub rolling_update: Option<RollingUpdateConfig>,
    #[serde(default = "default_revision_history_limit")]
    pub revision_history_limit: i32,
    pub service_account: Option<ServiceAccountRef>,
    pub cronjob: Option<CronJobConfig>,
    pub job: Option<JobConfig>,
    pub statefulset: Option<StatefulSetConfig>,
    #[serde(default)]
    pub containers: IndexMap<String, Container>,
    #[serde(default)]
    pub init_containers: IndexMap<String, Container>,
    #[serde(default)]
    pub pod_options: PodOptions,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Container {
    pub image: String,
    pub command: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
    #[serde(default)]
    pub env: IndexMap<String, EnvVar>,
    #[serde(default)]
    pub ports: IndexMap<String, ContainerPort>,
    pub resources: Option<ResourceRequirements>,
    pub security_context: Option<serde_json::Value>,
    #[serde(default)]
    pub volume_mounts: Vec<VolumeMount>,
    pub liveness_probe: Option<serde_json::Value>,
    pub readiness_probe: Option<serde_json::Value>,
    pub startup_probe: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvVar {
    pub value: Option<String>,
    pub value_from: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerPort {
    pub container_port: i32,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceRequirements {
    pub requests: Option<HashMap<String, String>>,
    pub limits: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMount {
    pub name: String,
    pub mount_path: String,
    pub read_only: Option<bool>,
    pub sub_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RollingUpdateConfig {
    pub unavailable: Option<String>,
    pub surge: Option<String>,
    pub partition: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceAccountRef {
    pub identifier: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CronJobConfig {
    pub suspend: Option<bool>,
    #[serde(default = "default_concurrency_policy")]
    pub concurrency_policy: String,
    pub time_zone: Option<String>,
    #[serde(default = "default_cron_schedule")]
    pub schedule: String,
    pub starting_deadline_seconds: Option<i64>,
    #[serde(default = "default_successful_jobs_history")]
    pub successful_jobs_history: i32,
    #[serde(default = "default_failed_jobs_history")]
    pub failed_jobs_history: i32,
    pub ttl_seconds_after_finished: Option<i32>,
    #[serde(default = "default_backoff_limit")]
    pub backoff_limit: i32,
    pub parallelism: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobConfig {
    pub suspend: Option<bool>,
    pub ttl_seconds_after_finished: Option<i32>,
    #[serde(default = "default_backoff_limit")]
    pub backoff_limit: i32,
    pub parallelism: Option<i32>,
    pub completions: Option<i32>,
    pub completion_mode: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetConfig {
    pub pod_management_policy: Option<String>,
    #[serde(default)]
    pub volume_claim_templates: Vec<PersistenceItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccount {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    pub automount_service_account_token: Option<bool>,
    #[serde(default)]
    pub secrets: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigMap {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default = "default_secret_type")]
    pub r#type: String,
    #[serde(default)]
    pub data: HashMap<String, String>,
    #[serde(default)]
    pub string_data: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingress {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    pub class_name: Option<String>,
    #[serde(default)]
    pub hosts: Vec<IngressHost>,
    #[serde(default)]
    pub tls: Vec<IngressTls>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IngressHost {
    pub host: String,
    #[serde(default)]
    pub paths: Vec<IngressPath>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressPath {
    pub path: String,
    #[serde(default = "default_path_type")]
    pub path_type: String,
    pub service: IngressService,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IngressService {
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IngressTls {
    pub secret_name: String,
    #[serde(default)]
    pub hosts: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    pub class_name: Option<String>,
    #[serde(default)]
    pub hosts: Vec<String>,
    #[serde(default)]
    pub rules: Vec<RouteRule>,
    #[serde(default)]
    pub tls: Vec<RouteTls>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteRule {
    pub matches: Vec<RouteMatch>,
    pub backend_refs: Vec<RouteBackendRef>,
    pub filters: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteMatch {
    pub path: Option<RoutePathMatch>,
    pub headers: Option<Vec<RouteHeaderMatch>>,
    pub query_params: Option<Vec<RouteQueryParamMatch>>,
    pub method: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutePathMatch {
    #[serde(default = "default_path_match_type")]
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteHeaderMatch {
    pub name: String,
    #[serde(default = "default_header_match_type")]
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteQueryParamMatch {
    pub name: String,
    #[serde(default = "default_query_param_match_type")]
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteBackendRef {
    pub name: String,
    pub port: Option<i32>,
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteTls {
    #[serde(default)]
    pub hosts: Vec<String>,
    pub secret_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default = "default_service_type")]
    pub r#type: String,
    pub cluster_ip: Option<String>,
    pub load_balancer_ip: Option<String>,
    pub load_balancer_source_ranges: Option<Vec<String>>,
    pub load_balancer_class: Option<String>,
    pub external_name: Option<String>,
    pub internal_traffic_policy: Option<String>,
    pub external_traffic_policy: Option<String>,
    pub allocate_load_balancer_node_ports: Option<bool>,
    pub session_affinity: Option<String>,
    pub session_affinity_config: Option<serde_json::Value>,
    pub external_ips: Option<Vec<String>>,
    pub publish_not_ready_addresses: Option<bool>,
    pub ip_family_policy: Option<String>,
    pub ip_families: Option<Vec<String>>,
    #[serde(default)]
    pub ports: IndexMap<String, ServicePort>,
    pub controller: Option<String>,
    #[serde(default)]
    pub extra_selector_labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicePort {
    pub port: i32,
    pub target_port: Option<i32>,
    pub protocol: Option<String>,
    pub node_port: Option<i32>,
    pub app_protocol: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceMonitor {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default = "default_service_monitor_interval")]
    pub interval: String,
    pub scrape_timeout: Option<String>,
    pub path: Option<String>,
    #[serde(default)]
    pub endpoints: Vec<ServiceMonitorEndpoint>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceMonitorEndpoint {
    pub port: String,
    pub path: Option<String>,
    pub interval: Option<String>,
    pub scrape_timeout: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPolicy {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub policy_types: Vec<String>,
    #[serde(default)]
    pub ingress: Vec<NetworkPolicyRule>,
    #[serde(default)]
    pub egress: Vec<NetworkPolicyRule>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkPolicyRule {
    #[serde(default)]
    pub from: Vec<serde_json::Value>,
    #[serde(default)]
    pub to: Vec<serde_json::Value>,
    #[serde(default)]
    pub ports: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistenceItem {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default = "default_persistence_type")]
    pub r#type: String,
    pub storage_class: Option<String>,
    pub size: Option<String>,
    #[serde(default)]
    pub access_modes: Vec<String>,
    pub data_source: Option<serde_json::Value>,
    pub data_source_ref: Option<serde_json::Value>,
    #[serde(default)]
    pub global_mounts: Vec<VolumeMount>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RbacConfig {
    #[serde(default)]
    pub roles: IndexMap<String, Role>,
    #[serde(default)]
    pub bindings: IndexMap<String, RoleBinding>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub cluster_wide: bool,
    #[serde(default)]
    pub rules: Vec<PolicyRule>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyRule {
    #[serde(default)]
    pub api_groups: Vec<String>,
    #[serde(default)]
    pub resources: Vec<String>,
    #[serde(default)]
    pub verbs: Vec<String>,
    #[serde(default)]
    pub resource_names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleBinding {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub cluster_wide: bool,
    pub role_ref: RoleRef,
    #[serde(default)]
    pub subjects: Vec<RoleSubject>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleRef {
    pub api_group: String,
    pub kind: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleSubject {
    pub kind: String,
    pub name: String,
    pub api_group: Option<String>,
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RawResource {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub spec: serde_json::Value,
}

// Default functions
fn default_enabled() -> bool {
    true
}
fn default_controller_type() -> String {
    "deployment".to_string()
}
fn default_revision_history_limit() -> i32 {
    3
}
fn default_concurrency_policy() -> String {
    "Forbid".to_string()
}
fn default_cron_schedule() -> String {
    "*/20 * * * *".to_string()
}
fn default_successful_jobs_history() -> i32 {
    1
}
fn default_failed_jobs_history() -> i32 {
    1
}
fn default_backoff_limit() -> i32 {
    6
}
fn default_secret_type() -> String {
    "Opaque".to_string()
}
fn default_path_type() -> String {
    "Prefix".to_string()
}
fn default_service_type() -> String {
    "ClusterIP".to_string()
}
fn default_service_monitor_interval() -> String {
    "30s".to_string()
}
fn default_persistence_type() -> String {
    "pvc".to_string()
}
fn default_pod_options_strategy() -> String {
    "overwrite".to_string()
}
fn default_path_match_type() -> String {
    "PathPrefix".to_string()
}
fn default_header_match_type() -> String {
    "Exact".to_string()
}
fn default_query_param_match_type() -> String {
    "Exact".to_string()
}

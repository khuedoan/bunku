use super::{ResourceGenerator, common};
use crate::values::Values;
use gateway_api::apis::v1::{HTTPRoute, HTTPRouteSpec, HTTPRouteRule, HTTPRouteMatch, HTTPPathMatch, HTTPHeaderMatch, HTTPQueryParamMatch, HTTPBackendRef, BackendRef, ParentReference};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

pub struct HttpRouteGenerator;

impl ResourceGenerator for HttpRouteGenerator {
    type Output = HTTPRoute;
    
    fn generate(&self, values: &Values, name: &str) -> Option<Self::Output> {
        let route_config = values.route.get(name)?;
        
        if !route_config.enabled {
            return None;
        }
        
        let route_name = common::generate_name(values, name);
        
        let mut labels = common::generate_labels(values, name, "httproute");
        labels.extend(route_config.labels.iter().map(|(k, v)| (k.clone(), v.clone())));
        
        let mut annotations = common::generate_annotations(values);
        annotations.extend(route_config.annotations.iter().map(|(k, v)| (k.clone(), v.clone())));
        
        // Convert rules from our values format to gateway-api format
        let rules: Vec<HTTPRouteRule> = route_config.rules.iter()
            .filter_map(|rule| {
                // Convert matches
                let matches = rule.matches.iter().filter_map(|m| {
                    let mut http_match = HTTPRouteMatch::default();
                    
                    // Convert path match
                    if let Some(path) = &m.path {
                        http_match.path = Some(HTTPPathMatch {
                            type_: Some(path.r#type.clone()),
                            value: Some(path.value.clone()),
                        });
                    }
                    
                    // Convert method
                    if let Some(method) = &m.method {
                        http_match.method = Some(method.clone());
                    }
                    
                    // Convert headers
                    if let Some(headers) = &m.headers {
                        http_match.headers = Some(headers.iter().map(|h| {
                            HTTPHeaderMatch {
                                type_: Some(h.r#type.clone()),
                                name: h.name.clone(),
                                value: h.value.clone(),
                            }
                        }).collect());
                    }
                    
                    // Convert query params
                    if let Some(query_params) = &m.query_params {
                        http_match.query_params = Some(query_params.iter().map(|q| {
                            HTTPQueryParamMatch {
                                type_: Some(q.r#type.clone()),
                                name: q.name.clone(),
                                value: q.value.clone(),
                            }
                        }).collect());
                    }
                    
                    Some(http_match)
                }).collect();
                
                // Convert backend refs
                let backend_refs = rule.backend_refs.iter().map(|br| {
                    HTTPBackendRef {
                        backend_ref: Some(BackendRef {
                            name: br.name.clone(),
                            port: br.port,
                            weight: br.weight,
                            ..Default::default()
                        }),
                        filters: None, // TODO: Implement filters if needed
                    }
                }).collect();
                
                Some(HTTPRouteRule {
                    matches: if matches.is_empty() { None } else { Some(matches) },
                    backend_refs: Some(backend_refs),
                    filters: rule.filters.as_ref().and_then(|_filters| {
                        // Convert filters from JSON values to proper types
                        // This would need more specific implementation based on filter types
                        None
                    }),
                    timeouts: None,
                    name: None,
                })
            })
            .collect();
        
        let http_route = HTTPRoute {
            metadata: ObjectMeta {
                name: Some(route_name),
                labels: Some(labels),
                annotations: if annotations.is_empty() { 
                    None 
                } else { 
                    Some(annotations) 
                },
                ..Default::default()
            },
            spec: HTTPRouteSpec {
                parent_refs: Some(vec![
                    ParentReference {
                        name: Some("gateway".to_string()), // Default gateway name
                        ..Default::default()
                    }
                ]),
                hostnames: if route_config.hosts.is_empty() { 
                    None 
                } else { 
                    Some(route_config.hosts.clone()) 
                },
                rules: if rules.is_empty() { None } else { Some(rules) },
            },
            status: None,
        };
        
        Some(http_route)
    }
} 
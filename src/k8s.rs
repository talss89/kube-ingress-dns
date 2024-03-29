use std::str::FromStr;
use std::net::Ipv4Addr;
use wildmatch::WildMatch;

use k8s_openapi::api::networking::v1::Ingress;
use k8s_gateway_api::{HttpRoute, Gateway, GatewayAddress};

use kube::{
    api::{Api, ListParams},
    Client, Resource,
};

pub async fn resolve_name(qname: &str) -> Option<Ipv4Addr> {

    let client = Client::try_default().await.ok()?;

    if let Some(addr) = search_ingresses(qname, &client).await {
        return Some(addr);
    }

    if let Some(addr) = search_gateways(qname, &client).await {
        return Some(addr);
    }

    if let Some(addr) = search_httproutes(qname, &client).await {
        return Some(addr);
    }

    None
}

pub async fn resolve_gateway_ip(gateway: Gateway) -> Option<Ipv4Addr> {

    let addresses = gateway.status?.addresses?;

    for addr in addresses {
        if let Ok(ip) = Ipv4Addr::from_str(&addr.value) {
            return Some(ip);
        }
        
    }
        
    None
}

async fn search_gateways(qname: &str, client: &Client) -> Option<Ipv4Addr> {
    let gateways: Api<Gateway> = Api::all(client.to_owned());

    let lp = ListParams::default();

    for gateway in gateways.list(&lp).await.ok()? {
        for listener in &gateway.spec.listeners {
            if let Some(hostname) = &listener.hostname {
                if WildMatch::new(&hostname).matches(qname) {
                    log::info!("Found Gateway {} matching {}", gateway.metadata.name.as_ref()?, qname);
                    return resolve_gateway_ip(gateway).await;
                }
            }
        }
    }

    None
}

async fn search_httproutes(qname: &str, client: &Client) -> Option<Ipv4Addr> {

    let routes: Api<HttpRoute> = Api::all(client.to_owned());

    let lp = ListParams::default(); // for this app only
    for route in routes.list(&lp).await.ok()? {
            if let Some(hostnames) = &route.spec.hostnames {
                for host in hostnames.iter() {

                        if WildMatch::new(&*host).matches(qname) {
                            log::info!("Found HTTPRoute {} matching {}", route.metadata.name.as_ref()?, qname);

                            for parent in route.status.to_owned()?.inner.parents {
                                if let Some(kind) = parent.parent_ref.kind {

                                    if kind == "Gateway" {
                                        log::info!("Found a gateway for {} -> {}", route.metadata.name.as_ref()?, parent.parent_ref.name);

                                        let gateway = Api::<Gateway>::namespaced(client.to_owned(), route.metadata.namespace.as_ref()?).get(&parent.parent_ref.name).await;
                                        if let Ok(gateway) = gateway {
                                            return resolve_gateway_ip(gateway).await;
                                        }
                                        
                                    }
                                }
                            }
                            
                            
                        }
                    
                }
            }
        
    }
    None
}


async fn search_ingresses(qname: &str, client: &Client) -> Option<Ipv4Addr> {
    // Manage ingresses
    let ingresses: Api<Ingress> = Api::all(client.to_owned());

    let lp = ListParams::default(); // for this app only
    for ingress in ingresses.list(&lp).await.ok()? {
        if let Some(spec) = &ingress.spec {

            if let Some(rules) = &spec.rules {
                for r in rules.iter() {
                    if let Some(host) = &r.host {
                        if WildMatch::new(&*host).matches(qname) {
                            log::info!("Found Ingress {} matching {}", ingress.metadata.name?, qname);
                            return Some(Ipv4Addr::from_str(
                                ingress.status.as_ref()?.load_balancer.as_ref()?.ingress.as_ref()?[0].ip.as_ref()?
                            ).ok()?);
                        }
                    }
                }
            }
        }
    }
    None
}
use std::str::FromStr;
use std::net::Ipv4Addr;
use wildmatch::WildMatch;

use k8s_openapi::api::networking::v1::Ingress;

use kube::{
    api::{Api, ListParams},
    Client,
};

pub async fn search_ingresses(qname: &str) -> Option<Ipv4Addr> {
    let client = Client::try_default().await.ok()?;

        // Manage ingresses
        let ingresses: Api<Ingress> = Api::all(client);

        let lp = ListParams::default(); // for this app only
        for ingress in ingresses.list(&lp).await.ok()? {
            if let Some(spec) = &ingress.spec {

                if let Some(rules) = &spec.rules {
                    for r in rules.iter() {
                        if let Some(host) = &r.host {
                            if WildMatch::new(&*host).matches(qname) {
                                log::info!("Found {} matching {}", ingress.metadata.name?, qname);
                                return Some(Ipv4Addr::from_str(
                                    ingress.status.as_ref()?.load_balancer.as_ref()?.ingress.as_ref()?[0].ip.as_ref()?
                                ).ok()?);
                            }
                        }
                    }
                }
            }
        }
        log::info!("No ingress found for {}", qname);
        None
}
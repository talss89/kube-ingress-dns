# ingress-dns

__Expose your k8s ingress rules via local DNS. A rewrite of [Minikube's ingress-dns addon](https://minikube.sigs.k8s.io/docs/handbook/addons/ingress-dns/), but in rust, and for any cluster (K3s, Microk8s, etc).__

## Quickstart

1. Install in your cluster (see `./manifest/ingress-dns.yaml`)
2. Set up your local machine to use the resolver (guide coming soon)

**This is a very early work-in-progress, but I am using this locally on my machine currently. Contributions are very welcome, and encouraged!**

## Why use this?

When running k8s locally, you often need to access services exposed via ingresses. You could manually maintain your `/etc/hosts` file with all the domain names, and respective IP addresses, but this is error-prone and cumbersome. A much better approach is to use DNS.

This pod exposes a DNS server on your k8s node's host network, which will resolve any ingress domains and point them at the correct ingress external IP.

All you need to do is make sure your local machine is configured to use the resolver.

**:warning: This should never be used in production, on a public cluster.**

## What was wrong with `minikube-ingress-dns`?

Aside from being fairly minikube opinionated, there are now a few issues with the `ingress-dns` minikube addon:

- It's not actively maintained, and fails to work on current k8s versions.
- It's fairly inefficient; relying on NodeJS

## How does this project address those issues?

This `kube-ingress-dns` project is:

- Non-opinionated - will work with any k8s cluster
- Lightweight - Built in rust with minimal dependencies
- K8s compliant - Uses `kube-rs` to interact with the cluster

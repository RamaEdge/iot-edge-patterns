# OpenTelemetry

## What is OpenTelemetry?

OpenTelemetry is an open-source observability framework for cloud-native software. It provides a set of APIs, libraries, agents, and instrumentation to enable the collection of distributed traces and metrics from applications. OpenTelemetry aims to make it easy to capture and export telemetry data to various backends, such as Prometheus, Jaeger, and others, for monitoring and analysis.

Key features of OpenTelemetry include:
- **Distributed Tracing**: Captures the flow of requests across services, providing insights into the performance and behavior of distributed systems.
- **Metrics**: Collects and exports metrics data, such as counters, gauges, and histograms, to monitor the health and performance of applications.
- **Context Propagation**: Propagates context information across service boundaries, enabling correlation of telemetry data.

## Prerequisites

Before deploying OpenTelemetry in a Kubernetes cluster, ensure you have the following prerequisites:

1. **Kubernetes Cluster**:
   - A running Kubernetes cluster (version 1.18 or later is recommended).
   - Access to the Kubernetes cluster using `kubectl`.

2. **kubectl**:
   - The Kubernetes command-line tool `kubectl` installed and configured to interact with your cluster.
   - Ensure `kubectl` is configured to use the correct context for your cluster.

## Deploy Using Kustomize

Run the following command to deploy the resources using Kustomize:

```sh
kubectl apply -k open-telemetry
```

This command will apply all the resources defined in the `kustomization.yaml` files, setting up the OpenTelemetry deployment in your Kubernetes cluster.


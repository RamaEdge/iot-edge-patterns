# IoT Edge Patterns


This repository provides patterns on how to deploy an IoT Edge Device. An IoT Edge device can be used as a gateway to local devices and allows you to send local device data to the cloud.

The various use cases for an IoT Edge include:
- Sending application data to the cloud
- Sending local application events and logs to the cloud
- Sending metrics, logs, and traces of local devices to the cloud

## IoT Edge Gateway Pattern

An IoT Edge Gateway can consist of the following modules:
- A Kubernetes Cluster to manage and execute applications
- An MQTT Broker and Client to send MQTT messages to the cloud
- An OPC UA Client to receive OPC UA data from an OPC UA Server
- OpenTelemetry to capture metrics, logs, and traces for applications running on the edge

## Deploying an IoT Edge Gateway

There are multiple ways to deploy applications on the IoT Edge Gateway. You can use `GitOps`, which allows you to synchronize Kubernetes YAML files to the Kubernetes Cluster running on the edge. Another option is using [Image Mode](https://developers.redhat.com/products/rhel-image-mode/overview), which packages the cluster and applications as a single ISO image.

This repository will cover how these methods can be used to deploy an edge easily and quickly.

This repository will cover how these methods can be used to deploy an Edge easily and quickly. 

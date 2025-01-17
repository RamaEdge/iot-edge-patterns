# IoT Edge Patterns


This repository provides patterns on how to deploy an IoT Edge Device. An IoT Edge device can be used as a gateway to local devices and allows you to send local device data to the cloud.

The various use cases for an IoT Edge include:
- Sending application data to the cloud
- Sending local application events and logs to the cloud
- Sending metrics, logs, and traces of local devices to the cloud

The various use cases covered where an IoT Edge would be:

- Sending application data to the cloud
- Sending Local application Events, Logs to Cloud
- Sending Metrics, Logs and Traces of Local Devices to Cloud

## IoT Edge Gateway Pattern

An IoT Edge Gateway can consist of the following modules

- A Kubernetes Cluster to manage and execute applications.

- An MQTT Broker and Client to send MQTT messages to the Cloud. 

- An OPCUA Client to receive OPCUA data from an OPCUA Server

- Open-telemetry to capture Metrics, logs and traces for applications running on the Edge

## Deploying an IoT Edge Gateway

There are multiple ways by which you can deploy applications on the IoT Edge Gateway. You can use `GitOps` which allows you to synchronize Kubernetes yamls to the Kubernetes Cluster running on the Edge.
The other possibilities is by using [Image Mode](https://developers.redhat.com/products/rhel-image-mode/overview). This is a mechanism by which you can package the Cluster and applications as a single ISO Image.

This repository will cover how these methods can be used to deploy an Edge easily and quickly. 

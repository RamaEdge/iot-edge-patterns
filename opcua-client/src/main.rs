use opcua_client::prelude::*;
use tokio;

[tokio::main]
async fn main() {
    // Create a new OPC UA client
    let client = ClientBuilder::new()
        .application_name("OPC UA Client")
        .application_uri("urn:example:opcua:client")
        .create_sample_keypair(true)
        .trust_server_certs(true)
        .client()
        .unwrap();

    // Connect to the OPC UA server
    let endpoint_url = "opc.tcp://localhost:4840";
    let session = client.connect_to_endpoint(endpoint_url, IdentityToken::Anonymous).await.unwrap();

    // Read a variable node
    let node_id = NodeId::new(2, "Demo.Static.Scalar.Double");
    let value = session.read_value(&node_id).await.unwrap();
    println!("Value: {:?}", value);

    // Disconnect from the server
    session.disconnect().await.unwrap();
}
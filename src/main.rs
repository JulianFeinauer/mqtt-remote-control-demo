mod mqtt_controller;

use mqtt_controller::MqttAgent;

#[tokio::main]
async fn main() {
    // Start the Server
    let mut controller = MqttAgent::new("ssl://mqtt.sandbox.drogue.cloud:8883".to_string(), Some("python-sender@e2e-demo".to_string()), Some("secret".to_string())).await;
    match controller.run_forever().await {
        Ok(_) => {
        }
        Err(_) => {
            println!("Finished...")
        }
    }
}

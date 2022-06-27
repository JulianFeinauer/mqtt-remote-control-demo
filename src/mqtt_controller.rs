use std::time::Duration;
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder, CreateOptionsBuilder, MQTT_VERSION_3_1_1, SslOptionsBuilder};
use paho_mqtt::PropertyCode::ResponseTopic;
use futures::{StreamExt};

pub(crate) struct MqttAgent {
    server_url: String,
    user: Option<String>,
    password: Option<String>,
}

impl MqttAgent {
    pub(crate) async fn new(server_url: String, user: Option<String>, password: Option<String>) -> MqttAgent {
        MqttAgent {
            server_url,
            user,
            password,
        }
    }
    pub(crate) async fn run_forever(&self) -> Result<(), ()> {
        let create_opts = CreateOptionsBuilder::new()
            .server_uri(self.server_url.as_str())
            // .client_id(format!("rust_async_publisher-{}", source))
            .finalize();

        // Create the client connection
        let mut client = match AsyncClient::new(create_opts) {
            Ok(c) => c,
            Err(_) => {
                return Err(());
            }
        };

        let ssl_opts = SslOptionsBuilder::new()
            .finalize();

        let mut builder = ConnectOptionsBuilder::new();
        builder
            .ssl_options(ssl_opts)
            .keep_alive_interval(Duration::from_secs(30))
            .mqtt_version(MQTT_VERSION_3_1_1)
            .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(30))
            .clean_session(true);
        if let Some(user) = &self.user {
            builder.user_name(user.as_str());
        }
        if let Some(password) = &self.password {
            builder.password(password.as_str());
        }
        let conn_opts = builder.finalize();

        match client.connect(conn_opts).await {
            Ok(ok) => {
                println!("Yay, connected!");
            }
            Err(e) => {
                println!("Unable to connect: {:?}", e);
                return Err(());
            }
        };

        // Set Handler for reconnect
        client.set_connected_callback(|_| {
            println!("Connection established!")
        });

        // Subscribe to commands
        client.subscribe("command/inbox/+/#", 0).await;
        println!("Subscribed to commadn topic");

        // Now start the consume / repeat loop
        let mut stream = client.get_stream(25);

        while let Some(msg) = stream.next().await {
            match msg {
                Some(msg) => {
                    println!("Message: {}", msg);
                    // let event: Event = serde_json::from_str(msg.payload_str().as_ref()).expect("Waaah");
                    // println!("Event: {:?}", event);
                    // let data: &Data = event.data().unwrap();
                    // match data {
                    //     Data::Binary(_) => {}
                    //     Data::String(_) => {}
                    //     Data::Json(json) => {
                    //         println!("Json Payload: {}", json);
                    //         match json {
                    //             _ => {
                    //                 // intentionally do nothing
                    //             }
                    //             Value::Object(map) => {
                    //                 for (key, value) in map {
                    //                     match value {
                    //                         Value::Number(s) => {
                    //                             println!("{} -> {}", key, value)
                    //                         }
                    //                         Value::String(s) => {
                    //                             println!("{} -> {}", key, value)
                    //                         }
                    //                         _ => {
                    //                             // do nothing
                    //                         }
                    //                     }
                    //                 }
                    //             }
                    //         }
                    //     }
                }
                None => {
                    println!("we are disconnected!");
                    // return Err(())
                }
            }
        }
        Ok(())
    }
}

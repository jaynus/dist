macro_rules! bootstrap_remote_client {
    () => {
        pub async fn bootstrap_client(connection_string: &str, config: tarpc::client::Config) -> ::std::io::Result<Client> {
            let c_str = &connection_string.parse().unwrap();
            let transport = await!(tarpc_bincode_transport::connect(c_str))?;
            let client = await!(new_stub(config, transport))?;
            Ok(client)
        }

        #[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
        pub struct RemoteInfo {
            id: u64,
            connection_string: String,
        }
        impl RemoteInfo {
            pub fn new(id: u64, connection_string: &str) -> Self {
                Self {
                    id,
                    connection_string: connection_string.to_string(),
                }
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub struct Remote {
            info: Arc<RemoteInfo>,
            #[serde(skip_serializing, skip_deserializing)]
            client: Option<Client>,
        }
        impl Remote {
            pub async fn bootstrap(info: Arc<RemoteInfo>, ) -> std::io::Result<Self> {
                let client = await!(bootstrap_client(&info.connection_string, tarpc::client::Config::default()))?;
                Ok(Self {
                    info,
                    client: Some(client),
                })
            }

            pub fn client(&mut self) -> &mut Client {
                self.client.as_mut().unwrap()
            }

            pub fn info(&self) -> Arc<RemoteInfo> {
                self.info.clone()
            }
        }
        impl PartialEq<Remote> for Remote {
            fn eq(&self, other: &Remote) -> bool {
                other.info == self.info
            }
        }
    };
}
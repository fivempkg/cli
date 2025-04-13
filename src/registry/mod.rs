use aws_config::{BehaviorVersion, meta::region::RegionProviderChain};
use aws_sdk_s3::{
    Client,
    config::endpoint::{Endpoint, EndpointFuture, Params, ResolveEndpoint},
};

#[derive(Debug, Default)]
struct RegistryResolver {}
impl ResolveEndpoint for RegistryResolver {
    fn resolve_endpoint(&self, _params: &Params) -> EndpointFuture<'_> {
        let s3_endpoint = std::env::var("S3_ENDPOINT").expect("S3_ENDPOINT not set");
        let endpoint = EndpointFuture::ready(Ok(Endpoint::builder().url(s3_endpoint).build()));

        return endpoint;
    }
}

pub struct Registry {}
impl Registry {
    async fn get_client() -> Result<Client, ()> {
        let region_provider = RegionProviderChain::default_provider().or_else("eeur");
        let region = region_provider.region().await.unwrap();

        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
        let config = aws_sdk_s3::config::Builder::from(&config)
            .endpoint_resolver(RegistryResolver::default())
            .region(region)
            .build();

        let client = Client::from_conf(config);
        Ok(client)
    }

    pub async fn download_package(key: &str) -> Result<String, ()> {
        let client = match Registry::get_client().await {
            Ok(client) => client,
            Err(_) => {
                println!("Failed to create S3 client");
                return Err(());
            }
        };

        let package_file = client
            .get_object()
            .bucket("fivempkg-registry")
            .key(key)
            .send()
            .await;

        let package_file = match package_file {
            Ok(file) => file,
            Err(_) => {
                println!("Failed to get object from S3");
                return Err(());
            }
        };

        let data = match package_file.body.collect().await {
            Ok(data) => data,
            Err(_) => {
                println!("Failed to collect object body");
                return Err(());
            }
        };
        let b = data.into_bytes();

        let mut file = std::fs::File::create(key).unwrap();
        std::io::copy(&mut b.as_ref(), &mut file).unwrap();

        Ok(key.to_string())
    }
}

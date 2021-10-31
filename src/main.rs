use std::collections::HashMap;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client as DynamoDB, Error, Region, PKG_VERSION, Blob};
use aws_sdk_dynamodb::model::AttributeValue;
use structopt::StructOpt;
use regex::Regex;
use csv2dynamodb::read_csv;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    #[structopt(short, long)]
    file_path: String,

    #[structopt(short, long)]
    table: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt { region, file_path, table, verbose } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let result = read_csv(file_path.as_str());
    if result.is_ok() {
        let csv = result.unwrap();
        let headers = &csv.headers;
        let records = csv.records;

        if verbose {
            println!("DynamoDB client version: {}", PKG_VERSION);
            println!(
                "Region:                  {}",
                shared_config.region().unwrap()
            );
            println!();
        }
        let client = DynamoDB::new(&shared_config);
        let empty = &String::from("");
        for record in &records {
            let mut attributes = HashMap::new();
            headers.iter()
                .for_each(|header| {
                    let caps = Regex::new(r"\([A-Z]+\)$")
                        .unwrap()
                        .captures(header.as_str())
                        .map(|c| c.get(0).unwrap().as_str())
                        .or(Some("(S)"));
                    let attr_type = Regex::new(r"[A-Z]+")
                            .unwrap()
                            .captures(caps.unwrap())
                            .map(|c| c.get(0).unwrap().as_str())
                            .unwrap();
                    let value = record.get(header.as_str()).unwrap_or(empty);
                    let attr_value = match attr_type {
                        "N" => AttributeValue::N(value.to_string()),
                        "NS" => AttributeValue::Ns(value.to_string().split("'").map(|v|v.to_string()).collect()),
                        "BOOL" => AttributeValue::Bool(value.to_string().parse().unwrap()),
                        "B" => AttributeValue::B(Blob::new(value.as_bytes())),
                        "BS" => AttributeValue::Bs(value.to_string().split("'").map(|v|Blob::new(v)).collect()),
                        "SS" => AttributeValue::Ss(value.to_string().split("'").map(|v|v.to_string()).collect()),
                        _ => AttributeValue::S(value.to_string()),
                    };
                    let header_value = Regex::new(r"[A-Z]+").unwrap()
                        .captures(header.as_str()).map_or(header.as_str(),|cap| cap.get(0).unwrap().as_str());
                    attributes.insert(header_value.to_string(), attr_value);
                });
            println!("{:?}", attributes);
            let request = client
                .put_item()
                .table_name(&table)
                .set_item(Some(attributes));

            println!("Executing request [{:?}] to add item...", request);

            request.send().await?;
        }
    }
    Ok(())
}


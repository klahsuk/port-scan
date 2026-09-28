use futures::{StreamExt, stream};
use port_scan::{error::Error::CliUsage, model::Subdomain, ports, subdomains};
use reqwest::{Client, redirect};
use std::{env, time::{Duration, Instant}};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(CliUsage.into());
    }

    let target = args[1].as_str();
    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(3))
        .timeout(http_timeout)
        .build()?;

    // set upper limit on concurrency for ports and subdomains

    let port_concurrency = 200;
    let subdomain_concurrency = 100;

    let scan_start = Instant::now();

    //subdomains require run enumerate on subdomains
    let subdomains: Vec<Subdomain> = subdomains::enumerate(&http_client, target).await?;

    // Concurrent stream method 1: Using buffer_unordered + collect
    // run port scans using stream::iter
    let scan_result: Vec<Subdomain> = stream::iter(subdomains.into_iter())
        .map(|subdomain| ports::scan_ports(port_concurrency, subdomain))
        .buffer_unordered(subdomain_concurrency)
        .collect()
        .await;

    let scan_duration = scan_start.elapsed();
    println!("Scan completed in {:?}", scan_duration);


    // print the ports
    for subdomain in scan_result {
        println!("{}:", &subdomain.domain);
        for port in &subdomain.open_ports {
            println!("    {}: open", port.port);
        }

        println!("");
    }

    Ok(())
}

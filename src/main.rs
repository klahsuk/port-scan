use std::{env, time::Duration};

use port_scan::{error::Error::CliUsage, model::Subdomain, ports, subdomains};
use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};


fn main() -> Result<(), anyhow::Error> {

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

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()?;

    // enumerate client
    // run port scans
    // print the ports
    thread_pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();
        
        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);

            for port in &subdomain.open_ports {
                println!("{}", port.port);
            }
        }
    });

    Ok(())
}

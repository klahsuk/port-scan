use std::collections::HashSet;

use reqwest::blocking::{Client};

use crate::{error::Error, model::{CrtShEntry, Subdomain}};

pub fn enumerate(http_client: &Client, target: &str) -> Result<Vec<Subdomain>, Error> {
    let entries: Vec<CrtShEntry> = http_client
    .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
    .send()?
    .json()?;

    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .map(|entry| {
            entry
                .name_value
                .split("\n")
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .filter(|subdomain: &String| subdomain != target)
        .filter(|subdomain: &String| !subdomain.contains("*") )
        .collect();

        subdomains.insert(target.to_string());

        let subdomains: Vec<Subdomain> = subdomains
            .into_iter()
            .map(|domain| Subdomain {
                domain,
                open_ports: Vec::new()
            }).collect();

            Ok(subdomains)

}
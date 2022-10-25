use std::{env, io};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use reqwest::{Client, Url};

fn read_urls_from_file(path: &String, queue: &mut HashSet<Url>) {
    match read_lines(&path) {
        Ok(lines) => {
            for line in lines {
                if let Ok(url) = line {
                    if let Ok(uri) = Url::parse(&url) {
                        queue.insert(uri);
                    }
                }
            }
        }
        
        Err(err) => {
            println!("error: {}", err);
        }
    }
}

fn read_lines(filename: &String) -> io::Result<io::Lines<io::BufReader<File>>>{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn perform_crawl(http_client: &Client, queue: &HashSet<Url>) {
    for url in queue {
        match get_status_code(&http_client, url.clone()).await {
            Ok(status_code) => println!("{}\t{}", url, status_code),
            Err(err) => println!("{}\t{}", url, err),
        }
    }
}

async fn get_status_code(http_client: &Client, uri: Url) -> Result<u16, &'static str> {
    match http_client.head(uri).send().await {
        Ok(resp) => Ok(resp.status().as_u16()),
        Err(_) => Err("error: http request failed")
    }
}

#[tokio::main]
async fn main() {
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("error: no arg specified");
            return
        }
    };
    
    let http_client = Client::new();
    let mut queue = HashSet::new();

    // enqueue urls from file
    read_urls_from_file(&path, &mut queue);

    // do the crawly crawly
    perform_crawl(&http_client, &queue).await
}
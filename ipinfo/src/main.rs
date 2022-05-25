use ipinfo::{IpInfo, IpInfoConfig};
use tokio;

#[tokio::main]
async fn main() {

  if let Some(ip) = public_ip::addr().await {
    let ipfor = format!("{:?}",ip);

    let config = IpInfoConfig { token: Some("token".to_string()), ..Default::default() };
    let mut ipinfo = IpInfo::new(config).expect("should construct");
    let res = ipinfo.lookup(&[&ipfor]);

    match res {
      Ok(r) => println!("{} {} {} {}", ipfor, r[&ipfor].city, r[&ipfor].region, r[&ipfor].country),
      Err(e) => println!("error occurred: {}", &e.to_string()),
    }

  } else {
      println!("couldn't get an IP address");
  }

}

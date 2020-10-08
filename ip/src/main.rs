extern crate reqwest;
extern crate lettre;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


fn main() {
    getip();
}

#[tokio::main]
async fn getip() -> Result<(), reqwest::Error> { //fn to read web api and get IP address.
    let client = reqwest::Client::new();
    let res = client.get("http://api.ipify.org/").send().await?;

    // Move and borrow value of `res`
    let ip = res.text().await?;
    println!("Your IP is:\n{}", ip);

    mail_ip(&ip);

    Ok(())
}

fn mail_ip(ip: &String) { //the function that actually sends the emails. 
    let email = Message::builder()
        .from("rust.imailip@gmail.com".parse().unwrap())
        .to("lanson.noah@gmail.com".parse().unwrap())
        .subject("Here is your IP")
        .body(ip)
        .unwrap();

    let creds = Credentials::new("<GMAIL ADDRESS HERE>".to_string(), "<GMAIL PASSWORD HERE>".to_string()); //input your gmail user and pass in the provided area.

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
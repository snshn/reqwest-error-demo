use reqwest::blocking::Client;

fn main() {
    let client = Client::builder()
        .build()
        .expect("Failed to initialize HTTP client");
    let url = "https://knowablemagazine.org/pb/css/t1639477624000-v1639477624000/default.css";
    match client.get(url).send() {
        Ok(mut response) => {
            let mut data: Vec<u8> = vec![];
            response.copy_to(&mut data).unwrap();
        }
        Err(_error) => {}
    }
}

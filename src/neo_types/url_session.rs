use reqwest::{Client, Request};

// Create a URLSession struct to manage HTTP requests
pub struct URLSession;

impl URLSession {
	// Make an async method to send a request and return the response body bytes
	pub async fn data(&self, request: Request) -> Result<Vec<u8>, reqwest::Error> {
		// Create a reqwest client
		let client = Client::new();

		// Send the request and await the response
		let response = client.execute(request).await.unwrap();

		// Get the response bytes
		let data = response.bytes().await.unwrap().to_vec();

		// Return the data or any errors
		Ok(data)
	}
}

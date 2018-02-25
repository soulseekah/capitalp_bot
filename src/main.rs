extern crate reqwest;
extern crate serde_json;

use serde_json::Value;
use std::time::Duration;

fn main() {
	let botkey = "botKEYHERE";
	let apiurl = "https://api.telegram.org";

	let mut offset = String::from("575719262");

	let client = reqwest::Client::builder()
			.timeout(Duration::from_secs(90))
			.build().unwrap();
	
	loop {
		let action = ["getUpdates?timeout=90&", ["offset=", offset.as_str()].join("").as_str()].join("");
		let uri = [apiurl, botkey, action.as_str()].join("/");

		println!("Requesting {:?}", uri);

		let update: Value = client.get(uri.as_str())
			.send().unwrap()
			.json().unwrap();

		for result in update["result"].as_array().unwrap() {
			offset = String::from((result["update_id"].as_u64().unwrap() + 1).to_string());
			if result["message"]["text"].is_string() {
				let text = result["message"]["text"].as_str().unwrap();

				if text.to_lowercase().contains("wordpress")
					&& ! text.to_lowercase().contains("wordpress.com")
					&& ! text.to_lowercase().contains("wordpress.org")
					&& ! text.contains("WordPress") {
						
						println!("Received: {:?}", result["message"]["text"]);
						println!("{:?}", result["message"]["chat"]["id"].as_i64());

						// capital_P_dangit()
						let action = ["sendMessage?chat_id=", result["message"]["chat"]["id"].as_i64().unwrap().to_string().as_str()].join("");
						let action = [action.as_str(), "&reply_to_message_id=", result["message"]["message_id"].as_u64().unwrap().to_string().as_str()].join("");
						let action = [action.as_str(), "&parse_mode=Markdown"].join("");
						let uri = [apiurl, botkey, action.as_str()].join("/");

						println!("{:?}", uri);

						let client = reqwest::Client::new();
						client.post(uri.as_str())
							.form(&[("text", "*WordPress*. Ни Wordpress, ни wordpress, ни w0rdPrеss!")])
							.send().unwrap();
				}
			}
		}
	}
}

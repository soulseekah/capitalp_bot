extern crate reqwest;
extern crate serde_json;
extern crate linkify;

use serde_json::Value;
use std::time::Duration;
use linkify::LinkFinder;

fn main() {
	let botkey = "botKEYHERE";
	let apiurl = "https://api.telegram.org";

	let mut offset = String::from("575723650");

	let client = reqwest::Client::builder()
			.timeout(Duration::from_secs(90))
			.build().unwrap();
	
	loop {
		let action = ["getUpdates?timeout=90&", ["offset=", offset.as_str()].join("").as_str()].join("");
		let uri = [apiurl, botkey, action.as_str()].join("/");

		println!("Requesting {:?}", uri);

		let mut response = match client.get(uri.as_str()).send() {
			Ok(response) => response,
			Err(e) => {
				println!("Error {:?}", e);
				continue
			}
		};

		let update: Value = match response.json() {
			Ok(update) => update,
			Err(e) => {
				println!("Error {:?}", e);
				continue
			}
		};

		for result in update["result"].as_array().unwrap() {
			offset = String::from((result["update_id"].as_u64().unwrap() + 1).to_string());
			if result["message"]["text"].is_string() {
				let text = result["message"]["text"].as_str().unwrap();

				if capital_p_dangit(text) {
						println!("Received: {:?}", result["message"]["text"]);
						println!("{:?}", result["message"]["chat"]["id"].as_i64());

						// capital_P_dangit()
						let action = ["sendMessage?chat_id=", result["message"]["chat"]["id"].as_i64().unwrap().to_string().as_str()].join("");
						let action = [action.as_str(), "&reply_to_message_id=", result["message"]["message_id"].as_u64().unwrap().to_string().as_str()].join("");
						let action = [action.as_str(), "&parse_mode=Markdown"].join("");
						let uri = [apiurl, botkey, action.as_str()].join("/");

						println!("{:?}", uri);

						client.post(uri.as_str())
							.form(&[("text", "*WordPress*. Ни Wordpress, ни wordpress, ни вротпресс!")])
							.send();
				}
			}
		}
	}
}

fn capital_p_dangit(text: &str) -> bool {
	// Strip any links we have and call again
	let finder = LinkFinder::new();

	let links: Vec<_> = finder.links(text).collect();
	if links.len() > 0 {
		let mut _text = String::from(text);
		for link in links {
			_text = _text.replace(link.as_str(), "");
		}
		return capital_p_dangit(_text.as_str());
	}

	if text.to_lowercase().contains("wordpress.org") || text.to_lowercase().contains("wordpress.com") {
		return capital_p_dangit(text.to_lowercase().replace("wordpress.org", "").replace("wordpress.com", "").as_str());
	}

	if text.to_lowercase().contains("wordpress.tv") {
		return capital_p_dangit(text.to_lowercase().replace("wordpress.tv", "").as_str());
	}

	text.to_lowercase().contains("wordpress") && ! text.contains("WordPress")
}

#[test]
fn test_capital_p_dangit() {
	assert!(true == capital_p_dangit("wordpress"));
	assert!(true == capital_p_dangit("Wordpress"));
	assert!(true == capital_p_dangit("WORDPRESS"));
	assert!(false == capital_p_dangit("WordPress"));
	assert!(false == capital_p_dangit("wordpress WordPress"));
	assert!(false == capital_p_dangit("wordpress.org"));
	assert!(false == capital_p_dangit("come to wordpress.tv"));
	assert!(false == capital_p_dangit("wordpress.com"));
	assert!(false == capital_p_dangit("check out https://someothersite.com/wordpress/hello"));
	assert!(true == capital_p_dangit("check out https://wordpress.org Wordpress"));
}
						

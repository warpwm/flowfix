use std::env;
use std::io::Read;
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use wl_clipboard_rs::{paste::{get_contents, ClipboardType, Error, MimeType, Seat}};


#[tokio::main]
async fn main() {
    let ip_env = env::var("OLLAMA_HOST").unwrap_or("localhost:11434".to_string());
    
    let parts: Vec<&str> = ip_env.split(':').collect();
    let ip = "http://".to_owned() + parts[0];
    let port = parts[1].parse::<u16>().unwrap();
    
    let ollama = Ollama::new(ip.to_string(), port);
    
    let model = "mistral:instruct".to_string();
    
    let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);
    let stringy = match result {
        Ok((mut pipe, _)) => {
            let mut contents = vec![];
            pipe.read_to_end(&mut contents).unwrap();
            String::from_utf8_lossy(&contents).to_string()
        }
        Err(Error::NoSeats) | Err(Error::ClipboardEmpty) | Err(Error::NoMimeType) => {"".to_string()}
        Err(err) => Err(err).unwrap()
    };
    
    let prompt = "Spell correct this text, but preserver all new line characters:\n".to_owned()
    + &stringy + "\n Return only the corrected text, don't include a preamble.";

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    if let Ok(res) = res {
        println!("{}", res.response);
    }
}
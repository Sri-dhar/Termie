// use ask_gemini::Gemini;
// use std::env;
// use std::fs;
// use std::io::{self, Error};

// //default one
// #[cfg(not(feature = "donot_fetch_api_key_from_system"))]
// pub fn fetch_api_key() -> Result<String, Error> {
//     match env::var("GEMINI_API_KEY") {
//         Ok(key) => Ok(key),
//         Err(e) => {
//             eprintln!("Error: GEMINI_API_KEY not set - {}", e);
//             Err(io::Error::new(
//                 io::ErrorKind::NotFound,
//                 "GEMINI_API_KEY not set",
//             ))
//         }
//     }
// }

// #[cfg(feature = "donot_fetch_api_key_from_system")]
// pub fn fetch_api_key() -> Result<String, Error> {
//     Ok(String::from(""))
// }

// pub async fn call_gemini(prompt_content: String) -> Result<String, Error> {
//     let api_key = fetch_api_key()?;
//     let api_key_ref: Option<&str> = Some(api_key.as_str());

//     let gemini = Gemini::new(api_key_ref, None);
//     let prompt_prefix = fs::read_to_string(
//     "./prompt_context/context1.txt",
//     )?
//     .trim()
//     .to_string();
//     let prompt = format!("{} {}", prompt_prefix, prompt_content);

//     // Anonymous function to process the input
//     let process = |input: Vec<String>| -> String {
//         let mut output = String::new();
//         for i in input {
//             output.push_str(i.as_str());
//         }
//         output
//     };

//     match gemini.ask(prompt.as_str()).await {
//         Ok(response) => Ok(process(response)),
//         Err(_e) => Err(io::Error::new(
//             io::ErrorKind::InvalidData,
//             "Response could not be fetched",
//         )),
//     }
// }


use ask_gemini::Gemini;
use std::env;
use std::fs;
use std::io::{self, Error};

#[cfg(not(feature = "donot_fetch_api_key_from_system"))]
pub fn fetch_api_key() -> Result<String, Error> {
    match env::var("GEMINI_API_KEY") {
        Ok(key) => Ok(key),
        Err(e) => {
            eprintln!("Error: GEMINI_API_KEY not set - {}", e);
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "GEMINI_API_KEY not set",
            ))
        }
    }
}

#[cfg(feature = "donot_fetch_api_key_from_system")]
pub fn fetch_api_key() -> Result<String, Error> {
    fs::read_to_string("~/.env").map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))
}

pub async fn call_gemini(prompt_content: String) -> Result<String, Error> {
    let api_key = fetch_api_key()?;
    let api_key_ref: Option<&str> = Some(api_key.as_str());

    let gemini = Gemini::new(api_key_ref, None);
    let prompt_prefix = fs::read_to_string(
        "/home/solomons/Rust_AttemptG/folder_geminiInRust/gui-terminal/prompt_context/context1.txt",
    )?
    .trim()
    .to_string();
    let prompt = format!("{} {}", prompt_prefix, prompt_content);

    // Anonymous function to process the input
    let process = |input: Vec<String>| -> String {
        let mut output = String::new();
        for i in input {
            output.push_str(i.as_str());
        }
        output
    };

    match gemini.ask(prompt.as_str()).await {
        Ok(response) => Ok(process(response)),
        Err(_e) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Response could not be fetched",
        )),
    }
}

use serde::{Deserialize, Serialize};
use anyhow::{Context};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    source: String,
    #[serde(rename = "dest")]
    destination: String,
    body: Body,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
  Echo {echo: String}
  
}


fn main() -> anyhow::Result<()>{
  let stdin = std::io::stdin().lock();  
  let mut stdout = std::io::stdout().lock();

  let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

  for input in inputs {
    let input = input.context("Failed to read input from STDIN")?;
    let output = Message {
      source: input.destination,
      destination: input.source,
      body: Body {
        id: input.body.id,
        in_reply_to: input.body.id,
        payload: Payload::Echo { echo: "Hello, World!".to_string() },
      },
    };
    serde_json::to_writer(&mut stdout, &output)?;
    println!();
  }

Ok(())
}

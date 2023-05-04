use anyhow;
use rust_bert::gpt2::GPT2Generator;
use rust_bert::pipelines::generation_utils::LanguageGenerator;
use std::io;

fn main() -> anyhow::Result<()> {

    let model = GPT2Generator::new(Default::default())?;

    println!("Enter some text for the model to generate from:");
    let mut input_context = String::new();
 
    io::stdin().read_line(&mut input_context).expect("failed to readline");

    let output = model.generate(Some(vec!(input_context).as_ref()), None);

    println!("{:?}", output);

    Ok(())
}

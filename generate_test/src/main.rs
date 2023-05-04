use anyhow::{anyhow, Result};
use rust_bert::gpt2::GPT2Generator;
use rust_bert::pipelines::generation_utils::LanguageGenerator;

fn main() -> Result<()> {

    let model = GPT2Generator::new(Default::default())?;
                                                        
    let input_context_1 = "The dog";

    let output = model.generate(Some(vec!(input_context_1).as_ref()), None);

    println!("{:?}", output);

    Ok(())
}

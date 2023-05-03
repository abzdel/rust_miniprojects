use anyhow::{anyhow, Result};
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};

fn main() -> Result<()> {
    let qa_model = QuestionAnsweringModel::new(Default::default())
        .map_err(|err| anyhow!("Failed to load model: {}", err))?;

    let question = String::from("Where does Amy live ?");
    let context = String::from("Amy lives in Amsterdam");

    let answers = qa_model.predict(&[QaInput { question, context }], 1, 32);

    println!("{:?}", answers);

    Ok(())
}

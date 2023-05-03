use anyhow::{anyhow, Result};
use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};

fn main() -> Result<()> {
    let qa_model = QuestionAnsweringModel::new(Default::default())
        .map_err(|err| anyhow!("Failed to load model: {}", err))?;

    let question = String::from("Where does Amy live ?");
    let context = String::from("Amy lives in Amsterdam");

    let answers = qa_model.predict(&[QaInput { question, context }], 1, 32);

    for answer_set in answers {
        for answer in answer_set {
            println!("{}", answer.answer);
            println!("Score: {:?}", answer.score);
            println!("Start: {:?}", answer.start);
            println!("End: {:?}", answer.end);
        }
    }

    Ok(())
}

use automl;
use smartcore;

fn main() {
    let dataset = smartcore::dataset::iris::load_dataset();
    let settings = automl::Settings::default_classification();
    let mut classifier = automl::SupervisedModel::new(dataset, settings);
    classifier.train();
    print!("{}", classifier);
}
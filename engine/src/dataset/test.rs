use crate::dataset::{Dataset, FreetextQuestion, Question, QuestionType};

#[test]
fn test_tagalog() {
    // Open tagalog.json and process with serdesjson to a Dataset struct
    let file = std::fs::File::open("../datasets/tagalog.json").unwrap();
    let reader = std::io::BufReader::new(file);
    let _dataset: Dataset = serde_json::from_reader(reader).unwrap();
}

#[test]
fn test_render() {
    let dataset = Dataset {
        id: "".to_string(),
        name: "".to_string(),
        author: "".to_string(),
        description: "".to_string(),
        version: "".to_string(),
        questions: vec![Question {
            id: "".to_string(),
            question: QuestionType::Freetext(Box::new(FreetextQuestion {
                question_prompt: "".to_string(),
                answers: vec![],
                tolerance: 0.0,
            })),
        }],
    };
    let json = serde_json::to_string_pretty(&dataset).unwrap();
    panic!("{}", json)
}

#[test]
fn test_jaccard() {
    let d = text_distance::DamerauLevenshtein {
        src: "some answer".to_string(),
        tar: "some answers".to_string(),
        restricted: false,
    }
    .normalized_distance();
    assert_eq!(
        // stringmetrics::jaccard("some answer".chars(), "some answers".chars()),
        1.0 - d,
        0.9166666666666666
    );
}

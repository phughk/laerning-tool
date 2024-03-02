use crate::xml::{
    Answer, CategoryDeclaration, EntryTag, LearningModuleEntries, LearningModuleEntry,
    LearningModuleEntryType, Question,
};
use serde::Deserialize;
use serde_xml_rs::Deserializer;
use xml::EventReader;

#[test]
fn result_des_test() {
    let input = r#"<answer category="category name" correct="true" id="abc-123">This is the expected answer</answer>"#;
    let actual = Answer::deserialize(&mut Deserializer::new(EventReader::new(input.as_bytes())));
    let expected = Answer {
        category: "category name".to_string(),
        correct: true,
        id: "abc-123".to_string(),
        label: "This is the expected answer".to_string(),
    };
    match actual {
        Ok(act) => {
            assert_eq!(act, expected)
        }
        Err(e) => {
            panic!("failed to deserialise: {}", e)
        }
    }
}

#[test]
fn category_des_test() {
    let input = "<category-declaration id=\"abc-123\">category text</category-declaration>";
    let actual = CategoryDeclaration::deserialize(&mut Deserializer::new(EventReader::new(
        input.as_bytes(),
    )));
    let expected = CategoryDeclaration {
        id: "abc-123".to_string(),
        label: "category text".to_string(),
    };
    match actual {
        Ok(act) => {
            assert_eq!(act, expected)
        }
        Err(e) => {
            panic!("failed to deserialise: {}", e)
        }
    }
}

#[test]
fn entry_des_test() {
    let input = r#"
    <entry id="abc-123" type="single-choice" sampleable="true">
      <question>the question</question>
      <answer correct="true" id="a1">An answer</answer>
    </entry>
    "#;
    let actual = LearningModuleEntry::deserialize(&mut Deserializer::new(EventReader::new(
        input.as_bytes(),
    )));
    let expected = LearningModuleEntry {
        entry_type: LearningModuleEntryType::SingleChoice,
        id: "abc-123".to_string(),
        sampleable: true,
        entry_tags: vec![
            EntryTag::Question(Question {
                label: "the question".to_string(),
            }),
            EntryTag::Answer(Answer {
                correct: true,
                id: "a1".to_string(),
                category: "".to_string(),
                label: "An answer".to_string(),
            }),
        ],
    };
    match actual {
        Ok(act) => {
            assert_eq!(act, expected)
        }
        Err(e) => {
            panic!("failed to deserialise: {}", e)
        }
    }
}

#[test]
fn entries_des_test() {
    let input = r#"
    <entries>
      <entry id="abc-123" type="single-choice" sampleable="true">
        <question>the question</question>
        <answer correct="true" id="a1">An answer</answer>
      </entry>
    </entries>
    "#;
    let actual = LearningModuleEntries::deserialize(&mut Deserializer::new(EventReader::new(
        input.as_bytes(),
    )));
    let expected = LearningModuleEntries {
        entries: vec![LearningModuleEntry {
            entry_type: LearningModuleEntryType::SingleChoice,
            id: "abc-123".to_string(),
            sampleable: true,
            entry_tags: vec![
                EntryTag::Question(Question {
                    label: "the question".to_string(),
                }),
                EntryTag::Answer(Answer {
                    correct: true,
                    id: "a1".to_string(),
                    category: "".to_string(),
                    label: "An answer".to_string(),
                }),
            ],
        }],
    };
    match actual {
        Ok(act) => {
            assert_eq!(act, expected)
        }
        Err(e) => {
            panic!("failed to deserialise: {}", e)
        }
    }
}

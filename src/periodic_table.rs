use rand::Rng;
use std::fs;
use std::io;

use crate::element::Element;

pub struct PeriodicTable {
    elements: Vec<Element>
}

impl PeriodicTable {
    pub fn init(file_path: String) -> Self {
        let content = fs::read_to_string(file_path).unwrap();
        let mut elements: Vec<Element> = Vec::new();

        let mut index: usize = 0;
        for line in content.lines() {
            let mut element_info: [String; 6] = [String::new(), String::new(), 
                                            String::new(), String::new(), 
                                            String::new(), String::new()];
            index = 0;
            for word in line.split_whitespace() {
                element_info[index] = word.to_string();
                index = index + 1;
            }

            elements.push(Element::new(element_info[0].clone(), element_info[1].clone(),
                                        element_info[2].clone(), element_info[3].clone(),
                                        element_info[4].clone(),
                                        (element_info[5]=="natural".to_string())));
        }

        Self {
            elements: elements
        }
    }

    pub fn quiz(&self, num_questions: u8) -> u8 {
        let mut rng = rand::rng();
        let mut questions: Vec<(String, String)> = Vec::new();
        let mut score: u8 = 0;

        while questions.clone().len() < (num_questions as usize) {
            let atomic_number: u8 = rng.random_range(0..=92);
            let element = self.element_by_atomic_number(atomic_number);
            if self.natural_element(element.clone()) {
                let mut pick_another = false;
                for question_element in questions.clone() {
                    if question_element.0 == element.symbol {
                        pick_another = true;
                    }
                }
                if !pick_another {
                    questions.push((element.symbol, element.name));
                }
            }
        }

        println!("|||||||||||Begin quiz|||||||||||||");
        for element in questions.clone() {
            println!("-What is the name of element: {}", element.0);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line.");
            let name = input.trim();
            if name == element.1 {
                score = score + 1;
            }
        }
        println!("||||||||||||||||||||||||||||||||||");
        println!("");
        println!("");
        println!("-----------------------");
        println!("Correct answers:");
        for element in questions {
            println!("{} -- {}", element.0, element.1);
        }
        println!("-----------------------");

        score
    }

    pub fn natural_element(&self, element: Element) -> bool {
        element.natural
    }

    pub fn element_by_atomic_number(&self, atomic_number: u8) -> Element {
        for element in self.elements.clone() {
            if element.atomic_number == atomic_number.to_string() {
                return element;
            }
        }
        (self.elements[0]).clone()
    }

    pub fn total_elements(self) -> usize {
        self.elements.len()
    }
}

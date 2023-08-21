extern crate turtle_graphics;

use turtle_graphics::{Canvas, Turtle};
use std::fs::File;
use std::io::BufReader;
use std::fmt;
use std::io::prelude::*;
use rand::Rng;

impl fmt::Debug for LSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LSystem {{ axiom: {:?}, rule1: {:?}, rule2: {:?}, rule3: {:?}, rule4: {:?}, rule5: {:?},iterations: {:?}, angle : {:?}, is_random_angle:{:?} }}",
            self.axiom, self.rule1, self.rule2, self.rule3, self.rule4, self.rule5, self.iterations, self.angle, self.is_random_angle
        )
    }
}

struct RulePair {
    rule1: String,
    rule2: String,
}
struct LSystem {
    axiom: String,
    rule1: [String; 2],
    rule2: [String; 2],
    rule3: [String; 2],
    rule4: [String; 2],
    rule5: [String; 2],
    iterations: u32,
    angle: i32,
    is_random_angle: bool,
}

fn parse_rules(s: String) -> [String; 2] {
    let vec: Vec<String> = s.split("->").map(String::from).collect();
    if vec.len() >= 2 {
        [vec[0].clone(), vec[1].clone()] // Преобразование в массив
    } else {
        // Здесь можно обработать случай, когда элементов меньше, чем 2
        [String::new(), String::new()]
    }
}
fn get_text() -> String {
    let file = File::open("config.txt").expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let _ = buf_reader.read_to_string(&mut contents);
    let binding = contents.replace(" ", "");
    let contents = binding.trim_matches('"');
    contents.to_string()
}
fn main() {
    let contents = get_text();
    let empty_string = String::new();
    let empty_rule: [String; 2] = [empty_string.clone(), empty_string.clone()];

    let mut l_system = LSystem {
        axiom: String::new(),
        rule1: empty_rule.clone(),
        rule2: empty_rule.clone(),
        rule3: empty_rule.clone(),
        rule4: empty_rule.clone(),
        rule5: empty_rule.clone(),
        iterations: 0,
        angle: 0,
        is_random_angle: false,
    };

    for line in contents.lines(){
        let splitted: Vec<&str> = line.split('=').collect();
        let key = splitted[0];
        let value: String = splitted[1].chars().filter(|&c| c != '\'' && c != '\\').collect();
        match key {
            "axiom" => l_system.axiom = value.to_string(),
            "rule1" => l_system.rule1 = parse_rules(value),
            "rule2" => l_system.rule2 = parse_rules(value),
            "rule3" => l_system.rule3 = parse_rules(value),
            "rule4" => l_system.rule4 = parse_rules(value),
            "rule5" => l_system.rule5 = parse_rules(value),
            "iterations" => l_system.iterations = value.parse().expect("Invalid value format"),
            "angle" => l_system.angle = value.parse().expect("Invalid value format"),
            "is_random_angle" => l_system.is_random_angle = value.parse().expect("Invalid value format"),
            _ => {}
        }
    };
    let rules = [
        RulePair {
            rule1: l_system.rule1[0].clone().to_string().trim_matches('"').to_string(),
            rule2: l_system.rule1[1].clone().to_string().trim_matches('"').to_string(),
        },
        RulePair {
            rule1: l_system.rule2[0].clone().to_string().trim_matches('"').to_string(),
            rule2: l_system.rule2[1].clone().to_string().trim_matches('"').to_string(),
        },
        RulePair {
            rule1: l_system.rule3[0].clone().to_string().trim_matches('"').to_string(),
            rule2: l_system.rule3[1].clone().to_string().trim_matches('"').to_string(),
        },
        RulePair {
            rule1: l_system.rule4[0].clone().to_string().trim_matches('"').to_string(),
            rule2: l_system.rule4[1].clone().to_string().trim_matches('"').to_string(),
        },
        RulePair {
            rule1: l_system.rule5[0].clone().to_string().trim_matches('"').to_string(),
            rule2: l_system.rule5[1].clone().to_string().trim_matches('"').to_string(),
        },

    ];
    let axiom =l_system.axiom.as_str();
    let n= l_system.iterations.clone();
    let ang = l_system.angle.clone();
    let result = generate_system(axiom, &rules, n);
    let r_a = l_system.is_random_angle.clone();
    println!("n = {} : {}", n, result);
    draw_l_s(result, 0.0, ang, 10.0, "Pentadendrite", r_a);
}
fn draw_l_s(
    l_s_s: String,
    init_direction: f32,
    default_angle: i32,
    default_distance: f32,
    filename: &str,
    is_r: bool,
) {
    let mut t = Canvas::new();
    t.right(init_direction);
    for sym in l_s_s.chars() {
        match sym {
        'F' => {
            t.forward(default_distance);
        }
        'G' => {
            t.forward(default_distance);
        }
        'A' => {
            t.forward(default_distance);
        }
        'B' => {
            t.forward(default_distance);
        }
        'C' => {
            t.forward(default_distance);
        }
        'D' => {
            t.forward(default_distance);
        }
        'E' => {
            t.forward(default_distance);
        }
        'f' => {
            t.move_forward(default_distance);
        }
        'g' => {
            t.move_forward(default_distance);
        }
        'a' => {
            t.move_forward(default_distance);
        }
        'b' => {
            t.move_forward(default_distance);
        }
        'c' => {
            t.move_forward(default_distance);
        }
        'd' => {
            t.move_forward(default_distance);
        }
        'e' => {
            t.move_forward(default_distance);
        }
        '+' => {
            t.rotate(-gg_r(default_angle,is_r));
        }
        '-' => {
            t.rotate(gg_r(default_angle,is_r));
        }
        '[' => {
            t.push();
        }
        ']' => {
            t.pop();
        }
        _ => {}
    }
    }
    t.save_svg(&mut File::create(filename.to_string() + ".svg").unwrap())
        .unwrap();
    t.save_eps(&mut File::create(filename.to_string() + ".eps").unwrap())
        .unwrap();
}
fn gg_r(ag: i32, b: bool) -> f32 {
    if b {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..ag);
        random_number as f32
    }
    else { ag as f32 }

}
fn generate_system(axiom: &str, rules: &[RulePair], n: u32) -> String {
    let mut result = axiom.to_string();

    for _ in 0..n {
        let mut new_result = String::new();
        for c in result.chars() {
            if let Some(rule) = rules.iter().find(|rule| rule.rule1 == c.to_string()) {
                new_result.push_str(&rule.rule2);
            } else {
                new_result.push(c);
            }
        }
        result = new_result;
    }

    result
}
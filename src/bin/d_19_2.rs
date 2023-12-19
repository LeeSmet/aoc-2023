use std::{collections::HashMap, ops::RangeInclusive};

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_19_1.txt");

fn main() {
    let (workflow_lines, _) = INPUT_FILE.split_once("\n\n").unwrap();

    let workflows = workflow_lines
        .lines()
        .map(Workflow::from_line)
        .map(|w| (w.name.to_string(), w))
        .collect::<HashMap<_, _>>();

    let mut to_explore = vec![("in", Part::new())];
    let mut accepted = vec![];

    while let Some((wf, mut part)) = to_explore.pop() {
        let workflow = workflows.get(wf).unwrap();
        for rule in &workflow.rules {
            match rule.apply(&mut part) {
                None => {}
                Some((Operation::Reject, anti)) => {
                    part = anti;
                }
                Some((Operation::Accept, anti)) => {
                    accepted.push(part);
                    part = anti;
                }
                Some((Operation::Redirect(w), anti)) => {
                    to_explore.push((w, part));
                    part = anti;
                }
            };
        }
        match workflow.fallback {
            Operation::Reject => {}
            Operation::Accept => accepted.push(part),
            Operation::Redirect(w) => to_explore.push((w, part)),
        }
    }

    let total_possible_parts = accepted.into_iter().map(|part| part.allowed()).sum::<u64>();

    println!("Total accepted part configurations: {total_possible_parts}");
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    fallback: Operation<'a>,
}

impl<'a> Workflow<'a> {
    fn from_line(line: &'a str) -> Self {
        let (name, rules) = line[..line.len() - 1].split_once('{').unwrap();
        let rule_desc = rules.split(',').collect::<Vec<_>>();
        let rules = rule_desc[..rule_desc.len() - 1]
            .iter()
            .map(|rd| Rule::from_line(rd))
            .collect();
        let fallback = match *rule_desc.last().unwrap() {
            "A" => Operation::Accept,
            "R" => Operation::Reject,
            x => Operation::Redirect(x),
        };

        Workflow {
            name,
            rules,
            fallback,
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    field: &'a str,
    greater: bool,
    value: usize,
    action: Operation<'a>,
}

impl<'a> Rule<'a> {
    fn from_line(line: &'a str) -> Self {
        let field = &line[..1];
        let greater = &line[1..2] == ">";
        let colon_idx = line.chars().position(|c| c == ':').unwrap();
        let value = line[2..colon_idx].parse().unwrap();

        let action = match &line[colon_idx + 1..] {
            "A" => Operation::Accept,
            "R" => Operation::Reject,
            x => Operation::Redirect(x),
        };

        Rule {
            field,
            greater,
            value,
            action,
        }
    }

    fn apply(&self, part: &mut Part) -> Option<(&Operation<'a>, Part)> {
        let mut anti = part.clone();
        match (self.field, self.greater) {
            ("x", true) => {
                anti.x =
                    part.x.clone().min().unwrap()..=part.x.clone().max().unwrap().min(self.value);
                part.x = part.x.clone().min().unwrap().max(self.value + 1)
                    ..=part.x.clone().max().unwrap();
                if part.x.is_empty() {
                    None
                } else {
                    Some((&self.action, anti))
                }
            }
            ("m", true) => {
                anti.m =
                    part.m.clone().min().unwrap()..=part.m.clone().max().unwrap().min(self.value);
                part.m = part.m.clone().min().unwrap().max(self.value + 1)
                    ..=part.m.clone().max().unwrap();
                if part.m.is_empty() {
                    None
                } else {
                    Some((&self.action, anti))
                }
            }
            ("a", true) => {
                anti.a =
                    part.a.clone().min().unwrap()..=part.a.clone().max().unwrap().min(self.value);
                part.a = part.a.clone().min().unwrap().max(self.value + 1)
                    ..=part.a.clone().max().unwrap();
                if part.a.is_empty() {
                    None
                } else {
                    Some((&self.action, anti))
                }
            }
            ("s", true) => {
                anti.s =
                    part.s.clone().min().unwrap()..=part.s.clone().max().unwrap().min(self.value);
                part.s = part.s.clone().min().unwrap().max(self.value + 1)
                    ..=part.s.clone().max().unwrap();
                if part.s.is_empty() {
                    None
                } else {
                    Some((&self.action, anti))
                }
            }
            ("x", false) => {
                anti.x =
                    part.x.clone().min().unwrap().max(self.value)..=part.x.clone().max().unwrap();
                part.x = part.x.clone().min().unwrap()
                    ..=part.x.clone().max().unwrap().min(self.value - 1);
                if part.x.is_empty() {
                    None
                } else {
                    Some((&self.action, anti))
                }
            }
            ("m", false) => {
                anti.m =
                    part.m.clone().min().unwrap().max(self.value)..=part.m.clone().max().unwrap();
                part.m = part.m.clone().min().unwrap()
                    ..=part.m.clone().max().unwrap().min(self.value - 1);
                if part.m.is_empty() {
                    None
                } else {
                    Some((&self.action, anti))
                }
            }
            ("a", false) => {
                anti.a =
                    part.a.clone().min().unwrap().max(self.value)..=part.a.clone().max().unwrap();
                part.a = part.a.clone().min().unwrap()
                    ..=part.a.clone().max().unwrap().min(self.value - 1);
                if part.a.is_empty() {
                    None
                } else {
                    Some((&self.action, anti))
                }
            }
            ("s", false) => {
                anti.s =
                    part.s.clone().min().unwrap().max(self.value)..=part.s.clone().max().unwrap();
                part.s = part.s.clone().min().unwrap()
                    ..=part.s.clone().max().unwrap().min(self.value - 1);
                if part.s.is_empty() {
                    None
                } else {
                    Some((&self.action, anti))
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Operation<'a> {
    Accept,
    Reject,
    Redirect(&'a str),
}

#[derive(Debug, Clone)]
struct Part {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl Part {
    fn new() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }

    fn allowed(&self) -> u64 {
        self.x.clone().count() as u64
            * self.m.clone().count() as u64
            * self.a.clone().count() as u64
            * self.s.clone().count() as u64
    }
}

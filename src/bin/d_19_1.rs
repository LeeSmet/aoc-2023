use std::collections::HashMap;

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_19_1.txt");

fn main() {
    let (workflow_lines, part_lines) = INPUT_FILE.split_once("\n\n").unwrap();

    let workflows = workflow_lines
        .lines()
        .map(Workflow::from_line)
        .map(|w| (w.name.to_string(), w))
        .collect::<HashMap<_, _>>();
    let accepted_parts_sum = part_lines
        .lines()
        .map(Part::from_line)
        .filter(|part| part_accepted(&workflows, part))
        .map(|part| part.rating())
        .sum::<usize>();

    println!("Sum of accepted part ratings: {accepted_parts_sum}");
}

fn part_accepted(wf: &HashMap<String, Workflow<'_>>, part: &Part) -> bool {
    let mut wf_name = "in";
    loop {
        let workflow = wf.get(wf_name).unwrap();
        match workflow.action(part) {
            Operation::Accept => return true,
            Operation::Reject => return false,
            Operation::Redirect(x) => wf_name = x,
        }
    }
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

    fn action(&'a self, part: &Part) -> &Operation<'a> {
        for rule in &self.rules {
            if let Some(action) = rule.action(part) {
                return action;
            }
        }
        &self.fallback
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

    fn action(&self, part: &Part) -> Option<&Operation> {
        match (self.field, self.greater) {
            ("x", true) if part.x > self.value => Some(&self.action),
            ("m", true) if part.m > self.value => Some(&self.action),
            ("a", true) if part.a > self.value => Some(&self.action),
            ("s", true) if part.s > self.value => Some(&self.action),
            ("x", false) if part.x < self.value => Some(&self.action),
            ("m", false) if part.m < self.value => Some(&self.action),
            ("a", false) if part.a < self.value => Some(&self.action),
            ("s", false) if part.s < self.value => Some(&self.action),
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

#[derive(Debug, Default)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn from_line(line: &str) -> Self {
        let mut part = Part {
            ..Default::default()
        };

        for (l, v) in line[1..line.len() - 1]
            .split(',')
            .map(|p| p.split_once('=').unwrap())
            .map(|(l, v)| (l, v.parse().unwrap()))
        {
            match l {
                "x" => part.x = v,
                "m" => part.m = v,
                "a" => part.a = v,
                "s" => part.s = v,
                _ => unreachable!(),
            }
        }

        part
    }

    fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

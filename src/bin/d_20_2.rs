use std::collections::{HashMap, VecDeque};

/// Input file included in binary to avoid runtime file IO.
const INPUT_FILE: &str = include_str!("../../assets/d_20_1.txt");

fn main() {
    let mut modules = INPUT_FILE
        .lines()
        .map(Module::from_line)
        .collect::<HashMap<_, _>>();

    let conjunctors = modules
        .values()
        .filter(|m| matches!(m.t, ModuleType::Conjunction))
        .map(|m| m.name)
        .collect::<Vec<_>>();

    // Insert conjunctor inputs
    let mut conj_inputs = vec![];
    for module in modules.values() {
        for dst in module.destinations.iter() {
            if conjunctors.contains(dst) {
                conj_inputs.push((*dst, module.name));
            }
        }
    }
    for (conj, src) in conj_inputs.into_iter() {
        modules
            .get_mut(conj)
            .unwrap()
            .inputs
            .insert(src, SignalState::Low);
    }

    let rx_feed = modules
        .values()
        .find(|m| m.destinations.contains(&"rx"))
        .unwrap();

    let mut feed_inputs = modules
        .values()
        .filter(|m| m.destinations.contains(&rx_feed.name))
        .flat_map(|m| m.inputs.keys())
        .map(|input| (*input, None))
        .collect::<HashMap<_, _>>();

    let mut pulses = VecDeque::new();
    let mut button_presses = 0u64;
    'outer: loop {
        button_presses += 1;

        pulses.push_back(Pulse {
            src: "".to_string(),
            value: SignalState::Low,
            dst: "broadcaster".to_string(),
        });

        while let Some(pulse) = pulses.pop_front() {
            if let Some(k) = feed_inputs.get_mut(pulse.src.as_str()) {
                if k.is_none() && matches!(pulse.value, SignalState::Low) {
                    *k = Some(button_presses);
                    if feed_inputs.values().all(Option::is_some) {
                        break 'outer;
                    }
                }
            }
            if let Some(module) = modules.get_mut(pulse.dst.as_str()) {
                let new_pulses = module.handle_pulse(pulse);
                pulses.extend(new_pulses.into_iter());
            }
        }
    }

    let minimum_presses = feed_inputs.into_values().map(Option::unwrap).fold(1, lcm);

    println!("Total button presses to get a low pulse to rx is {minimum_presses}");
}

#[derive(Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Clone)]
struct Pulse {
    src: String,
    value: SignalState,
    dst: String,
}

#[derive(Clone, Copy, Debug)]
enum SignalState {
    High,
    Low,
}

#[derive(Debug)]
struct Module<'a> {
    name: &'a str,
    t: ModuleType,
    destinations: Vec<&'a str>,
    inputs: HashMap<&'a str, SignalState>,
    state: SignalState,
}

impl<'a> Module<'a> {
    fn from_line(line: &'a str) -> (&'a str, Self) {
        let (name, destinations) = line.split_once("->").unwrap();
        let (mtype, name) = match name.trim() {
            n @ "broadcaster" => (ModuleType::Broadcast, n),
            n if &name[..1] == "%" => (ModuleType::FlipFlop, &n[1..]),
            n if &name[..1] == "&" => (ModuleType::Conjunction, &n[1..]),
            _ => unreachable!(),
        };

        let destinations = destinations.trim().split(',').map(str::trim).collect();

        (
            name,
            Self {
                name,
                t: mtype,
                destinations,
                inputs: HashMap::new(),
                state: SignalState::Low,
            },
        )
    }

    fn handle_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let pv = match self.t {
            ModuleType::Broadcast => Some(pulse.value),
            ModuleType::FlipFlop => {
                if matches!(pulse.value, SignalState::Low) {
                    let new_state = match self.state {
                        SignalState::Low => SignalState::High,
                        SignalState::High => SignalState::Low,
                    };
                    self.state = new_state;
                    Some(self.state)
                } else {
                    None
                }
            }
            ModuleType::Conjunction => {
                *self.inputs.get_mut(pulse.src.as_str()).unwrap() = pulse.value;
                Some(
                    if self
                        .inputs
                        .values()
                        .all(|st| matches!(st, SignalState::High))
                    {
                        SignalState::Low
                    } else {
                        SignalState::High
                    },
                )
            }
        };

        if let Some(pv) = pv {
            self.destinations
                .iter()
                .map(|dst| Pulse {
                    src: self.name.to_string(),
                    value: pv,
                    dst: dst.to_string(),
                })
                .collect()
        } else {
            vec![]
        }
    }
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

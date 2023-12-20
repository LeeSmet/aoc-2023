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

    let mut pulses = VecDeque::new();
    let mut lows = 0u64;
    let mut highs = 0;
    for _ in 0..1000 {
        pulses.push_back(Pulse {
            src: "".to_string(),
            value: SignalState::Low,
            dst: "broadcaster".to_string(),
        });

        while let Some(pulse) = pulses.pop_front() {
            match pulse.value {
                SignalState::Low => lows += 1,
                SignalState::High => highs += 1,
            }

            if let Some(module) = modules.get_mut(pulse.dst.as_str()) {
                let new_pulses = module.handle_pulse(pulse);
                pulses.extend(new_pulses.into_iter());
            }
        }
    }

    println!("Total pulse value is {lows} * {highs} = {}", lows * highs);
}

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

#[derive(Clone, Copy)]
enum SignalState {
    High,
    Low,
}

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

use std::collections::HashMap;
use std::str::FromStr;

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

enum PartAccessor { X, M, A, S }

impl FromStr for PartAccessor {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "x" => Ok(PartAccessor::X),
            "m" => Ok(PartAccessor::M),
            "a" => Ok(PartAccessor::A),
            "s" => Ok(PartAccessor::S),
            _ => Err(()),
        }
    }
}

impl Part {
    fn access(&self, accessor: &PartAccessor) -> u64 {
        match accessor {
            PartAccessor::X => self.x,
            PartAccessor::M => self.m,
            PartAccessor::A => self.a,
            PartAccessor::S => self.s,
        }
    }

    fn sum_all(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl Workflow<'_> {
    fn execute(&self, part: &Part) -> &RuleResult {
        for rule in self.rules.iter() {
            match rule.execute_rule(part) {
                Some(result) => return result,
                None => continue,
            }
        }
        &RuleResult::Result(Result::Accept)
    }
}

enum Rule<'a> {
    HigherThan(PartAccessor, u64, RuleResult<'a>),
    LowerThan(PartAccessor, u64, RuleResult<'a>),
    Immediate(RuleResult<'a>),
}

impl Rule<'_> {
    fn execute_rule(&self, part: &Part) -> Option<&RuleResult> {
        match self {
            Rule::HigherThan(part_accessor, value, result) =>
                if part.access(part_accessor) > *value {
                    Some(result)
                } else {
                    None
                },
            Rule::LowerThan(part_accessor, value, result) =>
                if part.access(part_accessor) < *value {
                    Some(result)
                } else {
                    None
                },
            Rule::Immediate(r) => Some(r),
        }
    }
}

enum RuleResult<'a> {
    Result(Result),
    Redirect(&'a str),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Result {
    Accept,
    Reject,
}

#[derive(Clone, Copy)]
struct RangedPart {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl RangedPart {
    fn new(x: Range, m: Range, a: Range, s: Range) -> Self {
        RangedPart { x, m, a, s }
    }

    fn new_all_same(range: Range) -> Self {
        RangedPart::new(range, range, range, range)
    }

    fn multiply_all(&self) -> u64 {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }

    fn access_part(&mut self, accessor: &PartAccessor) -> &mut Range {
        match accessor {
            PartAccessor::X => &mut self.x,
            PartAccessor::M => &mut self.m,
            PartAccessor::A => &mut self.a,
            PartAccessor::S => &mut self.s,
        }
    }

    fn split_at(&self, accessor: &PartAccessor, value: u64, upperbound: bool) -> Option<(RangedPart, RangedPart)> {
        let mut ranged_part_left = self.clone();
        let mut ranged_part_right = self.clone();

        let range = ranged_part_left.access_part(accessor);
        let (left, right) = range.split_at(value, upperbound)?;

        *ranged_part_left.access_part(accessor) = left;
        *ranged_part_right.access_part(accessor) = right;

        Some((ranged_part_left, ranged_part_right))
    }
}

#[derive(Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Range { start, end }
    }

    fn size(&self) -> u64 {
        self.end - self.start + 1
    }

    fn split_at(&self, value: u64, upperbound: bool) -> Option<(Range, Range)> {
        let right_value = if upperbound { value } else { value - 1 };
        let left_value = if upperbound { value + 1 } else { value };

        if right_value <= self.start || left_value >= self.end {
            return None;
        }
        Some((Range::new(self.start, right_value), Range::new(left_value, self.end)))
    }
}

struct System<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
}

impl System<'_> {
    fn execute_workflow(&self, part: &Part) -> Result {
        let start = self.workflows.get("in").expect("workflow in has to exist");

        fn execute_workflow_inner(system: &System, part: &Part, current: &Workflow) -> Result {
            match current.execute(part) {
                RuleResult::Result(result) => *result,
                RuleResult::Redirect(name) => {
                    let next = system.workflows.get(name).expect("system cannot contain unexisting workflow");
                    execute_workflow_inner(system, part, next)
                }
            }
        }

        execute_workflow_inner(self, part, start)
    }

    fn sum_all_accepted(&self) -> u64 {
        self.parts.iter()
            .filter(|part| self.execute_workflow(part) == Result::Accept)
            .map(|part| part.sum_all())
            .sum()
    }

    fn get_accepted_in_range_rule_result(&self, rule_result: &RuleResult, ranged_part: RangedPart) -> u64 {
        match rule_result {
            RuleResult::Result(Result::Accept) => ranged_part.multiply_all(),
            RuleResult::Result(Result::Reject) => 0,
            RuleResult::Redirect(name) => {
                let next = self.workflows.get(name).expect("system cannot contain unexisting workflow");
                self.get_accepted_in_range(next, ranged_part)
            }
        }
    }

    fn get_accepted_in_range(&self, workflow: &Workflow, mut ranged_part: RangedPart) -> u64 {
        let mut result = 0;
        for rule in &workflow.rules {
            match rule {
                Rule::HigherThan(part, value, rule_result) => {
                    if let Some((left_ranged_part, right_ranged_part)) = ranged_part.split_at(part, *value, true) {
                        result += self.get_accepted_in_range_rule_result(rule_result, right_ranged_part);
                        ranged_part = left_ranged_part;
                    } else {
                        result += self.get_accepted_in_range_rule_result(rule_result, ranged_part);
                    }
                }
                Rule::LowerThan(part, value, rule_result) => {
                    if let Some((left_ranged_part, right_ranged_part)) = ranged_part.split_at(part, *value, false) {
                        result += self.get_accepted_in_range_rule_result(rule_result, left_ranged_part);
                        ranged_part = right_ranged_part;
                    } else {
                        result += self.get_accepted_in_range_rule_result(rule_result, ranged_part);
                    }
                }
                Rule::Immediate(rule_result) => {
                    result += self.get_accepted_in_range_rule_result(rule_result, ranged_part);
                }
            }
        }
        result
    }
}

fn parse_rule_result(input: &str) -> RuleResult {
    match input {
        "A" => RuleResult::Result(Result::Accept),
        "R" => RuleResult::Result(Result::Reject),
        redirect => RuleResult::Redirect(redirect),
    }
}

fn parse_rule(input: &str) -> Rule {
    if let Some((condition_string, then_string)) = input.split_once(":") {
        if let Some((variable, value)) = condition_string.split_once(">") {
            let variable = variable.parse::<PartAccessor>().expect("variable has to be x, m, a or s");
            let value = value.parse::<u64>().expect("value has to be a number");
            Rule::HigherThan(variable, value, parse_rule_result(then_string))
        } else if let Some((variable, value)) = condition_string.split_once("<") {
            let variable = variable.parse::<PartAccessor>().expect("variable has to be x, m, a or s");
            let value = value.parse::<u64>().expect("value has to be a number");
            Rule::LowerThan(variable, value, parse_rule_result(then_string))
        } else {
            panic!("condition has to contain > or <")
        }
    } else {
        Rule::Immediate(parse_rule_result(input))
    }
}

fn parse_workflow(input: &str) -> Workflow {
    let (workflow_name, rules) = input.split_once("{").expect("workflow has to contain {");
    let rules = rules.trim_end_matches("}").split(",").map(parse_rule).collect();
    Workflow { name: workflow_name, rules }
}

fn parse_part(input: &str) -> Part {
    let mut x = 0;
    let mut m = 0;
    let mut a = 0;
    let mut s = 0;
    input
        .trim_start_matches("{")
        .trim_end_matches("}")
        .split(",")
        .for_each(|part| {
            let (variable_name, value) = part.split_once("=").expect("part has to contain =");
            let value = value.parse::<u64>().expect("value has to be a number");
            match variable_name {
                "x" => x = value,
                "m" => m = value,
                "a" => a = value,
                "s" => s = value,
                _ => panic!("unknown variable name"),
            }
        });

    Part { x, m, a, s }
}

fn parse_system(input: &str) -> System {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    let mut parsing_workflow = true;

    for line in input.lines() {
        if line.is_empty() {
            parsing_workflow = false;
            continue;
        }
        if parsing_workflow {
            let workflow = parse_workflow(line);
            workflows.insert(workflow.name, workflow);
        } else {
            let part = parse_part(line);
            parts.push(part);
        }
    }

    System { workflows, parts }
}

fn part_2(system: &System) -> u64 {
    let in_workflow = system.workflows.get("in").expect("workflow in has to exist");
    system.get_accepted_in_range(in_workflow, RangedPart::new_all_same(Range::new(1, 4000)))
}

fn main() {
    let input = include_str!("input.txt");
    let system = parse_system(input);

    println!("Part 1: {}", system.sum_all_accepted());
    println!("Part 2: {}", part_2(&system));
}

use crate::syntax_tree;

pub fn compile(tree: syntax_tree::FractranInput) -> Engine {
    let code = build_fractions(&tree.frac_list);
    let init = tree.init;
    Engine::new(code, init)
}

pub struct Engine {
    code: Vec<Fraction>,
    current: u128,
    limit: usize,
}

pub fn run_program(mut engine: Engine, limit: Option<usize>) -> u128 {
    if let Some(limit) = limit {
        engine.set_limit(limit);
        while engine.step_one_with_limit() {}
    } else {
        while engine.step_one() {}
    }

    engine.current
}

pub fn debug_program(mut engine: Engine, limit: Option<usize>) -> u128 {
    let engine = if let Some(limit) = limit {
        engine.set_limit(limit);
        run_debug(engine, |e| e.step_one_with_limit())
    } else {
        run_debug(engine, |e| e.step_one())
    };

    engine.current
}

fn run_debug<F>(mut engine: Engine, f: F) -> Engine
where
    F: Fn(&mut Engine) -> bool,
{
    let mut i = 0;
    loop {
        println!("{} - {}", i, engine.current);
        if !f(&mut engine) {
            break;
        }
        i += 1;
    }
    engine
}

impl Engine {
    fn new(code: Vec<Fraction>, init: u128) -> Self {
        Self {
            code,
            current: init,
            limit: 0,
        }
    }

    fn set_limit(&mut self, limit: usize) {
        self.limit = limit;
    }

    fn step_one_with_limit(&mut self) -> bool {
        if self.limit == 0 {
            false
        } else {
            self.limit -= 1;
            self.step_one()
        }
    }

    fn step_one(&mut self) -> bool {
        for frac in &self.code {
            if let Some(next) = frac.try_multiply(self.current) {
                self.current = next;
                return true;
            }
        }
        false
    }
}

struct Fraction {
    numerator: u128,
    denominator: u128,
}

impl Fraction {
    fn new(n: u128, d: u128) -> Self {
        Self {
            numerator: n,
            denominator: d,
        }
    }

    fn try_multiply(&self, n: u128) -> Option<u128> {
        let rem = n % self.denominator;
        if rem == 0 {
            let val = (self.numerator * n) / self.denominator;
            Some(val)
        } else {
            None
        }
    }
}

fn build_fractions(list: &[(u128, u128)]) -> Vec<Fraction> {
    list.into_iter()
        .map(|(n, d)| Fraction::new(*n, *d))
        .collect()
}

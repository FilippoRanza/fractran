use crate::syntax_tree;

pub fn compile(tree: syntax_tree::FractranInput) -> Engine {
    let code = build_fractions(&tree.frac_list);
    let init = tree.init;
    Engine::new(code, init)
}

fn build_fractions(list: &[(u128, u128)]) -> Vec<Fraction> {
    list.into_iter()
        .map(|(n, d)| Fraction::new(*n, *d))
        .collect()
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

pub struct Engine {
    code: Vec<Fraction>,
    pub current: u128,
}

pub enum EngineStatus {
    Progress,
    Halt,
}

impl Engine {
    fn new(code: Vec<Fraction>, init: u128) -> Self {
        Self {
            code,
            current: init,
        }
    }

    pub fn step_one(&mut self) -> EngineStatus {
        for frac in &self.code {
            if let Some(next) = frac.try_multiply(self.current) {
                self.current = next;
                return EngineStatus::Progress;
            }
        }
        EngineStatus::Halt
    }
}

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

struct Engine {
    code: Vec<Fraction>,
    current: u128,
}

enum EngineStatus {
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

    fn step_one(&mut self) -> EngineStatus {
        for frac in &self.code {
            if let Some(next) = frac.try_multiply(self.current) {
                self.current = next;
                return EngineStatus::Progress;
            }
        }
        EngineStatus::Halt
    }
}

fn main() {
    let fracs = vec![
        (17, 91),
        (78, 85),
        (19, 51),
        (23, 38),
        (29, 33),
        (77, 29),
        (95, 23),
        (77, 19),
        (1, 17),
        (11, 13),
        (13, 11),
        (15, 2),
        (1, 7),
        (55, 1),
    ];

    let code = fracs.into_iter().map(|(n, d)| Fraction::new(n, d)).collect();

    let mut engine = Engine::new(code, 2);
    for _ in 0..100 {
        println!("{}", engine.current);
        if let EngineStatus::Halt = engine.step_one() {
            break;
        }
    }

}

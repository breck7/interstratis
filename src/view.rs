use lichen::parse::{Parser, Env};
use lichen::eval::{Evaluator,Eval};
use lichen::var::Var;

use std::fs;
use std::io::BufReader;
use std::io::prelude::*;


pub struct View {
    env: Env
}

impl View {
    pub fn new (view: &str) -> Option<View> {
        if let Ok(h) = fs::File::open(view) {
            let mut r = BufReader::new(h);
            let mut src = String::new();
            if let Ok(rb) = r.read_to_string(&mut src) {
                if rb > 0 {
                    let p = Parser::parse_blocks(&src);
                    if let Ok(p) = p {
                        return Some(View { env: p.into_env() })
                    }
                }
            }
        }

        None
    }
    
    pub fn render (&mut self) -> String {
        let mut data = Data;
        let mut r = String::new();
        let eval = Evaluator::new(&mut self.env, &mut data); // NOTE: unless we save env, this will mut over time

        for (mut vars,_) in eval {
            for var in vars.drain(..) {
                match var {
                    Var::String(s) => r.push_str(s.as_str()),
                    _ => {},
                }
            }
        }

        r
    }
}

struct Data;
impl Eval for Data {
    #[allow(unused_variables)]
    fn get (&self, path: Option<Vec<&str>>, lookup: &str) -> Option<Var> {
        None
    }

    #[allow(unused_variables)]
    fn set (&mut self, path: Option<Vec<&str>>, lookup: &str, var: Var) {
    }

    fn call (&mut self, var: Var, fun: &str, vars: &Vec<Var>) -> Option<Var> {
        match fun {
            "tag" => { // TODO: use non-symmetric tag endings too
                let tag = vars[0].to_string();
                let mut s = "<".to_owned();
                s.push_str(&tag);
                s.push('>');
                let var = var.to_string();
                s.push_str(&var);
                s.push_str("</");
                s.push_str(&tag);
                s.push('>');

                Some(Var::String(s))
            },
            _ => { None }
        }
    }
}

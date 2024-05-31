
use serde::Deserialize;
use std::{
    collections::HashMap,
    io::Read, vec::Vec,
    process::Command,
    path, str, fs};

#[derive(Deserialize)]
struct Inventory {
    progs: toml::Table,
    langs: toml::Table,
    cases: toml::Table,
}

fn load_inventory<P: AsRef<path::Path>>(path: P) -> Inventory {
    let mut content = String::new();
    let mut file = fs::File::open(path).unwrap();
    file.read_to_string(&mut content).unwrap();
    toml::from_str(&content).unwrap()
}

#[derive(Debug)]
struct Case {
    name: String,
    prog: String,
    inp: String,
    out: String,
}

#[derive(Debug)]
struct Runner {
    langs: HashMap<String, String>,
    progs: HashMap<String, String>,
    cases: Vec<Case>,
}

impl From<Inventory> for Runner {
    fn from(inv: Inventory) -> Self {
        let mut langs = HashMap::new();
        let mut progs = HashMap::new();
        let mut cases = vec![];
        for (lang, short) in inv.langs {
            langs.insert(short.try_into().unwrap(), lang);
        }
        for (prog, tests) in inv.cases {
            let tests: toml::Table = tests.try_into().unwrap();
            for (name, case) in tests {
                let inp = case.get("inp").unwrap().clone().try_into().unwrap();
                let out = case.get("out").unwrap().clone().try_into().unwrap();
                let name = format!("{prog}.{name}");
                cases.push(Case { prog: prog.clone(), name, inp, out });
            }
        }
        for (prog, langs) in inv.progs {
            for lang in langs.try_into::<Vec<String>>().unwrap() {
                progs.insert(prog.clone(), lang);
            }
        }
        Runner { progs, langs, cases }
    }
}

impl Runner {
    fn test(&self, case: &Case) {
        let short = self.progs.get(&case.prog).unwrap();
        let lang = self.langs.get(short).unwrap();
        let out = Command::new(lang)
            .args([format!("{short}/{}.{short}", &case.prog)])
            .args(case.inp.clone().split(' '))
            .output().unwrap();
        if cfg!(debug_assertions) {
            println!("{lang} {short}/{}.{short} {}", &case.prog, case.inp.clone());
        }
        assert_eq!(str::from_utf8(&out.stdout).unwrap(), case.out);
        println!("Test case {} on {}.{short} succeeded!", case.name, case.prog);
    }

    fn all(&self) {
        for case in &self.cases {
            self.test(case);
        }
    }
}

fn main() {
    let runner: Runner = load_inventory("inventory.toml").into();
    runner.all();
}

#[test]
fn it_works() {
    let result = 2 + 2;
    println!("balls");
    assert_eq!(result, 4);
}

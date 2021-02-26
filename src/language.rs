use std::collections::BTreeMap;

pub struct Compiler {
    output: Vec<u8>,
    input: String,
    labels: BTreeMap<String, usize>,
    errors: Vec<String>
}

impl Compiler {
    pub fn new(input: String) -> Self {
        Self {
            output: vec![],
            input,
            labels: BTreeMap::new(),
            errors: vec![],
        }
    }

    fn load(&mut self, splited: &Vec<&str>, current: &mut usize) {

    }

    pub fn compile(&mut self) -> Result<Vec<u8>, Vec<String>> {
        let cloned = self.input.clone();
        for line in cloned.lines() {
            let splited = line.split(|c| {
                c == ' ' || c == '\t'
            }).filter(|el| *el != "").collect::<Vec<_>>();
            let mut current = 0;
            if splited.len() != 0 {
                match splited[current] {
                    "load" => self.load(&splited, &mut current),
                    _ => {},
                }
            }
        }

        Ok(
            self.output.clone()
        )
    }


}
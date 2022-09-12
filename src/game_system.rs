use std::default::Default;

pub enum ModuleType {
    SolarCell,
    Habitation,
    ScienceLab
}

impl ToString for ModuleType {
    fn to_string(&self) -> String {
        match self {
            ModuleType::SolarCell => String::from("Solar Cell"),
            ModuleType::Habitation => String::from("Habitation"),
            ModuleType::ScienceLab => String::from("Science Lab")
        }
    }
}

pub struct Module {
    module_type: ModuleType,
    breakdown_bias: f32,
    broken: bool
}

impl Module {
    pub fn new(module_type: ModuleType, breakdown_bias: f32, broken: bool) -> Self {
        Module {module_type, breakdown_bias, broken}
    }
}

impl ToString for Module {
    fn to_string(&self) -> String {
        format!("{} STATUS {}({:.2})", self.module_type.to_string(), if self.broken {String::from("BROKEN")} else {String::from("NOMINAL")}, self.breakdown_bias)
    }
}

pub enum StationName{
    ISS,
    Mir
}

impl ToString for StationName {
    fn to_string(&self) -> String {
        match self {
            StationName::ISS => String::from("ISS"),
            StationName::Mir => String::from("Mir")
        }
    }
}

pub struct Station {
    name: StationName,
    modules: Vec<Module>
}

impl Station {
    pub fn new(name: StationName, modules: Vec<Module>) -> Self {
        Station {name, modules}
    }
}

impl Default for Station {
    fn default() -> Self {
        Station::new(StationName::ISS, vec![
            Module::new(ModuleType::SolarCell, 0.0, false),
            Module::new(ModuleType::Habitation, 0.0, false),
            Module::new(ModuleType::ScienceLab, 0.0, false),
            Module::new(ModuleType::SolarCell, 0.0, false),
        ])
    }
}

impl ToString for Station {
    fn to_string(&self) -> String {
        let mut ret = self.name.to_string();
        for module in &self.modules {
            ret.push_str("\n\t");
            ret.push_str(&module.to_string())
        }
        ret
    }
}
use std::default::Default;
use rand::prelude::*;
use rand_derive2::RandGen;
#[derive(RandGen)]
pub enum ModuleType {
    SolarCell,
    Habitation,
    ScienceLab
}

impl ModuleType {
    pub(crate) fn value(&self) -> f32 {
        match self {
            Self::SolarCell => 0.2,
            Self::Habitation => 0.1,
            Self::ScienceLab => 0.4,
        }
    }
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

    pub fn tick(&mut self) {
        let break_value: f32 = random();
        if break_value < self.breakdown_bias {
            self.broken = true;
        }
        else {
            self.breakdown_bias += break_value * self.module_type.value();
        }
    }
}

impl ToString for Module {
    fn to_string(&self) -> String {
        format!("{} STATUS {}({:.2})", self.module_type.to_string(), if self.broken {String::from("BROKEN")} else {String::from("NOMINAL")}, self.breakdown_bias)
    }
}

impl Default for Module {
    fn default() -> Self {
        Self::new(random(), random(), random())
    }
}

#[derive(RandGen)]
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
    pub fn tick(&mut self) {
        for module in &mut self.modules {
            module.tick()
        }
    }
    pub fn get_random_station() -> Self {
        Self::new(random(), (0..10).map(|_| Module::default()).collect())
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

pub struct Player{
    name: String
}
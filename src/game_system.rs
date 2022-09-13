use std::default::Default;
use rand::prelude::*;
use rand_derive2::RandGen;

#[derive(RandGen, PartialEq)]
pub enum ModuleType {
    SolarCell,
    Habitation,
    ScienceLab,
    Transceiver,
}

impl ModuleType {
    pub(crate) fn value(&self) -> f32 {
        match self {
            Self::SolarCell => 0.2,
            Self::Habitation => 0.1,
            Self::ScienceLab => 0.4,
            Self::Transceiver => 0.1
        }
    }
}


impl ToString for ModuleType {
    fn to_string(&self) -> String {
        match self {
            ModuleType::SolarCell => String::from("Solar Cell"),
            ModuleType::Habitation => String::from("Habitation"),
            ModuleType::ScienceLab => String::from("Science Lab"),
            ModuleType::Transceiver => String::from("Transceiver")
        }
    }
}

pub struct Module {
    pub module_type: ModuleType,
    pub breakdown_bias: f32,
    pub broken: bool,
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
        format!("{} STATUS {}({:.2})", self.module_type.to_string(),  if self.broken {String::from("BROKEN")} else {String::from("NOMINAL")},  self.breakdown_bias)
    }
}

impl Default for Module {
    fn default() -> Self {
        Self::new(random(), 0.0, false)
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
    pub name: StationName,
    pub modules: Vec<Module>
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
        Self::new(random(), (0..10).map(|_| Module::new(random(), 0.0, false)).collect())
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
    pub name: String,
    pub days_survived: i32,
    pub pips_left_today: i32,
    pub station: Station,
    pub science_done: f32
}

impl Player {
    pub fn new(name: String, on_station: Station) -> Self {
        Self { name, days_survived: 0, pips_left_today: 3, station: on_station, science_done: 0.0 }
    }

    pub fn do_science(&mut self) -> f32 {
        let mut operating_science_labs = 0.0;
        for module in &self.station.modules {
            if module.module_type == ModuleType::ScienceLab {
                operating_science_labs += 1.0;
            }
        }
        let science_done = f32::exp(self.days_survived as f32) * operating_science_labs;
        self.science_done += science_done;
        self.pips_left_today -= 1;
        science_done
    }

    pub fn fix_module_at_index(&mut self, index: usize){
        self.station.modules[index].broken = false;
        self.station.modules[index].breakdown_bias = 0.0;
        self.pips_left_today -= 1;
    }
}
use std::default::Default;
use rand::prelude::*;
use rand_derive2::RandGen;

#[derive(RandGen, PartialEq, Copy, Clone)]
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
#[derive(Clone, Copy)]
pub struct Module {
    pub module_type: ModuleType,
    pub breakdown_bias: f32,
    pub broken: bool,
}
impl Module {
    pub fn new(module_type: ModuleType, breakdown_bias: f32, broken: bool) -> Self {
        Module {module_type, breakdown_bias, broken}
    }

    pub fn tick(&mut self, days_survived: i32) -> bool {

        let break_value: f32 = random();
        if break_value * self.get_break_chance_modifier(days_survived) < self.breakdown_bias {
            self.broken = true;
        }
        else {
            self.breakdown_bias += break_value * self.module_type.value();
        }
        self.broken
    }

    pub fn get_break_chance_modifier(&self, days_survived: i32) -> f32 {
        1.0 / f32::ceil(days_survived as f32 / 3.0)
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

pub enum StationTickMessage {
    NothingToReport,
    ModulesBroke(Vec<Module>),
    LostStation
}

impl Station {
    pub fn new(name: StationName, modules: Vec<Module>) -> Self {
        Station {name, modules}
    }
    pub fn tick(&mut self, days_survived: i32) -> StationTickMessage {
        {
            let mut v = Vec::<Module>::new();
            for module in &mut self.modules {
                if module.tick(days_survived) {
                    v.push(module.clone());
                }
            }

            if v.len() > 0 {
                return StationTickMessage::ModulesBroke(v.clone())
            }
        }

        if !self.station_safe() {
            StationTickMessage::LostStation
        }
        else {
            StationTickMessage::NothingToReport
        }
    }
    pub fn get_random_station() -> Self {
        let mut v = vec![
            Module::new(ModuleType::Habitation, 0.0, false),
            Module::new(ModuleType::Transceiver, 0.0, false),
            Module::new(ModuleType::ScienceLab, 0.0, false),
            Module::new(ModuleType::SolarCell, 0.0, false)];
        for _ in 0..6 {
            let r_num: f32 = random();
            if r_num >= 0.2 {
                v.push(Module::new(random(), 0.0, false))
            }
        }

        let solar_cells : i32 = v.iter().map(|x| {if x.module_type == ModuleType::SolarCell {1} else {0}}).sum();
        let other_modules = (v.len() - solar_cells as usize) as i32;
        let mut delta = (solar_cells * 2) - other_modules;
        while delta <= 2 {
            v.push(Module::new(ModuleType::SolarCell, 0.0, false));
            delta += 2;
        }

        Self::new(random(), v)
    }

    pub fn get_operational_modules_count(&self, module_type: ModuleType) -> i32 {
        let mut operational_modules = 0;
        for module in &self.modules {
            if module.module_type == module_type && !module.broken {
                operational_modules += 1;
            }
        }
        operational_modules
    }

    pub fn get_operational_modules_on_station(&self) -> i32 {
        self.modules.iter().map(|x| {if x.broken {0} else {1}}).sum()
    }

    pub fn station_safe(&self) -> bool {
        let living_solar_cells = self.get_operational_modules_count(ModuleType::SolarCell);
        (self.get_operational_modules_on_station() - living_solar_cells) <= 2 * living_solar_cells
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

        let science_done = f32::exp(self.days_survived as f32) * self.station.get_operational_modules_count(ModuleType::ScienceLab) as f32;
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
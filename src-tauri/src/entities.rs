use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

type StopName = String;

#[derive(Debug)]
pub struct AppState {
    pub db_path: PathBuf,
    pub bus_numbers: Vec<BusNumber>,
    pub stop_to_lines: HashMap<StopName, Vec<Line>>,
    pub stops: HashMap<Line, HashSet<(u8, StopName)>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Line(pub String, pub Direction);

#[derive(Debug)]
pub struct BusNumber {
    pub line: Line,
    pub data_uuid: String,
    pub start_stop: String,
    pub terminal_stop: String,
}

// 更换为HashMap
// #[derive(Debug)]
// pub struct StopToLines {
//     pub stop_name: String,
//     pub lines: Vec<Line>,
// }

// 更换为HashMap + HashSet
// #[derive(Debug)]
// pub struct Stops {
//     pub line: Line,
//     pub stop_count: u8,
//     pub stop_name: String,
// }

impl AppState {
    pub fn new(db_path: PathBuf) -> Self {
        AppState {
            db_path,
            bus_numbers: vec![],
            stop_to_lines: HashMap::new(),
            stops: HashMap::new(),
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    algorithm::dijkstra,
    dbaccess::{connect_db, get_lines_name_by_keyword, get_stops_name_by_keywords},
    entities::{AppState, Direction, Line},
};

// 根据用户输入 keyword 找到对应的站
#[tauri::command]
pub fn search_stops_name(state: tauri::State<AppState>, keyword: &str) -> Result<String, String> {
    connect_db(state.db_path.clone())
        .map_err(|_| "Error: connect_db error".to_string())
        .and_then(|conn| {
            get_stops_name_by_keywords(&conn, keyword)
                .map(|res| serde_json::json!(res).to_string())
                .map_err(|_| "Error: get_stops_name_by_keywords error".to_string())
        })
}

// 根据用户输入的 keyword 找对应的线路
#[tauri::command]
pub fn search_lines_name(state: tauri::State<AppState>, keyword: &str) -> Result<String, String> {
    connect_db(state.db_path.clone())
        .map_err(|_| "Error: connect_db error".to_string())
        .and_then(|conn| {
            get_lines_name_by_keyword(&conn, keyword)
                .map(|res| {
                    let res_str = res
                        .iter()
                        .map(|line| line.to_string())
                        .collect::<Vec<String>>();
                    serde_json::json!(res_str).to_string()
                })
                .map_err(|_| "Error: get_lines_name_by_keyword error".to_string())
        })
}

#[derive(Debug, Serialize, Deserialize)]
struct BusPath {
    length: u32,
    path_vec: Vec<(String, String)>,
}

// 找始发站和最短路径，返回length为乘坐的站数，path_vec为路径
#[tauri::command]
pub fn search_the_path(
    state: tauri::State<AppState>,
    start: &str,
    terminal: &str,
) -> Result<String, String> {
    if start.is_empty() || terminal.is_empty() {
        return Ok("".to_string());
    }
    let (len, path) = dijkstra(&state.stops, &state.stop_to_lines, &start.to_string());
    // 假定传进来的start和terminal都是有效的，因为前端只能从搜索中选择特定的站
    // 加了个同站，或者两站之间没有路径的
    let len: u32 = match len.get(terminal) {
        Some(Some(len)) => len.clone().1,
        _ => 0,
    };
    let mut path_vec: Vec<(String, String)> = vec![];
    let mut terminal_last = terminal;
    while let Some(prev_stop) = path.get(terminal_last) {
        if prev_stop.1.as_str() == start {
            break;
        }
        path_vec.push((prev_stop.0.to_string(), prev_stop.1.clone()));
        terminal_last = prev_stop.1.as_str();
    }
    path_vec.reverse();
    let bus_path = BusPath {
        length: len,
        path_vec,
    };
    Ok(serde_json::json!(bus_path).to_string())
}

// 根据用户输入的站，获取该站停靠的所有线路
#[tauri::command]
pub fn search_the_stops_lines(
    state: tauri::State<AppState>,
    stop_name: &str,
) -> Result<String, String> {
    // 假定输入的stop name有效，因为前端只让用户选择
    Ok(serde_json::json!(state
        .stop_to_lines
        .get(stop_name)
        .unwrap()
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>())
    .to_string())
}

// 根据用户选择线路，获取该线路停靠的所有站点
#[tauri::command]
pub fn search_the_lines_stops(
    state: tauri::State<AppState>,
    line_name: &str,
    direction: i32, // 0 是上行，1 是下行
) -> Result<String, String> {
    let line = Line(
        line_name.to_string(),
        match direction {
            0 => Direction::Up,
            _ => Direction::Down,
        },
    );
    let line_vec = state.stops.get(&line).unwrap().to_vec();
    Ok(serde_json::json!(line_vec).to_string())
}

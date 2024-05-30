use crate::entities::{BusNumber, Direction, Line};
use rusqlite::{Connection, Result};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

type StopName = String;

// app_handle: AppHandle
pub fn connect_db(db_path: PathBuf) -> Result<Connection> {
    // let db_path = app_handle
    //     .path_resolver()
    //     .resolve_resource("_up_/bus-data/bus.db")
    //     .expect("failed to resolve resource");
    dbg!(db_path.clone());
    Connection::open(db_path)
    // let conn = Connection::open(db_path)?;
    // let mut stmt = conn.prepare("select * from bus_number where 线路号=10")?;
    // let bus_iter = stmt.query_map([], |row| {
    //     Ok(BusNumber {
    //         line_number: row.get(0)?,
    //         data_uuid: row.get(1)?,
    //         direction: row.get(2)?,
    //         start_stop: row.get(3)?,
    //         terminal_stop: row.get(4)?,
    //     })
    // })?;

    // for bus in bus_iter {
    //     dbg!(bus.unwrap());
    // }
}

pub fn get_bus_numbers(conn: &Connection) -> Result<Vec<BusNumber>> {
    let mut stmt = conn.prepare("select * from bus_number")?;
    let bus_vec = stmt
        .query_map([], |row| {
            Ok(BusNumber {
                line: Line(
                    row.get(0)?,
                    match row.get(2)? {
                        0 => Direction::Up,
                        _ => Direction::Down,
                    },
                ),
                data_uuid: row.get(1)?,
                start_stop: row.get(3)?,
                terminal_stop: row.get(4)?,
            })
        })?
        .map(|bus_number| bus_number.unwrap())
        .collect::<Vec<BusNumber>>();
    Ok(bus_vec)
}

pub fn get_stop_to_lines(conn: &Connection) -> Result<HashMap<StopName, HashSet<Line>>> {
    let mut stmt = conn.prepare("select * from stop_to_lines")?;
    let stops_vec = stmt
        .query_map([], |row| {
            let line_str: String = row.get(1)?;
            Ok((
                row.get::<_, String>(0)?,
                // 分割线路String，然后检查倒数第二个字符的数字，然后获取从0到倒数第三个字符即为线路名
                line_str
                    .split(", ")
                    .map(|s: &str| match s.chars().rev().nth(1).unwrap() {
                        '1' => Line(s[0..s.len() - 3].to_string(), Direction::Down),
                        _ => Line(s[0..s.len() - 3].to_string(), Direction::Up),
                    })
                    .collect::<HashSet<Line>>(),
            ))
        })?
        .map(|stop| stop.unwrap())
        .collect::<HashMap<StopName, HashSet<Line>>>();
    Ok(stops_vec)
}

pub fn get_stops(conn: &Connection) -> Result<HashMap<Line, Vec<(u8, StopName)>>> {
    let mut stmt = conn.prepare("select * from stops")?;
    let mut stops: HashMap<Line, Vec<(u8, StopName)>> = HashMap::new();
    stmt.query_map([], |row| {
        let line = Line(
            row.get(0)?,
            match row.get(1)? {
                0 => Direction::Up,
                _ => Direction::Down,
            },
        );
        Ok((line, (row.get::<_, u8>(2)?, row.get::<_, String>(3)?)))
    })?
    .for_each(|stop| {
        let stop = stop.unwrap();
        stops.entry(stop.0).or_default().push(stop.1);
    });
    Ok(stops)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::entities::{BusNumber, Direction, Line};

    use super::{connect_db, get_bus_numbers, get_stop_to_lines, get_stops};

    #[test]
    fn test_connect_db() {
        // 这里的路径应该是动态通过app_handle动态获取的
        let db_path = PathBuf::from("/Users/cakeal/Desktop/vsc/beijing-bus-transfer-system/src-tauri/target/debug/_up_/bus-data/bus.db");
        let conn = connect_db(db_path).unwrap();
        // dbg!(conn);
        let mut stmt = conn
            .prepare("select * from bus_number where 线路号=10")
            .unwrap();
        let bus_iter = stmt
            .query_map([], |row| {
                Ok(BusNumber {
                    line: Line(
                        row.get(0)?,
                        match row.get(2)? {
                            0 => Direction::Up,
                            _ => Direction::Down,
                        },
                    ),
                    data_uuid: row.get(1)?,
                    start_stop: row.get(3)?,
                    terminal_stop: row.get(4)?,
                })
            })
            .unwrap();

        for bus in bus_iter {
            println!("{:?}", bus.unwrap());
        }
    }

    #[test]
    fn test_get_bus_numbers() {
        let db_path = PathBuf::from("/Users/cakeal/Desktop/vsc/beijing-bus-transfer-system/src-tauri/target/debug/_up_/bus-data/bus.db");
        let conn = connect_db(db_path).unwrap();
        let bus_numbers = get_bus_numbers(&conn);
        dbg!(bus_numbers.unwrap());
    }

    #[test]
    fn test_get_stop_to_lines() {
        let db_path = PathBuf::from("/Users/cakeal/Desktop/vsc/beijing-bus-transfer-system/src-tauri/target/debug/_up_/bus-data/bus.db");
        let conn = connect_db(db_path).unwrap();
        let stops_line = get_stop_to_lines(&conn).unwrap();
        dbg!(&stops_line["地铁天通苑北站南"]);
    }

    #[test]
    fn test_get_stops() {
        let db_path = PathBuf::from("/Users/cakeal/Desktop/vsc/beijing-bus-transfer-system/src-tauri/target/debug/_up_/bus-data/bus.db");
        let conn = connect_db(db_path).unwrap();
        let line_stops = get_stops(&conn).unwrap();
        dbg!(&line_stops[&Line("特19".to_string(), Direction::Up)]);
    }
}

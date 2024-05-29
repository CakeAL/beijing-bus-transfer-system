use std::path::PathBuf;

use rusqlite::{Connection, Result};

// app_handle: AppHandle
pub fn connect_db(db_path: PathBuf) -> Result<Connection> {
    // let db_path = app_handle
    //     .path_resolver()
    //     .resolve_resource("_up_/bus-data/bus.db")
    //     .expect("failed to resolve resource");
    dbg!(db_path.clone());
    Ok(Connection::open(db_path)?)
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::entities::BusNumber;

    use super::connect_db;

    #[test]
    fn test_connect_db() {
        let db_path = PathBuf::from("/Users/cakeal/Desktop/vsc/beijing-bus-transfer-system/src-tauri/target/debug/_up_/bus-data/bus.db");
        let conn = connect_db(db_path).unwrap();
        // dbg!(conn);
        let mut stmt = conn
            .prepare("select * from bus_number where 线路号=10")
            .unwrap();
        let bus_iter = stmt
            .query_map([], |row| {
                Ok(BusNumber {
                    line_number: row.get(0)?,
                    data_uuid: row.get(1)?,
                    direction: row.get(2)?,
                    start_stop: row.get(3)?,
                    terminal_stop: row.get(4)?,
                })
            })
            .unwrap();

        for bus in bus_iter {
            println!("{:?}", bus.unwrap());
        }
    }
}

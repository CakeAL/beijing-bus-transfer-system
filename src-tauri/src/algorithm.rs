use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::entities::{Line, StopName};

const DISTANCE: u32 = 1;

// 由于数据源没有两个站点之间的耗时/路程，我们只能假设所有边的长度都是1
// 返回的值是该站点到任何可以到达的站点的长度（站点数）
pub fn dijkstra(
    stops: &HashMap<Line, Vec<(u8, String)>>,
    graph: &HashMap<StopName, HashSet<Line>>,
    start: &StopName,
) -> HashMap<StopName, Option<(StopName, u32)>> {
    // 该站点到任何可以到达的站点的长度（站点数）
    let mut ans = HashMap::new();
    // 从该站到能到达的下一个站点的记录
    let mut prio = BinaryHeap::new();

    // 开始的Stop没有前驱点
    ans.insert((*start).clone(), None);

    // 遍历这个站点所有的公交线路
    for line in graph.get(start).unwrap() {
        // 找到该站点出发乘坐这些线路可以到达的下一个站点
        let next_stop = stops.get(line).and_then(|stops| {
            stops
                .iter()
                .position(|stop| stop.1 == *start)
                .and_then(|index| stops.get(index + 1))
        });
        // 如果下一站不是None
        if let Some(next_stop) = next_stop {
            // 添加下一站到本站的距离(1)
            ans.insert((next_stop.1).clone(), Some(((*start).clone(), DISTANCE)));
            prio.push(Reverse((
                DISTANCE,
                (next_stop.1).clone(),
                (*start).clone(),
            )));
        }
    }

    // 遍历这些记录
    while let Some(Reverse((dist, next_stop, prev_stop))) = prio.pop() {
        match ans.get(&next_stop).unwrap() {
            // 如果下一个站点在ans里，我们根据这个站点再找下一站
            Some((next, dis)) if *next == prev_stop && *dis == dist => {}
            _ => continue,
        }

        // 遍历下一个站点的所有公交线路
        for line in graph.get(&next_stop).unwrap() {
            // 找到下一个站点的下一站
            let next_next_stop = stops.get(line).and_then(|stops| {
                stops
                    .iter()
                    .position(|stop| stop.1 == next_stop)
                    .and_then(|index| stops.get(index + 1))
            });
            // 保证下一站不是None
            if let Some(next_next_stop) = next_next_stop {
                match ans.get(&(next_next_stop.1)) {
                    // 如果下一站的下一站的距离比现在的更长，我们什么也不做
                    Some(Some((_, dist_next))) if dist + DISTANCE >= *dist_next => {}
                    // 如果下一站的下一站是None，那么从下一站的距离不会改变，所以不用再次向prio添加
                    Some(None) => {}
                    // 新路径更短，或者新路径不再ans中，或者更长
                    _ => {
                        ans.insert(
                            next_next_stop.1.clone(),
                            Some((next_stop.clone(), DISTANCE + dist)),
                        );
                        prio.push(Reverse((
                            DISTANCE + dist,
                            next_next_stop.1.clone(),
                            next_stop.clone(),
                        )));
                    }
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::dbaccess::{connect_db, get_stop_to_lines, get_stops};

    use super::dijkstra;


    #[test]
    fn test_dijkstra() {
        let db_path = PathBuf::from("/Users/cakeal/Desktop/vsc/beijing-bus-transfer-system/src-tauri/target/debug/_up_/bus-data/bus.db");
        let conn = connect_db(db_path).unwrap();
        let stops = get_stops(&conn).unwrap();
        let graph = get_stop_to_lines(&conn).unwrap();
        let start = "成府路口南".to_string();
        let res = dijkstra(&stops, &graph, &start);

        dbg!(res.get("碧水庄园").unwrap());
    }
}

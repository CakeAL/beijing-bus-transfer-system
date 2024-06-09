use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

use crate::entities::{Line, StopName};

const DISTANCE: u32 = 1;

// 由于数据源没有两个站点之间的耗时/路程，我们只能假设所有边的长度都是1
// 返回的值是该站点到任何可以到达的站点的长度（站点数），以及path
pub fn dijkstra(
    stops: &'static HashMap<Line, Vec<(u8, String)>>,
    graph: &'static HashMap<StopName, HashSet<Line>>,
    start: &StopName,
    target: &StopName,
) -> (
    HashMap<StopName, Option<(StopName, u32)>>,
    HashMap<StopName, (Line, StopName)>,
) {
    let mut anss = vec![];
    let mut paths = vec![];
    let (tx, rx) = channel();
    let graph = Arc::new(graph);
    let stops = Arc::new(stops);
    let mut handles = vec![];
    // 遍历这个站点所有的公交线路
    for line in graph.get(start).unwrap() {
        let tx2 = tx.clone();
        let start = start.clone();
        let graph = Arc::clone(&graph);
        let stops = Arc::clone(&stops);
        let handle = thread::spawn(move || {
            // 该站点到任何可以到达的站点的长度（站点数）
            let mut ans = HashMap::new();
            // 从该站到能到达的下一个站点的记录
            let mut prio = BinaryHeap::new();
            // 记录path
            let mut path: HashMap<StopName, (Line, StopName)> = HashMap::new();

            // 开始的Stop没有前驱点
            ans.insert(start.clone(), None);
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
                ans.insert((next_stop.1).clone(), Some((start.clone(), DISTANCE)));
                // 路径添加next_stop的路径（线路和始发站）
                path.insert((next_stop.1).clone(), (line.clone(), start.clone()));
                prio.push(Reverse((DISTANCE, (next_stop.1).clone(), start.clone())));
            }
            // 遍历这些记录
            while let Some(Reverse((dist, next_stop, prev_stop))) = prio.pop() {
                match ans.get(&next_stop).unwrap() {
                    // 如果下一个站点在ans里，我们根据这个站点再找下一站
                    Some((next, dis)) if *next == prev_stop && *dis == dist => {}
                    _ => continue,
                }

                // 找一下上个站点坐的哪条线路
                // println!("{} {} {}", dist, next_stop, prev_stop);
                // dbg!(&path);
                let last_line = path.get(&next_stop).unwrap().0.clone();
                // println!("{:?}", last_line);
                // 如果下一站，上一趟公交也经过，优先考虑（不用换乘）
                // let mut signnal = false;
                // if graph.get(&next_stop).unwrap().contains(&last_line) {
                //     signnal = true;
                // }
                if graph.get(&next_stop).unwrap().contains(&last_line) {
                    // 找到下一个站点的下一站
                    let next_next_stop = stops.get(&last_line).and_then(|stops| {
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
                                path.insert(
                                    next_next_stop.1.clone(),
                                    (last_line.clone(), next_stop.clone()),
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

                // 遍历下一个站点的所有公交线路
                graph.get(&next_stop).unwrap().iter().for_each(|line| {
                    if *line != last_line {
                        // 跳过已经存储的
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
                                    path.insert(
                                        next_next_stop.1.clone(),
                                        (line.clone(), next_stop.clone()),
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
                });
            }
            tx2.send((ans, path)).unwrap();
        });
        handles.push(handle);
    }

    // 等待子线程结束
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    // drop 发送端
    drop(tx);

    // 从管道接收结果
    while let Ok((ans, path)) = rx.recv() {
        anss.push(ans);
        paths.push(path);
    }

    let position = anss
        .iter()
        .min_by_key(|&key| {
            key.get(target)
                .unwrap_or(&None)
                .as_ref()
                .unwrap_or(&("".to_string(), u32::MAX))
                .1
        })
        .and_then(|ans| anss.iter().position(|this| this == ans))
        .unwrap();
    (anss[position].clone(), paths[position].clone())
}

// **错误的**
// // dfs 会偏向于不优先换乘的策略
// // 返回一个Option，如果是None则找不到两者间路径，Some说明有
// pub fn dfs(
//     stops: &HashMap<Line, Vec<(u8, String)>>,
//     graph: &HashMap<StopName, HashSet<Line>>,
//     start: &StopName,
//     target: &StopName,
// ) -> Option<Vec<(Line, StopName)>> {
//     let mut answers: Vec<Vec<(Line, String)>> = vec![];
//     let mut answers_len: Vec<usize> = vec![];

//     // 遍历起始点所有的公交线路
//     for line in graph.get(start).unwrap() {
//         let mut visited: HashSet<StopName> = HashSet::new();
//         let mut history: Vec<(Line, StopName)> = vec![];
//         let mut queue = VecDeque::new();
//         // 向队列末尾添加根节点
//         queue.push_back((line.clone(), start.clone()));
//         // 如果queue中有元素，获取头元素
//         while let Some(current_stop) = queue.pop_front() {
//             // 向history添加当前节点
//             history.push(current_stop.clone());
//             // 验证这个站点是否是目标站点，如果是的话就添加到答案集中
//             if current_stop.1.eq(target) {
//                 answers.push(history.clone());
//                 // 收集该路径经过的所有的线路
//                 let lines_set: HashSet<Line> = history.iter().map(|stop| stop.0.clone()).collect();
//                 // 把换乘作为代价加到长度里去
//                 answers_len.push(history.len() + lines_set.len() * 5);
//                 continue;
//             }

//             // 如果当前线路的下一站存在
//             let next_stop = stops.get(&current_stop.0).and_then(|stop| {
//                 stop.iter()
//                     .position(|stop_name| stop_name.1 == current_stop.1)
//                     .and_then(|index| stop.get(index + 1))
//             });
//             // 如果下一个站点存在
//             if let Some(next_stop) = next_stop {
//                 if visited.insert(next_stop.1.clone()) {
//                     // 添加线路的下一站节点
//                     queue.push_front((current_stop.0.clone(), next_stop.1.clone()));
//                 }
//             }

//             // 对于当前站点的其他所有线路可达的下一个站点
//             // 过滤掉当前线路
//             for next_line in graph
//                 .get(&current_stop.1)
//                 .unwrap()
//                 .iter()
//                 .filter(|&next_line| next_line.clone() != current_stop.0)
//             {
//                 // dbg!(next_line.clone());
//                 let next_stop = stops.get(next_line).and_then(|stop| {
//                     stop.iter()
//                         .position(|stop_name| stop_name.1 == current_stop.1)
//                         .and_then(|index| stop.get(index + 1))
//                 });
//                 // 如果下一个站点存在
//                 if let Some(next_stop) = next_stop {
//                     if visited.insert(next_stop.1.clone()) {
//                         // 添加线路的下一站节点
//                         // dbg!((next_line.clone(), next_stop.1.clone()));
//                         queue.push_back((next_line.clone(), next_stop.1.clone()));
//                     }
//                 }
//             }
//         }
//     }

//     if !answers.is_empty() {
//         let index = answers_len
//             .iter()
//             .min()
//             .and_then(|stop_num| {
//                 answers_len
//                     .iter()
//                     .position(|curr_num: &usize| curr_num == stop_num)
//             })
//             .unwrap();
//         return Some(answers[index].clone());
//     }
//     // 如果所有的节点都访问过，并且没有找到终点，返回None
//     None
// }

pub fn improved_dijkstra(
    stops: &'static HashMap<Line, Vec<(u8, String)>>,
    graph: &'static HashMap<StopName, HashSet<Line>>,
    start: &StopName,
    target: &StopName,
) -> (
    HashMap<StopName, Option<(StopName, u32)>>,
    HashMap<StopName, (Line, StopName)>,
) {
    let mut anss = vec![];
    let mut paths = vec![];
    let (tx, rx) = channel();
    let graph = Arc::new(graph);
    let stops = Arc::new(stops);
    let mut handles = vec![];
    // 遍历这个站点所有的公交线路
    for line in graph.get(start).unwrap() {
        let tx2 = tx.clone();
        let start = start.clone();
        let graph = Arc::clone(&graph);
        let stops = Arc::clone(&stops);
        let handle = thread::spawn(move || {
            // 该站点到任何可以到达的站点的长度（站点数）
            let mut ans = HashMap::new();
            // 从该站到能到达的下一个站点的记录
            let mut prio = BinaryHeap::new();
            // 记录path
            let mut path: HashMap<StopName, (Line, StopName)> = HashMap::new();

            // 开始的Stop没有前驱点
            ans.insert(start.clone(), None);
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
                ans.insert((next_stop.1).clone(), Some((start.clone(), DISTANCE)));
                // 路径添加next_stop的路径（线路和始发站）
                path.insert((next_stop.1).clone(), (line.clone(), start.clone()));
                prio.push(Reverse((DISTANCE, (next_stop.1).clone(), start.clone())));
            }
            // 遍历这些记录
            while let Some(Reverse((dist, next_stop, prev_stop))) = prio.pop() {
                match ans.get(&next_stop).unwrap() {
                    // 如果下一个站点在ans里，我们根据这个站点再找下一站
                    Some((next, dis)) if *next == prev_stop && *dis == dist => {}
                    _ => continue,
                }

                // 找一下上个站点坐的哪条线路
                // println!("{} {} {}", dist, next_stop, prev_stop);
                // dbg!(&path);
                let last_line = path.get(&next_stop).unwrap().0.clone();
                // println!("{:?}", last_line);
                // 如果下一站，上一趟公交也经过，优先考虑（不用换乘）
                // let mut signnal = false;
                // if graph.get(&next_stop).unwrap().contains(&last_line) {
                //     signnal = true;
                // }
                if graph.get(&next_stop).unwrap().contains(&last_line) {
                    // 找到下一个站点的下一站
                    let next_next_stop = stops.get(&last_line).and_then(|stops| {
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
                                path.insert(
                                    next_next_stop.1.clone(),
                                    (last_line.clone(), next_stop.clone()),
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

                // 遍历下一个站点的所有公交线路
                graph
                    .get(&next_stop)
                    .unwrap()
                    .iter()
                    .filter(|&line| line.clone() != last_line)
                    .for_each(|line| {
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
                                        Some((next_stop.clone(), DISTANCE * 1000 + dist)), // 增加换乘代价
                                    );
                                    path.insert(
                                        next_next_stop.1.clone(),
                                        (line.clone(), next_stop.clone()),
                                    );
                                    prio.push(Reverse((
                                        DISTANCE * 1000 + dist, // 增加换乘代价
                                        next_next_stop.1.clone(),
                                        next_stop.clone(),
                                    )));
                                }
                            }
                        }
                    });
            }
            tx2.send((ans, path)).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    drop(tx);

    while let Ok((ans, path)) = rx.recv() {
        anss.push(ans);
        paths.push(path);
    }

    let position = anss
        .iter()
        .min_by_key(|&key| {
            key.get(target)
                .unwrap_or(&None)
                .as_ref()
                .unwrap_or(&("".to_string(), u32::MAX))
                .1
        })
        .and_then(|ans| anss.iter().position(|this| this == ans))
        .unwrap();
    (anss[position].clone(), paths[position].clone())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{
        algorithm::improved_dijkstra,
        dbaccess::{connect_db, get_stop_to_lines, get_stops},
    };

    use super::dijkstra;

    #[test]
    fn test_dijkstra() {
        let db_path = PathBuf::from("/Users/cakeal/Desktop/vsc/beijing-bus-transfer-system/src-tauri/target/debug/_up_/bus-data/bus.db");
        let conn = connect_db(db_path).unwrap();
        let stops = get_stops(&conn).unwrap();
        let graph = get_stop_to_lines(&conn).unwrap();
        let start = "成府路口南".to_string();
        let mut terminal = "百旺新城".to_string();
        let (len, path) = dijkstra(&stops, &graph, &start, &terminal);
        dbg!(len.get(&terminal).unwrap());
        while let Some(prev_stop) = path.get(&terminal) {
            println!("{} : {}", prev_stop.0, prev_stop.1);
            if *(prev_stop.1) == start {
                break;
            }
            terminal = prev_stop.1.clone();
        }
    }

    // #[test]
    // fn test_dfs() {
    //     let db_path = PathBuf::from("/Users/cakeal/Desktop/vsc/beijing-bus-transfer-system/src-tauri/target/debug/_up_/bus-data/bus.db");
    //     let conn = connect_db(db_path).unwrap();
    //     let stops = get_stops(&conn).unwrap();
    //     let graph = get_stop_to_lines(&conn).unwrap();
    //     let start = "学院桥东".to_string();
    //     let target = "沙河".to_string();
    //     let res = dfs(&stops, &graph, &start, &target);
    //     // dbg!(res.clone());
    //     // dbg!(res.unwrap().len());
    //     if let Some(res) = res {
    //         let len = res.len();
    //         for (line, stop) in res {
    //             println!("{}: {}", line, stop);
    //         }
    //         println!("{}", len);
    //     } else {
    //         dbg!(res);
    //     }
    // }

    #[test]
    fn test_improved_dijkstra() {
        let db_path = PathBuf::from("/Users/cakeal/Desktop/vsc/beijing-bus-transfer-system/src-tauri/target/debug/_up_/bus-data/bus.db");
        let conn = connect_db(db_path).unwrap();
        let stops = get_stops(&conn).unwrap();
        let graph = get_stop_to_lines(&conn).unwrap();
        let start = "成府路口南".to_string();
        let mut terminal: String = "百旺新城".to_string();
        let (len, path) = improved_dijkstra(&stops, &graph, &start, &terminal);
        dbg!(len.get(&terminal).unwrap());
        while let Some(prev_stop) = path.get(&terminal) {
            println!("{} : {}", prev_stop.0, prev_stop.1);
            if *(prev_stop.1) == start {
                break;
            }
            terminal = prev_stop.1.clone();
        }
    }
}

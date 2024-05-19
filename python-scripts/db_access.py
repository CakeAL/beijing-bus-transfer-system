import sqlite3


def get_db_conn():
    return sqlite3.connect("./bus-data/bus.db")


def create_bus_number_table():
    conn = get_db_conn()
    cur = conn.cursor()
    sql_text = """CREATE TABLE bus_number
                (线路号 TEXT,
                data_uuid TEXT PRIMARY KEY NOT NULL,
                方向 NUMBER,
                始发站 TEXT,
                终到站 TEXT
                );"""
    cur.execute(sql_text)
    conn.commit()
    conn.close()


# (line_id, data_uuid, direction, starting_stop, terminal_stop)
def insert_to_bus_number(data):
    conn = get_db_conn()
    cur = conn.cursor()
    sql_text = "INSERT INTO bus_number VALUES(?, ?, ?, ?, ?)"
    cur.execute(sql_text, data)
    conn.commit()
    conn.close()


def create_stops_table():
    conn = get_db_conn()
    cur = conn.cursor()
    sql_text = """CREATE TABLE stops
                (线路号 TEXT,
                方向 NUMBER,
                站编号 NUMBER,
                站名 TEXT
                );"""
    cur.execute(sql_text)
    conn.commit()
    conn.close()


# (line_id, direction, stop_num, stop_name)
def insert_to_stops(data):
    conn = get_db_conn()
    cur = conn.cursor()
    sql_text = "INSERT INTO stops VALUES(?, ?, ?, ?)"
    cur.execute(sql_text, data)
    conn.commit()
    conn.close()


def get_all_lines():
    conn = get_db_conn()
    cur = conn.cursor()
    sql_text = "SELECT 线路号, data_uuid, 方向 FROM bus_number"
    data = []
    for line_info in cur.execute(sql_text):
        data.append(line_info)
    conn.close()
    return data


def create_stop_to_lines_table():
    conn = get_db_conn()
    cur = conn.cursor()
    sql_text = """CREATE TABLE stop_to_lines
                (站名 TEXT PRIMARY KEY NOT NULL,
                线路 TEXT
                );"""
    cur.execute(sql_text)
    conn.commit()
    conn.close()


def reset_data_to_stop_to_lines():
    conn = get_db_conn()
    cur = conn.cursor()
    # 提取所有站名（有重复）
    cur.execute("SELECT DISTINCT 站名 FROM stops")
    stops = [row[0] for row in cur.fetchall()]
    # 遍历站名
    for stop in stops:
        # 检查该站名是否已经存在于 stop_to_lines 表中
        cur.execute("SELECT * FROM stop_to_lines WHERE 站名 = ?", (stop,))
        existing_stop = cur.fetchone()
        if existing_stop:
            continue

        # 查询该站名在 stops 表中对应的所有线路号和方向
        cur.execute(
            "SELECT DISTINCT 线路号, 方向 FROM stops WHERE 站名 = ?",
            (stop,),
        )
        routes_data = cur.fetchall()

        # 构造线路字符串
        routes_str = ", ".join([f"{route[0]}({route[1]})" for route in routes_data])
        # print(routes_str)

        # 将站名和线路字符串插入到 stop_to_lines 表中
        cur.execute(
            "INSERT INTO stop_to_lines (站名, 线路) VALUES (?, ?)",
            (stop, routes_str),
        )
    print("完成整理数据到stop_to_lines表中")
    conn.commit()
    conn.close()


def main():
    # create_bus_number_table()
    # insert_to_bus_number(data)
    # create_stops_table()
    # get_all_lines()
    # create_stop_to_lines_table()
    reset_data_to_stop_to_lines()
    _ = 233


if __name__ == "__main__":
    main()

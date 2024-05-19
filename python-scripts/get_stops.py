import requests
import re
from db_access import insert_to_stops, get_all_lines

AJAX_RTBUS_DATA = r"https://www.bjbus.com/home/ajax_rtbus_data.php"


def get_stops(line_id, data_uuid, direction):
    r = requests.get(
        AJAX_RTBUS_DATA,
        params={"act": "getDirStation", "selBLine": line_id, "selBDir": data_uuid},
    )
    # print(r.content.decode(encoding="utf-8"))
    pattern = r'<a href="javascript:;" data-seq="(\d+)">(.*?)</a>'
    matches = re.findall(pattern, r.content.decode(encoding="utf-8"))
    results = [(line_id, direction, int(match[0]), match[1]) for match in matches]
    return results


def main():
    # res = get_stops('400快外', '21012115242297247571', 1)
    # print(res)
    all_line = get_all_lines()
    for line in all_line:
        results = get_stops(line[0], line[1], line[2])
        for res in results:
            insert_to_stops(res)
        print(line[0] + " 方向: " + str(line[2]) + " 站点存储完成")


if __name__ == "__main__":
    main()

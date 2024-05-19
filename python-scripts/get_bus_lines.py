import requests
import re
from db_access import insert_to_bus_number

# 北京公交主页，用于获取全部线路名称
MAIN_URL = r"https://www.bjbus.com/home/index.php"
AJAX_RTBUS_DATA = r"https://www.bjbus.com/home/ajax_rtbus_data.php"


# 获取所有线路
def get_all_line_number():
    r = requests.get(MAIN_URL)
    return re.findall(
        '<a href="javascript:;">(.*?)</a>', r.content.decode(encoding="utf-8")
    )


# 获取所有两个方向(或一个)的线路，还有data-uuid，然后存储到db中
def get_lines_with_directions(all_line_number):
    for line in all_line_number:
        r = requests.get(
            AJAX_RTBUS_DATA, params={"act": "getLineDir", "selBLine": line}
        )
        # print(r.content.decode(encoding="utf-8"))
        pattern = r'<a href="javascript:;" data-uuid="(\d+)">(.*?)\((.*?)-(.*?)\)</a>'
        matches = re.findall(pattern, r.content.decode(encoding="utf-8"))
        results = [
            (matches[i][1], matches[i][0], i, matches[i][2], matches[i][3])
            for i in range(0, len(matches))
        ]
        for line in results:
            insert_to_bus_number(line)
            print(line[0] + " 方向: " + str(line[2]) + " 存储完成")


def main():
    all_line_number = get_all_line_number()
    # all_line_number = ["1", "夜21"]
    get_lines_with_directions(all_line_number)


if __name__ == "__main__":
    main()

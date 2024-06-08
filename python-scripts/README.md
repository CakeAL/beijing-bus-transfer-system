# 虚拟环境使用

## 创建虚拟环境

> [!WARNING]  
> 首先你应该在`beijing-bus-transfer-system`文件夹下

```bash
python3 -m venv .venv
```

## 启用虚拟环境

linux/mac:

```bash
source ./.venv/bin/activate
```

win:

```powershell
.\.venv\Scripts\Activate.ps1
```

## 安装所有 requirement 依赖

```bash
pip install -r requirements.txt
```

## 保存所有 requirement 依赖

```bash
pip freeze > requirements.txt
```

## 退出虚拟环境

```bash
deactivate
```

# 获取bus.db

## 创建bus_number表

在`db_access.py`中依次调用

```python
create_bus_number_table()
create_stops_table()
create_stop_to_lines_table()
```

来在`bus-data`文件夹中创建`.db`文件和表

## 将数据插入到bus_number表中

在`get_bus_lines.py`中依次调用：
```python
all_line_number = get_all_line_number()
get_lines_with_directions(all_line_number)
```

## 将数据插入到stops表中

运行`get_stops.py`，可能需要9分钟，你可以更改为多线程，但是我无法确保北京公交是否会封掉你

## 整理数据到stop_to_lines表中

把每个站对应的线路，都存储在`stop_to_lines`中 \
格式大概是（括号中0和1分别代表两个方向的车，环路只有0）

| 站名      | 线路 |
| ----------- | ----------- |
| 学院桥东      | 319(0), 319(1), 386(0), 186(1), 400快外(0)...        |
| 城府路口南   | 26(0), 26(1), 145(0), 145(1)...        |
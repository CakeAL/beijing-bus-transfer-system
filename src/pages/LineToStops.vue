<script setup lang="ts">
import { BusOutline } from "@vicons/ionicons5";
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { store } from "../store.js";

interface LineName {
  label: string;
  value: string;
}

const select_line = ref("");
const lines_name = ref<Array<string>>([]);
const options = ref<Array<LineName>>([]);
const line_stops = ref([]);
const card_title = ref("");

onMounted(() => {
  // 挂载时获取一次数据
  get_lines();
  if (store.line_name !== "") {
    select_line.value = store.line_name;
    get_line_stops();
    store.line_name = "";
  }
});

const get_lines = async () => {
  // 因为我发现了选择器（可过滤）所以这个search功能实际上用不到了
  const result = await invoke("search_lines_name", { keyword: "" }).catch(
    (err) => console.log(err)
  );
  lines_name.value = JSON.parse(result as string);
  lines_name.value.forEach((line_name) => {
    options.value.push({
      label: line_name,
      value: line_name,
    });
  });
  // console.log(lines_name.value);
};

const get_line_stops = async () => {
  let name = select_line.value.split(" ")[0];
  if (name === "") return;
  let dir = 1;
  if (select_line.value.includes("上行")) {
    dir = 0;
  }
  // console.log(name);
  // console.log(dir);
  const result = (await invoke("search_the_lines_stops", {
    lineName: name,
    direction: dir,
  }).catch((err) => console.log(err))) as string;
  line_stops.value = JSON.parse(result);
  // console.log(line_stops.value);
  card_title.value =
    select_line.value +
    " " +
    line_stops.value[0][1] +
    " -> " +
    line_stops.value[line_stops.value.length - 1][1];
};

const store_stop_name = (name) => {
  store.stop_name = name;
  window.location.href = '#/stop_to_lines';
};
</script>

<template>
  <div class="container">
    <p>试试在下面的输入框中输入并选择你想要查询的<b>公交线路</b>吧！</p>
    <n-select
      filterable
      v-model:value="select_line"
      placeholder="线路名"
      :options="options"
      @update:value="get_line_stops"
    >
      <template #arrow>
        <BusOutline />
      </template>
    </n-select>
    <!-- <n-button strong secondary round type="primary" @click="get_line_stops">
      查！
    </n-button> -->
    <n-card class="show-stops" hoverable :title="card_title">
      <n-timeline>
        <n-timeline-item
          v-for="(stop, index) in line_stops"
          :key="index"
          @click="store_stop_name(stop[1])"
          color="#dc2533"
          :title="stop[0]"
          :content="stop[1]"
          class="timeline-item"
        />
      </n-timeline>
    </n-card>
  </div>
</template>

<style scoped>
.container {
  padding: 15px;
}

.show-stops {
  height: 60vh;
  overflow: auto;
  margin-top: 15px;
}

.timeline-item {
  cursor: pointer;
}
</style>

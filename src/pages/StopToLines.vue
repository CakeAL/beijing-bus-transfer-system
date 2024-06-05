<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { store } from "../store.js";

interface StopName {
  label: string;
  value: string;
}

const select_stop = ref("");
const stops_name = ref<Array<string>>([]);
const options = ref<Array<StopName>>([]);
const stop_lines = ref([]);

onMounted(() => {
  // 挂载时获取一次数据
  get_stops();
  if (store.stop_name !== "") {
    select_stop.value = store.stop_name;
    get_stop_lines();
    store.stop_name = "";
  }
});

const get_stops = async () => {
  // 同 LineToStops.vue
  const result = await invoke("search_stops_name", { keyword: "" }).catch(
    (err) => console.log(err)
  );
  stops_name.value = JSON.parse(result as string);
  stops_name.value.forEach((stop_name) => {
    options.value.push({
      label: stop_name,
      value: stop_name,
    });
  });
};

const get_stop_lines = async () => {
  const result = await invoke("search_the_stops_lines", {
    stopName: select_stop.value,
  }).catch((err) => console.log(err));
  stop_lines.value = JSON.parse(result as string);
};

const store_line_name = (name) => {
  store.line_name = name;
  window.location.href = "#/line_to_stops";
};
</script>

<template>
  <div class="container">
    <p>试试在下面的输入框中输入并选择你想要查询的<b>公交站点</b>吧！</p>
    <n-select
      filterable
      v-model:value="select_stop"
      placeholder="站名"
      :options="options"
      @update:value="get_stop_lines"
    >
    </n-select>
    <n-card class="show-lines" hoverable>
      <n-space>
        <n-tag
          :bordered="false"
          type="success"
          class="my-tag"
          v-for="(line, index) in stop_lines"
          :key="index"
          @click="store_line_name(line)"
        >
          {{ line }}
        </n-tag>
      </n-space>
    </n-card>
  </div>
</template>

<style scoped>
.container {
  padding: 15px;
}

.show-lines {
  height: 60vh;
  overflow: auto;
  margin-top: 15px;
}

.my-tag {
  cursor: pointer;
}
</style>

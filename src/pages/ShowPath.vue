<script setup lang="ts">
import { onMounted, ref } from "vue";
import { BusOutline, ArrowForward } from "@vicons/ionicons5";
import { invoke } from "@tauri-apps/api";
import { useLoadingBar } from "naive-ui";

interface StopName {
  label: string;
  value: string;
}

interface BusPath {
  length: number;
  path_vec: Array<[string, string]>;
}

const start_stop = ref("");
const target_stop = ref("");
const options = ref<Array<StopName>>([]);
const bus_path = ref<BusPath>({ length: 0, path_vec: [] });
const my_switch = ref(false);
const loadingBar = useLoadingBar();

onMounted(() => {
  // 挂载时获取一次数据
  get_stops();
});

const get_stops = async () => {
  // 同 LineToStops.vue
  const result = await invoke("search_stops_name", { keyword: "" }).catch(
    (err) => console.log(err)
  );
  let stops_name: Array<string> = JSON.parse(result as string);
  stops_name.forEach((stop_name) => {
    options.value.push({
      label: stop_name,
      value: stop_name,
    });
  });
};

// 用来禁用Switch，防止快速切换
const disabledSwitch = ref<boolean>(false);
const handleChangeSwitch = () => {
  disabledSwitch.value = true; // 先设置按钮禁用
  setTimeout(() => { // 1s 后解除
    disabledSwitch.value = false;
  }, 1000);
  if (start_stop.value === "" || target_stop.value === "") return;
  else {
    get_the_path();
  }
};

const get_the_path = async () => {
  if (start_stop.value === "" || target_stop.value === "") return;
  const result = ref<string>("");
  loadingBar.start();
  if (!my_switch.value) {
    result.value = await invoke("search_the_shortest_path", {
      start: start_stop.value,
      target: target_stop.value,
    }).catch((err) => console.log(err));
  } else {
    result.value = await invoke("search_the_min_transfer_path", {
      start: start_stop.value,
      target: target_stop.value,
    }).catch((err) => console.log(err));
  }
  // console.log(result);
  bus_path.value = JSON.parse(result.value);
  loadingBar.finish();
  // console.log(bus_path.value);
};

const railStyle = ({
  focused,
  checked,
}: {
  focused: boolean;
  checked: boolean;
}) => {
  const style: CSSProperties = {};
  if (checked) {
    style.background = "#4b9e5f";
    if (focused) {
      style.boxShadow = "0 0 0 2px #dbecdfff";
    }
  } else {
    style.background = "#2080f0";
    if (focused) {
      style.boxShadow = "0 0 0 2px #2080f040";
    }
  }
  return style;
};
</script>

<template>
  <div class="container">
    <p>分别输入起点和终点～</p>
    <div class="select-input-box">
      <n-select
        filterable
        v-model:value="start_stop"
        placeholder="起始站"
        :options="options"
        @update:value="get_the_path"
        class="select-input"
      >
        <template #arrow>
          <BusOutline />
        </template>
      </n-select>
      <n-icon size="25" class="my-icon" depth="3">
        <ArrowForward />
      </n-icon>
      <n-select
        filterable
        v-model:value="target_stop"
        placeholder="终到站"
        :options="options"
        @update:value="get_the_path"
        class="select-input"
      >
        <template #arrow>
          <BusOutline />
        </template>
      </n-select>
    </div>
    <n-switch
      v-model:value="my_switch"
      @update:value="handleChangeSwitch"
      class="my-switch"
      :rail-style="railStyle"
      :disabled="disabledSwitch"
    >
      <template #checked> 最少换乘优先 </template>
      <template #unchecked> 最小站数优先 </template>
    </n-switch>
    <n-card
      ref="loadingBarTargetRef"
      class="show-path"
      hoverable
      :title="'共经过 ' + bus_path.length + ' 站'"
    >
      <n-timeline>
        <n-timeline-item
          v-for="(stop, index) in bus_path.path_vec"
          :key="index"
          color="#dc2533"
          :title="stop[0]"
          :content="stop[1]"
        />
        <n-timeline-item color="#dc2533" title="到达" :content="target_stop" />
      </n-timeline>
    </n-card>
  </div>
</template>

<style scoped>
.container {
  padding: 15px;
}

.show-path {
  height: 60vh;
  overflow: auto;
  margin-top: 15px;
}

.select-input-box {
  display: inline-block;
}

.select-input {
  width: 20vw;
  display: inline-block;
}

.my-icon {
  margin: 0 30px;
}

.show-path {
  height: 60vh;
  overflow: auto;
  margin-top: 15px;
}

.my-switch {
  margin-top: 15px;
}
</style>

<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import Menu from "./components/Menu.vue";

// const stops_name = ref({});

// const get_stops_name = async () => {
//   const result = await invoke("search_stops_name", { keyword: "西口" }).catch(
//     (err) => console.log(err)
//   );
//   stops_name.value = JSON.parse(result);
//   // console.log(stops_name.value)
// };

// routers
import About from "./pages/About.vue";
import LineToStops from "./pages/LineToStops.vue";
import StopToLines from "./pages/StopToLines.vue";
import ShowPath from "./pages/ShowPath.vue";
import NotFound from "./pages/NotFound.vue";

const routes = {
  "/": ShowPath,
  "/about": About,
  "/line_to_stops": LineToStops,
  "/stop_to_lines": StopToLines,
};
const currentPath = ref(window.location.hash);
window.addEventListener("hashchange", () => {
  currentPath.value = window.location.hash;
});
const currentView = computed(() => {
  return routes[currentPath.value.slice(1) || "/"] || NotFound;
});
</script>

<template>
  <n-loading-bar-provider>
    <div class="container">
      <n-split
        direction="horizontal"
        style="height: 100vh"
        max="300px"
        min="260px"
        default-size="260px"
      >
        <template #1>
          <Menu />
        </template>
        <template #2>
          <component :is="currentView" />
        </template>
      </n-split>
    </div>
  </n-loading-bar-provider>
</template>

<style scoped></style>

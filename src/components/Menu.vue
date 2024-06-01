<template>
  <n-menu :options="menuOptions" />
</template>

<style scoped></style>

<script setup lang="ts">
import { defineComponent, h, Component } from "vue";
import { NIcon, useMessage } from "naive-ui";
import type { MenuOption } from "naive-ui";
import {
  InformationCircleOutline as AboutIcon,
  LocationOutline as StopsIcon,
  ReorderFourOutline as LinesIcon,
  BusOutline as PathIcon,
} from "@vicons/ionicons5";

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

const menuOptions: MenuOption[] = [
  {
    label: () =>
      h(
        "a",
        {
          href: "#/",
        },
        "查找站数最少换乘路径"
      ),
    key: "show-path",
    icon: renderIcon(PathIcon),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/line_to_stops",
        },
        "查看某条线路全部站点"
      ),
    key: "line-to-stops",
    icon: renderIcon(StopsIcon),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/stop_to_lines",
        },
        "查看某个站点停靠所有线路"
      ),
    key: "stop-to-lines",
    icon: renderIcon(LinesIcon),
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/about",
        },
        "关于作者和其他信息"
      ),
    key: "about",
    icon: renderIcon(AboutIcon),
  },
];
</script>

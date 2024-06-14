<script setup></script>

<template>
  <div>
    <n-card title="Beijing Bus Transfer System 北京公交换乘系统" hoverable>
      <h2 id="-">简介</h2>
      <p>
        基于 Tauri 框架构建应用，前端使用 Vite + Vue，后端使用 Rust 进行调用
        sqlite3 数据库，并进行推荐从 A 站到 B 站的换乘线路。
      </p>
      <p>同时使用 python 进行爬取北京公交官网数据。</p>
      <p>
        系 USTB
        物联网工程大三下人工智能与机器学习大作业——公交换乘系统设计与开发（2
        人组）
      </p>
      <h2 id="-">功能</h2>
      <ol>
        <li>
          根据用户输入的起点站和终点站找到最短路径（可选站数最少，不是路程或时间，或者换乘最少，根据按钮来切换）。
        </li>
        <li>根据用户输入并选择某条线路查看某条线路全部站点。</li>
        <li>根据用户输入并选择某站查看该站所有的线路。</li>
        <li>上述 2 和 3 可以点击对应的内容跳转到对应的线路/站点的详细信息。</li>
        <li>因为查询耗时，所以添加了“焦虑的安慰剂，疗效尚可”的加载条。</li>
        <li>为了防止用户疯狂点击 Switch 按钮导致后台并行计算占用过多资源，添加了 1s 的按钮禁用。</li>
      </ol>
      <h2 id="-">关于数据</h2>
      <p>
        从北京公交官网首页爬取(2024 年 5
        月)，且该数据已经过期。不具有事实上的参考意义。
      </p>
      <p>
        可以考虑使用北京公交实时公交的接口。但是那里不能一次性获得全部线路。
      </p>
      <p>问题：由于数据没有坐标信息，无法使用 Astar 算法来有指向性的计算最短路径。</p>
      <h2 id="github-">Github仓库</h2>
      <p>
        <a href="https://github.com/CakeAL/beijing-bus-transfer-system"
          >https://github.com/CakeAL/beijing-bus-transfer-system</a
        >
      </p>
      <h2 id="-">作者</h2>
      <p><a href="https://github.com/CakeAL/">@CakeAL</a></p>
    </n-card>
  </div>
</template>

<style scoped>
.n-card {
  max-width: 500px;
  margin: 15px;
  height: 93vh;
  overflow: scroll;
}

</style>

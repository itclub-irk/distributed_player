<script setup lang="ts">
import NodeList from '../components/NodeList.vue'
</script>
<template>
  <div class="row">
    <div class="col-3 sidebar">
      <NodeList></NodeList>
    </div>
    <div class="col main">
      <RouterView />
    </div>
  </div>
</template>
<style scoped></style>

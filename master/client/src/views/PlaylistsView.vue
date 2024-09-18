<script setup lang="ts">
import NodeList from '../components/NodeList.vue'
</script>
<template>
  <div class="row">
    <div class="col-3 sidebar">
      <NodeList></NodeList>
    </div>
    <div class="col main">
      <router-view v-slot="{ Component }">
        <transition mode="out-in" :appear="true">
          <component :is="Component" :key="$route.fullPath" />
        </transition>
      </router-view>
    </div>
  </div>
</template>
<style scoped></style>

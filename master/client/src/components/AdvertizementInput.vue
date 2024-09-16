<script setup lang="ts"></script>
<template>
  <h4>Рекламный блок №1</h4>
  <div class="row">
    <div class="col">Время начала воспроизведения, мин.</div>
  </div>
  <div class="row">
    <div class="col">
      <input type="number" />
    </div>
  </div>
  <p>Директории</p>
  <div class="row">
    <div class="col">
      <p class="grouped">
        <input type="text" value="/реклама_общая/блок_10" />
        <button type="button" class="button error">Удалить</button>
      </p>
    </div>
  </div>
  <div class="row">
    <div class="col">
      <button type="button" class="button primary outline">Добавить директорию</button>
    </div>
    <div class="col is-right">
      <button type="button" class="button error">Удалить блок</button>
    </div>
  </div>
</template>
<style scoped></style>

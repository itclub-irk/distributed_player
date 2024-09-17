<script setup lang="ts"></script>
<template>
  <h4>Интервал №1</h4>
  <div class="row">
    <div class="col">Дата начала</div>
    <div class="col">Дата окончания</div>
  </div>
  <div class="row">
    <div class="col">
      <input type="date" />
    </div>
    <div class="col">
      <input type="date" />
    </div>
  </div>
  <p>Директории</p>
  <div class="row">
    <div class="col">
      <p class="grouped">
        <input type="text" value="/music_1" />
        <button type="button" class="button error">Удалить</button>
      </p>
      <p class="grouped">
        <input type="text" value="/music_2" />
        <button type="button" class="button error">Удалить</button>
      </p>
    </div>
  </div>
  <div class="row">
    <div class="col">
      <button type="button" class="button primary outline">Добавить директорию</button>
    </div>
    <div class="col is-right">
      <button type="button" class="button error">Удалить интервал</button>
    </div>
  </div>
</template>
<style scoped></style>

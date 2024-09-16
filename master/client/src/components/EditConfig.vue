<script setup lang="ts">
import ScheduleInput from '../components/ScheduleInput.vue'
import ScheduleExceptionInput from '../components/ScheduleExceptionInput.vue'
import ScheduleMusicIntervalInput from '../components/ScheduleMusicIntervalInput.vue'
import AdvertizementInput from './AdvertizementInput.vue'
</script>
<template>
  <form class="config_form">
    <div class="row">
      <div class="col-8">
        <h2>Общее расписание</h2>
      </div>
    </div>
    <div class="row">
      <div class="col-8">
        <h3>Режим работы</h3>
        <p>Укажите в форме ниже режим работы плеера для каждого дня недели.</p>
        <div class="config_form__schedule">
          <div class="row">
            <div class="col">
              <ScheduleInput label="Понедельник"></ScheduleInput>
            </div>
            <div class="col">
              <ScheduleInput label="Вторник"></ScheduleInput>
            </div>
          </div>
          <div class="row">
            <div class="col">
              <ScheduleInput label="Среда"></ScheduleInput>
            </div>
            <div class="col">
              <ScheduleInput label="Четверг"></ScheduleInput>
            </div>
          </div>
          <div class="row">
            <div class="col">
              <ScheduleInput label="Пятница"></ScheduleInput>
            </div>
            <div class="col">
              <ScheduleInput label="Суббота"></ScheduleInput>
            </div>
          </div>
          <div class="row">
            <div class="col">
              <ScheduleInput label="Воскресенье"></ScheduleInput>
            </div>
            <div class="col"></div>
          </div>
        </div>
      </div>
    </div>
    <div class="row">
      <div class="col-8">
        <h3>Исключения из режима работы</h3>
        <p>Добавьте исключения, чтобы определить режим работы плеера для определенной даты.</p>
        <div class="row">
          <div class="col">
            <label>Дата</label>
          </div>
          <div class="col">
            <label>Время начала</label>
          </div>
          <div class="col">
            <label>Время окончания</label>
          </div>
          <div class="col"></div>
        </div>

        <ScheduleExceptionInput></ScheduleExceptionInput>
        <ScheduleExceptionInput></ScheduleExceptionInput>
        <ScheduleExceptionInput></ScheduleExceptionInput>
        <button type="button" class="button primary outline">Добавить</button>
      </div>
    </div>
    <div class="row">
      <div class="col-8">
        <h3>Воспроизведение музыки</h3>
        <p>
          При включении режима перемешивания каждый раз для воспроизведения будет выбираться
          случайный файл.
        </p>

        <div class="row">
          <div class="col">
            <p class="grouped">
              <button type="button" class="button primary">Воспроизводить по порядку</button>
              <button type="button" class="button outline">Перемешать файлы</button>
            </p>
          </div>
        </div>
        <p>
          Добавьте интервалы воспроизведения и директории с музыкальными файлами для каждого
          интервала. Интервал с годом 1970 будет действовать ежегодно.
        </p>
        <div class="row">
          <div class="col">
            <ScheduleMusicIntervalInput></ScheduleMusicIntervalInput>
          </div>
        </div>
        <div class="row">
          <div class="col">
            <button type="button" class="button primary outline">Добавить интервал</button>
          </div>
        </div>
      </div>
    </div>
    <div class="row">
      <div class="col-8">
        <h3>Рекламные блоки</h3>
        <p>
          Добавьте рекламные блоки, которые будут воспроизводиться каждый час в течении рабочего
          времени. Рекламный блок будет начинаться в указанное количество минут текущего часа.
        </p>
        <AdvertizementInput></AdvertizementInput>
        <h4>Джинглы</h4>
        <div class="row">
          <div class="col">
            <label>Начальный</label>
            <p class="grouped">
              <input type="text" value="/джинглы/начало.mp3" />
              <button type="button" class="button error">Очистить</button>
            </p>
          </div>
        </div>
        <div class="row">
          <div class="col">
            <label>Конечный</label>
            <p class="grouped">
              <input type="text" value="/джинглы/окончание.mp3" />
              <button type="button" class="button error">Очистить</button>
            </p>
          </div>
        </div>
      </div>
    </div>
  </form>
</template>
<style scoped></style>
./PlaylistWorkingHoursScheduleInput.vue

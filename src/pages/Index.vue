<template>
  <q-page class="flex flex-center column">
    <q-card style="width:480px">
      <q-card-section class="row justify-center">
        <h3 class="row text-center">{{ msg }}</h3>
      </q-card-section>
      <q-separator />
      <q-card-section>
        <div class="row justify-center q-gutter-sm">
          <q-input outlined dense v-model="a" style="width: 40%" type="number"></q-input>
          <q-select outlined dense v-model="operator" :options="operations" />
          <q-input outlined dense v-model="b" style="width: 40%" type="number"></q-input>
        </div>
      </q-card-section>
      <q-separator />
      <q-card-actions align="right">
        <q-btn dense label="evaluate" @click="eventToRust" />
      </q-card-actions>
    </q-card>
  </q-page>
</template>

<script>
import tauri from 'tauri/api'

export default {
  name: 'HelloWorld',
  data () {
    return {
      msg: 'waiting for rust',
      a: null,
      b: null,
      operator: '+',
      operations: ['+', '*']
    }
  },
  mounted () {
    tauri.listen('reply', res => {
      this.msg = res.payload.msg
    })
  },
  methods: {
    // set up an event listener
    eventToRust (a, b) {
      tauri.emit('add', `${this.a}${this.operator}${this.b}`)
    }
  }
}
</script>

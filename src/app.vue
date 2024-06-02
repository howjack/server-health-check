<template>
  <main class="grid gap-3">
    <section>
      <div v-for="i in apis">
        <code v-if="i" class="relative rounded bg-muted px-[0.3rem] py-[0.2rem] font-mono text-sm font-semibold">
        <span>{{ i }}</span>
      </code>
      </div>
    </section>

    <section class="flex gap-2">
      <UiButton class="px-8 py-2" @click="add">
        Adicionar
      </UiButton>
      <UiButton class="px-8 py-2" @click="list">
        Listar
      </UiButton>
      <UiButton class="px-8 py-2" @click="get_api_by_id">
        Pegar um
      </UiButton>
      <UiButton class="px-8 py-2" @click="edit">
        Editar
      </UiButton>
      <UiButton class="px-8 py-2" @click="remove">
        Deletar
      </UiButton>
    </section>
  </main>
</template>

<script setup>
import { invoke } from '@tauri-apps/api'

const colorMode = useColorMode()

const apis = ref()

colorMode.preference = "dark"

async function add() {
  const data = {
    name: 'Pepe22',
    url: 'pepe.nem22'
  }

  await invoke('add_apis', { data })
}

async function list() {
  const data = await invoke('list_apis')
  console.log(data)

  apis.value = data
}

async function get_api_by_id() {
  const data = await invoke('get_api_by_id', { id: 3 })
  console.log(data)

  apis.value = data
}

async function edit() {
  const data = {
    id: 1,
    name: "Dale99",
    url: "github.com",
    method: "POST",
    color_hex: null,
    status: true
  }

  await invoke('edit_api', { data })
}

async function remove() {
  const data = await invoke('delete_api', { id: 10 })
}
</script>

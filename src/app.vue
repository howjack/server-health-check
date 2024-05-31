<template>
  <div>
    {{ opa }}
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api'

const colorMode = useColorMode()

colorMode.preference = "dark"

const data = {
  name: 'Pepe22',
  url: 'pepe.nem22'
}

// await invoke('add_apis', { data })

const opa = await invoke('list_apis')
</script>

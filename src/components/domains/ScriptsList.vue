<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'

interface Props {
  scripts: Record<string, string>
  readonly currentDir: string
}

interface Emit {
  (e: 'execute', text: string): void
}

const props = defineProps<Props>()

const emit = defineEmits<Emit>()

async function executeScript(command: string) {
  // コマンド実行前に標準出力をクリアしておく
  emit('execute', '')
  await listen<{ text: string }>('on-print-stdout', (event) => {
    console.log(event.payload.text)
    emit('execute', event.payload.text)
  })
  await invoke('execute_script', { currentDir: props.currentDir, script: command })
}
</script>

<template>
  <ul>
    <template v-for="(script, command) in scripts" :key="command">
      <li>
        <div class="card" @click="executeScript(command)">
          <p class="card__title">{{ command }}</p>
          <p class="card__script">{{ script }}</p>
        </div>
      </li>
    </template>
  </ul>
</template>

<style>
.card {
  @apply bg-[#eeeeee] px-3 py-2 rounded mb-2 hover:bg-[#dddddd] hover:cursor-pointer;
}

.card__title {
  @apply font-semibold;
}

.card__script {
  @apply ml-5;
}
</style>

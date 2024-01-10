<script setup lang="ts">
import { computed } from 'vue'
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

const isExecuting = ref(false)

// コマンド:終了ステータスのハッシュマップ
// コマンド実行のたびにコマンドと終了ステータスを更新する
const exitStatuses = ref<Record<string, number>>({})

async function executeScript(command: string) {
  isExecuting.value = true

  // コマンド実行前に標準出力をクリアしておく
  emit('execute', '')
  await listen<{ text: string }>('on-print-stdout', (event) => {
    emit('execute', event.payload.text)
  })

  try {
    const result: { code: number } = await invoke('execute_async', { currentDir: props.currentDir, script: command })
    exitStatuses.value[command] = result.code
  } catch (e) {
    exitStatuses.value[command] = 1
  }

  isExecuting.value = false
}

function toIcon(code: number): { name: string, color: string } {
  return code === 0 ? {
    name: 'mdi:check',
    color: '#22C55E'
  } : {
    name: 'mdi:alert-circle-outline',
    color: '#EF4444'
  }
}

const classes = computed(() => ({
  'card__button--disabled': isExecuting.value
}))
</script>

<template>
  <ul>
    <template v-for="(script, command) in scripts" :key="command">
      <li>
        <div class="card">
          <button
            class="card__button"
            @click="executeScript(command)"
            :disabled="isExecuting"
            :class="classes">
            <p class="card__title">{{ command }}</p>
            <p class="card__script">{{ script }}</p>
            <template v-if="exitStatuses[command] != undefined">
              <div class="card__status">
                <Icon
                  :name="toIcon(exitStatuses[command]).name"
                  :color="toIcon(exitStatuses[command]).color"
                  />
              </div>
            </template>
          </button>
        </div>
      </li>
    </template>
  </ul>
</template>

<style>
.card {
  @apply bg-[#eeeeee] rounded mb-2 hover:bg-[#dddddd] hover:cursor-pointer;
}

.card__button {
  @apply w-full px-3 py-2;
}

.card__button--disabled {
  @apply bg-[#dddddd] text-[#666666];
}

.card__title {
  @apply font-semibold text-left;
}

.card__script {
  @apply ml-5 text-left;
}

.card__status {
  @apply text-right;
}
</style>

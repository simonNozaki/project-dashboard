<script setup lang="ts">
interface Props {
  stdout: string
}

const _ = defineProps<Props>()

/**
 * バックエンドからは改行コード含めて来ているが、表示時にマークアップとして縦に列挙したいので分割する
 */
function toLines(text: string): string[] {
  let start = 0
  const lines = []
  for (let i = 0; i < text.length; i++) {
    if (text[i] === '\n') {
      const line = text.substring(start, i)
      lines.push(line)
      // 前回区切り位置を更新
      start = i
    }
  }
  return lines
}
</script>

<template>
  <ul class="scripts__stdout">
    <template v-for="(line, i) in toLines(stdout)" :key="i">
      <li>
        <span class="scripts__stdout_text">
          {{ line }}
        </span>
      </li>
    </template>
  </ul>
</template>

<style>
.scripts__stdout {
  @apply bg-blue-tomorrow-night flex-1 ml-8 rounded w-full overflow-y-scroll;
}

.scripts__stdout_text {
  @apply p-2 text-white font-mono;
}
</style>

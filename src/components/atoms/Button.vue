<script setup lang="ts">
</script>

<template>
  <button type="button" class="button">
    <slot />
  </button>
</template>

<style scoped>
/* TODO: ボタンのバリエーションが必要になったら動的なクラス定義を追加 */
.button {
  @apply bg-blue-mazarine font-semibold text-white rounded px-4 py-3 hover:opacity-80;
}
</style>

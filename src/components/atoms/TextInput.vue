<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  modelValue: string
  placeholder?: string
}

interface Emit {
  (e: 'update:modelValue', newValue: string): void
}

const props = defineProps<Props>()

const emit = defineEmits<Emit>()

const value = computed({
  get: () => props.modelValue,
  set: (newValue: string) => emit('update:modelValue', newValue)
})
</script>

<template>
  <input
    type="text"
    v-model="value"
    :placeholder="placeholder" />
</template>

<style scoped>
input {
  @apply p-2 rounded bg-[#eeeeee];
}
</style>

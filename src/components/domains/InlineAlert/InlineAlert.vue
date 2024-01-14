<script setup lang="ts">
import { computed } from 'vue'
import type { AlertType, InlineAlert } from './privates/alert';

interface Props {
  alert: InlineAlert
}

const props = defineProps<Props>()

const iconNames = {
  success: 'mdi:check',
  error: 'mdi:alert-circle-outline'
} satisfies Record<AlertType, string>

const iconColors = {
  success: '#22C55E',
  error: '#EF4444'
} satisfies Record<AlertType, string>

const iconName = computed(() => iconNames[props.alert.status])
const iconColor = computed(() => iconColors[props.alert.status])
</script>

<template>
  <p class="header__alert">
    <Icon
      :name="iconName"
      :color="iconColor"
      />
    <span class="header__alert_text">{{ alert.message }}</span>
  </p>
</template>

<style>
.header__alert {
  @apply mt-2;
}

.header__alert_text {
  @apply text-gray-3 text-sm;
}
</style>

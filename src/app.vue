<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import type { InlineAlert } from '~/components/domains/InlineAlert/privates/alert';

const defaultProjectName = '無題のプロジェクト'
const projectName = ref(defaultProjectName)
// プロジェクトメタ情報を読み込めたかを管理する
const isNpmProject = ref(false)

interface ProjectMeta {
  name: string,
  scripts: Record<string, string>
}

const dir = ref('')
const scripts = ref<Record<string, string>>({})
const stdout = ref('')
const alert = ref<InlineAlert>()

type ErrorType = '$error_file_not_found'

function isErrorType(v?: unknown): v is ErrorType {
  if (typeof v === 'string') {
    return new Set(['$error_file_not_found']).has(v)
  }
  return false
}

const alertMap = {
  '$error_file_not_found': 'package.jsonを読み込めませんでした ... 指定されたディレクトリにはpackage.jsonが存在しません'
} satisfies Record<ErrorType, string>

/**
 * ディレクトリのpackage.jsonを読み込み、結果に応じて状態を更新する
 * - 成功: プロジェクトメタ情報の設定
 * - 失敗: エラーメッセージの設定、入力前のプロジェクトメタ情報のフラッシュ
 */
async function updateProjectMeta(dir: string) {
  try {
    const result: ProjectMeta = await invoke('set_npm_project', { dir })
    projectName.value = result.name
    scripts.value = result.scripts
    alert.value = {
      status: 'success',
      message: 'npmプロジェクトを読み込みました'
    }
    isNpmProject.value = true
  } catch (e) {
    // バックエンドからは定数が返ってくるはずだが一応型チェックして正確にキャストしておく
    const message = isErrorType(e) ? alertMap[e] : 'package.jsonの読み込み中にエラーが発生しました'
    alert.value = {
      status: 'error',
      message
    }
    projectName.value = defaultProjectName
    scripts.value = {}
    isNpmProject.value = false
  }
}

function resetProjectMeta() {
  projectName.value = defaultProjectName
  dir.value = ''
  scripts.value = {}
  isNpmProject.value = false
  stdout.value = ''

  alert.value = {
    status: 'success',
    message: 'npmプロジェクトをクリアしました'
  }
}

const dirUpdated = computed<string>({
  get: () => dir.value,
  set: async (newValue: string) => {
    dir.value = newValue;
    if (dir.value === '') {
      resetProjectMeta()
    } else {
      updateProjectMeta(dir.value)
    }
  }
})
</script>

<template>
  <div class="main">
    <h1 class="header__title">
      {{ projectName }}
    </h1>
    <template v-if="isNpmProject">
      <button
        class="header__project_dir_button"
        @click="resetProjectMeta">
        <span class="header__project_dir_button_text">{{ dir }}</span>
        <Icon name="mdi:close-circle-outline" color="#666666" />
      </button>
    </template>
    <template v-else>
      <TextInput
        class="header__project_dir"
        :model-value="dir"
        placeholder="npmプロジェクトを入力..."
        @update:model-value="dirUpdated = $event" />
    </template>
    <template v-if="alert">
      <InlineAlert :alert="alert" />
    </template>
    <div class="scripts">
      <ScriptsList
        :scripts="scripts"
        :current-dir="dir"
        @execute="stdout = $event" />
      <ScriptsStdout :stdout="stdout" />
    </div>
  </div>
</template>

<style scoped>
.main {
  @apply p-10 h-[100vh] overflow-y-auto;
}

.header__title {
  @apply font-semibold;
}

.header__project_dir {
  @apply w-full mt-2;
}

.header__project_dir_button {
  @apply mt-2 rounded p-2 bg-[#eeeeee] text-[#666666] hover:bg-[#dddddd];
}

.header__project_dir_button_text {
  @apply mr-1;
}

.scripts {
  @apply flex flex-row mt-10 h-full;
}
</style>

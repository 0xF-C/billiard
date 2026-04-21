<template>
  <div class="login-page">
    <div class="bg-gradient"></div>
    <div class="bg-grid"></div>

    <div class="login-container">
      <div class="login-card">
        <div class="login-header">
          <div class="logo">
            <div class="logo-icon">🎱</div>
            <h1>Billiard</h1>
          </div>
          <p class="subtitle">{{ t('loginTitle') }}</p>
        </div>

        <el-form ref="formRef" :model="form" :rules="rules" class="login-form" @submit.prevent="handleLogin">
          <div class="input-group">
            <el-icon class="input-icon"><User /></el-icon>
            <el-input
              v-model="form.username"
              :placeholder="t('username')"
              size="large"
              class="login-input"
              autofocus
            />
          </div>

          <div class="input-group">
            <el-icon class="input-icon"><Lock /></el-icon>
            <el-input
              v-model="form.password"
              type="password"
              :placeholder="t('password')"
              size="large"
              class="login-input"
              show-password
              @keyup.enter="handleLogin"
            />
          </div>

          <el-button
            type="primary"
            size="large"
            class="login-btn"
            :loading="loading"
            @click="handleLogin"
          >
            <template v-if="!loading">
              <el-icon><Key /></el-icon>
              {{ t('login') }}
            </template>
            <template v-else>
              {{ t('login') }}...
            </template>
          </el-button>
        </el-form>

        <div class="login-footer">
          <div class="lang-switcher">
            <button
              v-for="l in langs"
              :key="l.value"
              :class="['lang-btn', { active: lang === l.value }]"
              @click="changeLang(l.value)"
            >
              {{ l.label }}
            </button>
            <button class="lang-btn db-path-btn" @click="ballClick">🔧</button>
          </div>
          <div class="hint">{{ t('loginHint') }}</div>
        </div>
      </div>

      <div class="decoration">
        <div class="deco-ball ball-1">8</div>
        <div class="deco-ball ball-2">9</div>
        <div class="deco-ball ball-3">①</div>
      </div>
    </div>

    <!-- {{ t('FirstLoginSetupDialog') }} -->
    <el-dialog
      v-model="showSetup"
      :title="t('firstLoginSetup')"
      width="480px"
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      class="setup-dialog"
    >
      <el-form :model="setupForm" label-width="100px">
        <el-form-item :label="t('newUsername')">
          <el-input v-model="setupForm.username" :placeholder="t('username')" />
        </el-form-item>
        <el-form-item :label="t('newPassword')">
          <el-input v-model="setupForm.password" type="password" :placeholder="t('password')" show-password />
        </el-form-item>
        <el-form-item :label="t('confirmPassword')">
          <el-input v-model="setupForm.confirmPassword" type="password" :placeholder="t('confirmPassword')" show-password />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button type="primary" @click="handleSetup">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>

    <!-- {{ t('DBPathDialog') }} -->
    <el-dialog
      v-model="showDbPaths"
      :title="t('dbPathLabel')"
      width="500px"
      class="db-path-dialog"
    >
      <div class="db-path-content">
        <p>{{ dbPath }}</p>
        <el-button size="small" @click="copyDbPath">{{ t('copy') }}</el-button>
      </div>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElForm, ElFormItem, ElInput, ElButton, ElIcon } from 'element-plus'
import { User, Lock, Key } from '@element-plus/icons-vue'
import { setLang, t } from '../i18n'
import api, { getDbPath } from '../api'

const router = useRouter()
const formRef = ref(null)
const loading = ref(false)
const lang = ref('zh')
const form = ref({ username: '', password: '' })

const showSetup = ref(false)
const setupForm = ref({ username: '', password: '', confirmPassword: '' })
const showDbPaths = ref(false)
const dbPath = ref('')

const langs = computed(() => [
  { value: 'zh', label: '中文' },
  { value: 'ug', label: '维语' },
  { value: 'en', label: 'EN' }
])

const changeLang = (l) => {
  lang.value = l
  setLang(l)
}

const rules = computed(() => ({
  username: [{ required: true, message: t('username'), trigger: 'blur' }],
  password: [{ required: true, message: t('password'), trigger: 'blur' }]
}))

const ballClick = async () => {
  dbPath.value = await getDbPath()
  showDbPaths.value = true
}

const copyDbPath = () => {
  navigator.clipboard.writeText(dbPath.value).then(() => {
    ElMessage.success(t('copySuccess'))
  }).catch(() => {
    ElMessage.error(t('copyFailed'))
  })
}

const handleSetup = async () => {
  if (!setupForm.value.username || !setupForm.value.password) {
    return ElMessage.warning(t('pleaseComplete'))
  }
  if (setupForm.value.password !== setupForm.value.confirmPassword) {
    return ElMessage.warning(t('passwordMismatch'))
  }
  try {
    const user = JSON.parse(localStorage.getItem('user') || '{}')
    await api.setupFirstLogin(user.id, setupForm.value.username, setupForm.value.password)
    ElMessage.success(t('setupSuccess'))
    showSetup.value = false
    localStorage.removeItem('token')
    localStorage.removeItem('user')
    form.value = { username: setupForm.value.username, password: '' }
  } catch (e) {
    ElMessage.error(e.message || t('operationFailed'))
  }
}

const handleLogin = async () => {
  try { await formRef.value.validate() } catch { return }
  loading.value = true
  try {
    const res = await api.login(form.value.username, form.value.password)
    if (res.success && res.token) {
      localStorage.setItem('token', res.token)
      localStorage.setItem('user', JSON.stringify(res.user))
      localStorage.setItem('lang', lang.value)
      setLang(lang.value)
      if (res.user.first_login) {
        showSetup.value = true
      } else {
        ElMessage.success(t('loginSuccess'))
        router.push('/')
      }
    } else {
      ElMessage.error(res.message || t('loginError'))
    }
  } catch (e) {
    ElMessage.error(e.message || t('loginError'))
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  const savedLang = localStorage.getItem('lang')
  if (savedLang) {
    lang.value = savedLang
    setLang(savedLang)
  }
})
</script>

<style scoped>
.login-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-primary);
  position: relative;
  overflow: hidden;
}

.bg-gradient {
  position: absolute;
  inset: 0;
  background:
    radial-gradient(ellipse 80% 50% at 50% -20%, rgba(88,166,255,0.12), transparent),
    radial-gradient(ellipse 60% 40% at 80% 100%, rgba(163,113,247,0.08), transparent),
    radial-gradient(ellipse 40% 30% at 20% 80%, rgba(63,185,80,0.06), transparent);
  pointer-events: none;
}

.bg-grid {
  position: absolute;
  inset: 0;
  background-image:
    linear-gradient(rgba(255,255,255,0.015) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255,255,255,0.015) 1px, transparent 1px);
  background-size: 60px 60px;
  pointer-events: none;
}

.login-container {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: 60px;
}

.login-card {
  width: 400px;
  padding: 48px 40px;
  background: rgba(17,24,32,0.88);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-xl), 0 0 0 1px rgba(88,166,255,0.04);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
}

.login-header {
  text-align: center;
  margin-bottom: 36px;
}

.logo {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  margin-bottom: 8px;
}

.logo-icon {
  font-size: 40px;
  animation: bounce 2s ease-in-out infinite;
  filter: drop-shadow(0 0 10px rgba(255,215,0,0.3));
}

@keyframes bounce {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-6px); }
}

.logo h1 {
  font-size: 32px;
  font-weight: 700;
  background: var(--gradient-gold);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  margin: 0;
}

.subtitle {
  color: var(--text-secondary);
  font-size: 14px;
  margin: 0;
}

.login-form { display: flex; flex-direction: column; gap: 16px; }

.input-group {
  position: relative;
  display: flex;
  align-items: center;
}

.input-icon {
  position: absolute;
  left: 16px;
  color: var(--text-secondary);
  font-size: 18px;
  z-index: 1;
  pointer-events: none;
}

.login-input {
  width: 100%;
}

:deep(.el-input__wrapper) {
  background: var(--bg-tertiary) !important;
  padding-left: 48px !important;
  height: 52px;
  border-radius: var(--radius-md) !important;
  box-shadow: none !important;
  border: 1px solid var(--border-default) !important;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast) !important;
}

:deep(.el-input__wrapper:hover) {
  border-color: rgba(88,166,255,0.4) !important;
}

:deep(.el-input.is-focus .el-input__wrapper) {
  border-color: var(--accent-primary) !important;
  box-shadow: 0 0 0 3px rgba(88,166,255,0.1) !important;
}

:deep(.el-input__inner) {
  color: var(--text-primary) !important;
  font-size: 15px !important;
}

:deep(.el-input__inner)::placeholder {
  color: var(--text-tertiary) !important;
}

.login-btn {
  width: 100%;
  height: 52px;
  font-size: 16px;
  font-weight: 600;
  background: var(--gradient-blue) !important;
  border: none !important;
  border-radius: var(--radius-md);
  box-shadow: 0 4px 16px rgba(31,111,235,0.3);
  transition: all var(--transition-normal);
}

.login-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(31,111,235,0.4);
}

.login-btn:active {
  transform: translateY(0);
}

.login-footer {
  margin-top: 24px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
}

.lang-switcher {
  display: flex;
  gap: 4px;
  padding: 4px;
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  border: 1px solid var(--border-default);
}

.lang-btn {
  padding: 8px 16px;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-tertiary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.lang-btn:hover {
  color: var(--text-secondary);
  background: var(--bg-tertiary);
}

.lang-btn.active {
  background: var(--gradient-blue);
  color: #fff;
  box-shadow: 0 2px 6px rgba(31,111,235,0.35);
}

.db-path-btn {
  opacity: 0.5;
  font-size: 14px;
  padding: 8px 10px;
}

.db-path-btn:hover {
  opacity: 1;
}

.hint {
  color: var(--text-tertiary);
  font-size: 12px;
  padding: 6px 12px;
  background: var(--bg-primary);
  border-radius: var(--radius-sm);
  border: 1px solid var(--border-muted);
}

.decoration {
  position: relative;
  width: 200px;
  height: 200px;
}

.deco-ball {
  position: absolute;
  width: 60px;
  height: 60px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  font-weight: 700;
  color: #fff;
  box-shadow:
    inset -4px -4px 8px rgba(0,0,0,0.3),
    inset 4px 4px 8px rgba(255,255,255,0.2),
    0 4px 16px rgba(0,0,0,0.3),
    0 0 30px rgba(0,0,0,0.2);
  cursor: pointer;
  user-select: none;
}

.ball-1 {
  background: linear-gradient(135deg, #1a1a1a, #333);
  top: 20px;
  left: 70px;
}

.ball-2 {
  background: linear-gradient(135deg, #ff6b6b, #ee5a5a);
  top: 80px;
  left: 20px;
  animation: float 3s ease-in-out infinite;
}

.ball-3 {
  background: linear-gradient(135deg, var(--accent-primary), #388bfd);
  top: 100px;
  left: 110px;
  animation: float 3s ease-in-out infinite 1.5s;
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10px); }
}

@media (max-width: 900px) {
  .decoration { display: none; }
}

@media (max-width: 480px) {
  .login-card {
    width: 100%;
    margin: 16px;
    padding: 32px 24px;
  }
}
</style>

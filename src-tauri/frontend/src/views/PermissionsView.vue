<template>
  <div class="permissions-page">
    <div class="page-header">
      <h1 class="page-title">{{ t('permissions') }}</h1>
      <el-button type="primary" :icon="Plus" @click="showAddRole = true">{{ t('addRole') }}</el-button>
    </div>

    <!-- User Management Section -->
    <div class="settings-section">
      <h3 class="section-title">{{ t('userManagement') }}</h3>
      <div class="user-toolbar">
        <el-button type="primary" size="small" @click="openAddUser">
          <el-icon><Plus /></el-icon>
          {{ t('add') }}
        </el-button>
      </div>
      <el-table :data="users" style="width: 100%" size="default">
        <el-table-column prop="username" :label="t('name')" />
        <el-table-column prop="role" :label="t('role')" />
        <el-table-column prop="created_at" :label="t('createdAt')">
          <template #default="{ row }">{{ row.created_at || '-' }}</template>
        </el-table-column>
        <el-table-column :label="t('actions')" width="160">
          <template #default="{ row }">
            <el-button size="small" @click="editUser(row)">{{ t('edit') }}</el-button>
            <el-button size="small" type="danger" @click="deleteUser(row)" :disabled="row.username === 'admin'">{{ t('delete') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <div class="roles-grid">
      <div v-for="role in roles" :key="role.id" class="role-card">
        <div class="role-header">
          <span class="role-name">{{ role.name }}</span>
          <el-tag :type="role.isSystem ? 'primary' : 'default'" size="small">
            {{ role.isSystem ? t('systemRole') : t('customRole') }}
          </el-tag>
        </div>
        <div class="role-desc">{{ role.description }}</div>
        <div class="role-perms">
          <el-tag v-for="p in role.permissions.slice(0, 5)" :key="p" size="small" class="perm-tag">{{ t(p) }}</el-tag>
          <el-tag v-if="role.permissions.length > 5" size="small">+{{ role.permissions.length - 5 }}</el-tag>
        </div>
        <div class="role-users">{{ role.userCount }}{{ t('usersWithRole') }}</div>
        <div class="role-actions">
          <el-button link type="primary" @click="editRole(role)">{{ t('edit') }}</el-button>
          <el-button link type="danger" @click="deleteRole(role)" v-if="!role.isSystem">{{ t('delete') }}</el-button>
        </div>
      </div>
    </div>

    <div class="perm-matrix-section">
      <div class="section-header">
        <h3>{{ t('permissionMatrix') }}</h3>
      </div>
      <el-table :data="permMatrix" border size="small">
        <el-table-column prop="permName" :label="t('permission')" min-width="140" fixed />
        <el-table-column v-for="role in roles.slice(0, 5)" :key="role.id" :label="role.name" width="100" align="center">
          <template #default="{ row }">
            <el-checkbox :model-value="row.perms.includes(role.id)" @change="togglePerm(row, role)" />
          </template>
        </el-table-column>
      </el-table>
    </div>

    <el-dialog v-model="showAddRole" :title="t('addRole')" width="500px" top="5vh" append-to-body>
      <el-form :model="form" label-width="100px">
        <el-form-item :label="t('roleName')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="t('description')">
          <el-input v-model="form.description" type="textarea" :rows="2" />
        </el-form-item>
        <el-form-item :label="t('permissions')">
          <el-checkbox-group v-model="form.permissions">
            <el-checkbox v-for="p in allPermissions" :key="p" :value="p">{{ t(p) }}</el-checkbox>
          </el-checkbox-group>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAdd = false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="submitRole">{{ t('confirm') }}</el-button>
      </template>
    </el-dialog>

    <!-- Add/Edit User Dialog -->
    <el-dialog v-model="showUserForm" :title="editUserTarget ? t('edit') : t('add')" width="400px" top="5vh" append-to-body>
      <div class="form-item">
        <label>{{ t('name') }}</label>
        <el-input v-model="userForm.username" :placeholder="t('name')" />
      </div>
      <div class="form-item">
        <label>{{ editUserTarget ? t('password') + ' (' + t('optional') + ')' : t('password') }}</label>
        <el-input v-model="userForm.password" type="password" show-password :placeholder="t('password')" />
      </div>
      <div class="form-item">
        <label>{{ t('role') }}</label>
        <el-select v-model="userForm.role" style="width: 100%">
          <el-option :label="t('admin')" value="管理员" />
          <el-option :label="t('cashier')" value="收银员" />
          <el-option :label="t('staff')" value="服务员" />
        </el-select>
      </div>
      <template #footer>
        <el-button @click="showUserForm=false">{{ t('cancel') }}</el-button>
        <el-button type="primary" @click="saveUser">{{ t('save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { Plus } from '@element-plus/icons-vue'
import { t } from '../i18n'
import { getUsers, createUser, updateUser, deleteUser as delUser } from '../api'

const showAddRole = ref(false)
const showUserForm = ref(false)
const editUserTarget = ref(null)
const users = ref([])

const form = reactive({ name: '', description: '', permissions: [] })
const userForm = reactive({ username: '', password: '', role: '服务员' })

const allPermissions = [
  'tableManage', 'orderManage', 'memberManage', 'recharge', 'pointsManage',
  'inventoryManage', 'stockIO', 'bookingManage', 'financeView', 'financeManage',
  'staffManage', 'attendance', 'performance', 'salary',
  'reportView', 'reportExport', 'settings', 'userManage',
]

const roles = ref([
  { id: 1, name: '管理员', description: '拥有所有权限', isSystem: true, userCount: 2, permissions: allPermissions },
  { id: 2, name: '店长', description: '日常经营管理权限', isSystem: true, userCount: 3, permissions: allPermissions.filter((_, i) => i !== 15 && i !== 16) },
  { id: 3, name: '收银员', description: '收银及订单操作权限', isSystem: true, userCount: 5, permissions: ['tableManage', 'orderManage', 'recharge', 'bookingManage'] },
  { id: 4, name: '服务员', description: '前台接待及服务权限', isSystem: true, userCount: 8, permissions: ['tableManage', 'bookingManage'] },
])

const permMatrix = [
  { permName: 'tableManage', perms: [1, 2, 3, 4] },
  { permName: 'orderManage', perms: [1, 2, 3] },
  { permName: 'memberManage', perms: [1, 2] },
  { permName: 'recharge', perms: [1, 2, 3] },
  { permName: 'inventoryManage', perms: [1, 2] },
  { permName: 'bookingManage', perms: [1, 2, 3, 4] },
  { permName: 'financeView', perms: [1, 2, 3] },
  { permName: 'reportView', perms: [1, 2] },
  { permName: 'settings', perms: [1, 2] },
  { permName: 'userManage', perms: [1] },
]

const togglePerm = (row, role) => {
  const idx = row.perms.indexOf(role.id)
  if (idx === -1) row.perms.push(role.id)
  else row.perms.splice(idx, 1)
  ElMessage.success(t('saveSuccess'))
}

const submitRole = () => {
  if (!form.name) { ElMessage.warning(t('pleaseComplete')); return }
  roles.value.push({ id: Date.now(), ...form, isSystem: false, userCount: 0 })
  ElMessage.success(t('addSuccess'))
  showAddRole.value = false
}

const editRole = (role) => ElMessage.info(`${t('edit')}: ${role.name}`)
const deleteRole = async (role) => {
  await ElMessageBox.confirm(t('confirmDelete'), t('confirm'), { type: 'warning' })
  const idx = roles.value.findIndex(r => r.id === role.id)
  if (idx !== -1) { roles.value.splice(idx, 1); ElMessage.success(t('deleteSuccess')) }
}

const loadUsers = async () => {
  try {
    users.value = await getUsers()
  } catch (e) {
    users.value = [{ id: 1, username: 'admin', role: '管理员', created_at: '2024-01-01' }]
  }
}

const openAddUser = () => {
  editUserTarget.value = null
  userForm.username = ''
  userForm.password = ''
  userForm.role = '服务员'
  showUserForm.value = true
}

const editUser = (user) => {
  editUserTarget.value = user
  userForm.username = user.username
  userForm.password = ''
  userForm.role = user.role
  showUserForm.value = true
}

const saveUser = async () => {
  if (!userForm.username) return ElMessage.warning(t('pleaseComplete'))
  try {
    if (editUserTarget.value) {
      const data = { username: userForm.username, role: userForm.role }
      if (userForm.password) data.password = userForm.password
      await updateUser(editUserTarget.value.id, data)
      ElMessage.success(t('updateSuccess'))
    } else {
      if (!userForm.password) return ElMessage.warning(t('pleaseComplete'))
      await createUser(userForm)
      ElMessage.success(t('createSuccess'))
    }
    showUserForm.value = false
    await loadUsers()
  } catch (e) {
    ElMessage.error(e.response?.data?.error || t('operationFailed'))
  }
}

const deleteUser = async (user) => {
  try {
    await ElMessageBox.confirm(t('confirmDelete'), t('confirm'), { type: 'warning' })
    await delUser(user.id)
    ElMessage.success(t('deleteSuccess'))
    await loadUsers()
  } catch (e) {
    if (e !== 'cancel') ElMessage.error(t('operationFailed'))
  }
}

onMounted(() => {
  loadUsers()
})
</script>

<style scoped>
.permissions-page { padding: 24px; }
.page-title { font-size: 24px; font-weight: 600; margin: 0; }
.page-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 24px; }

.roles-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.role-card {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.role-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.role-name { font-weight: 600; font-size: 16px; }
.role-desc { font-size: 13px; color: var(--text-secondary); margin-bottom: 12px; }

.role-perms {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  margin-bottom: 12px;
}

.perm-tag { margin-bottom: 2px; }

.role-users { font-size: 12px; color: var(--text-secondary); margin-bottom: 8px; }
.role-actions { display: flex; gap: 8px; }

.perm-matrix-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
}

.section-header { margin-bottom: 16px; }
.section-header h3 { margin: 0; font-size: 16px; font-weight: 600; }

.settings-section {
  background: var(--card-bg);
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border-color);
  margin-bottom: 24px;
}
.section-title { font-size: 14px; font-weight: 600; margin: 0 0 16px 0; text-transform: uppercase; }
.user-toolbar { margin-bottom: 12px; }

.form-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 16px;
}
.form-item label { font-size: 13px; color: var(--text-secondary); }
</style>

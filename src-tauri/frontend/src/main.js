import { createApp, computed, ref, watch } from "vue";
import { createPinia } from "pinia";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import "./style.css";
import zhCn from "element-plus/dist/locale/zh-cn.mjs";
import en from "element-plus/dist/locale/en.mjs";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";
import router from "./router";
import App from "./App.vue";
import { currentLang, lang } from "./i18n";

const locales = { zh: zhCn, en, ug: zhCn }; // Use zhCn as fallback for Uyghur

const app = createApp(App);
const pinia = createPinia();

for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}

app.use(pinia);
app.use(router);

// Create reactive locale for Element Plus
const elementLocale = computed(() => {
  return locales[lang.value] || zhCn;
});

// Watch for language changes and update Element Plus locale
watch(() => lang.value, (newLang) => {
  // Update Element Plus locale by accessing the global config
  if (app.config.globalProperties.$ELEMENT) {
    app.config.globalProperties.$ELEMENT.locale = locales[newLang] || zhCn;
  }
}, { immediate: true });

app.use(ElementPlus, { locale: elementLocale.value });

// Provide the reactive locale to all components
app.provide('elementLocale', elementLocale);

app.mount("#app");

currentLang.value = lang.value;
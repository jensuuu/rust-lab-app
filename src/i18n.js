// Tiny translation layer.
//
// Wrap any text in t("English text") and the matching entry from the active
// locale is returned. Text without an entry is shown unchanged, so nothing
// breaks while a translation is still missing.
//
// In HTML, add data-i18n to an element to translate its text. Use
// data-i18n-placeholder, data-i18n-aria-label or data-i18n-title for attributes.
//
// Add a language: create src/locales/<code>.js that calls
// I18n.add("<code>", { "English text": "Translation" }), add a <script> tag for
// it in index.html and add the code to LANGUAGES below.

const I18n = (() => {
  const STORAGE_KEY = "rust-academy:lang";
  const DEFAULT_LANGUAGE = "en";
  const LANGUAGES = ["en", "et"];
  const dictionaries = {};
  const sources = new WeakMap();
  let current = DEFAULT_LANGUAGE;

  function add(language, entries) {
    dictionaries[language] = { ...dictionaries[language], ...entries };
  }

  function detect() {
    try {
      const saved = localStorage.getItem(STORAGE_KEY);

      if (LANGUAGES.includes(saved)) {
        return saved;
      }
    } catch {
      // Storage can be unavailable, fall through to the system language.
    }

    const system = (navigator.language || DEFAULT_LANGUAGE).slice(0, 2).toLowerCase();
    return LANGUAGES.includes(system) ? system : DEFAULT_LANGUAGE;
  }

  function translate(text, values = {}) {
    const entry = dictionaries[current]?.[text] ?? text;
    return entry.replace(/\{(\w+)\}/g, (match, name) => (name in values ? values[name] : match));
  }

  // Remembers the original English of each element so switching language
  // back and forth always translates from the source text.
  function original(node, attribute) {
    const store = sources.get(node) ?? {};
    sources.set(node, store);

    if (!(attribute in store)) {
      store[attribute] = attribute === "text" ? node.textContent.trim() : node.getAttribute(attribute);
    }

    return store[attribute];
  }

  function apply(root = document) {
    root.querySelectorAll("[data-i18n]").forEach((node) => {
      node.textContent = translate(original(node, "text"));
    });

    for (const attribute of ["placeholder", "aria-label", "title"]) {
      root.querySelectorAll(`[data-i18n-${attribute}]`).forEach((node) => {
        node.setAttribute(attribute, translate(original(node, attribute)));
      });
    }
  }

  function set(language) {
    current = LANGUAGES.includes(language) ? language : DEFAULT_LANGUAGE;
    document.documentElement.lang = current;

    try {
      localStorage.setItem(STORAGE_KEY, current);
    } catch {
      return;
    }
  }

  function init() {
    current = detect();
    document.documentElement.lang = current;
    apply();
  }

  return { add, apply, init, set, t: translate, get language() { return current; }, languages: LANGUAGES };
})();

const t = I18n.t;

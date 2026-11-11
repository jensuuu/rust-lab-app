const { invoke } = window.__TAURI__.core;

const STORAGE_KEYS = {
  tab: "rust-academy:tab",
  position: "rust-academy:book-position",
};

const TAB_ACTIVE = "bg-orange-500 text-stone-950";
const TAB_IDLE = "text-stone-300 hover:text-white";
const NAV_ACTIVE = "bg-stone-800 text-orange-300";
const NAV_IDLE = "text-stone-300";
const LANG_ACTIVE = "bg-orange-500 text-stone-950";
const LANG_IDLE = "text-stone-300 hover:text-white";

const STATUS_COLORS = {
  ok: "text-emerald-400",
  error: "text-red-400",
  info: "text-stone-400",
};

const HEADING_CLASSES = {
  2: "pt-6 text-2xl font-bold",
  3: "pt-4 text-xl font-semibold",
  4: "pt-2 text-base font-semibold text-orange-300",
};

const SYMBOLS = ["Tab", "{", "}", "(", ")", "[", "]", "<", ">", ";", ":", "::", "&", "\"", "'", "!", "_", "->", "=>"];

const RUST_TOKENS =
  /(\/\/[^\n]*)|("(?:\\.|[^"\\\n])*")|(\b(?:as|async|await|break|const|continue|crate|dyn|else|enum|extern|false|fn|for|if|impl|in|let|loop|match|mod|move|mut|pub|ref|return|self|Self|static|struct|super|trait|true|type|unsafe|use|where|while)\b)|(\b\d[\d_]*(?:\.\d+)?\b)|(\b[A-Za-z_]\w*!)/g;

const TOKEN_CLASSES = [
  "text-stone-500 italic",
  "text-emerald-300",
  "text-orange-400",
  "text-sky-300",
  "text-amber-300",
];

const state = {
  tab: "lessons",
  lessons: [],
  completed: new Set(),
  currentLessonId: null,
  drafts: new Map(),
  outline: [],
  order: [],
  position: null,
};

const sectionButtons = new Map();
const chapterDetails = new Map();
let searchTimer;

const byId = (id) => document.getElementById(id);

const ui = {
  sidebar: byId("sidebar"),
  backdrop: byId("backdrop"),
  menuButton: byId("menu-button"),
  symbolBar: byId("symbol-bar"),
  langSwitch: byId("lang-switch"),
  bookNote: byId("book-note"),
  tabLessons: byId("tab-lessons"),
  tabBook: byId("tab-book"),
  lessonsPanel: byId("lessons-panel"),
  bookPanel: byId("book-panel"),
  lessonView: byId("lesson-view"),
  bookView: byId("book-view"),
  mainScroll: byId("main-scroll"),
  lessonList: byId("lesson-list"),
  progressBar: byId("progress-bar"),
  progressLabel: byId("progress-label"),
  lessonTitle: byId("lesson-title"),
  lessonSummary: byId("lesson-summary"),
  lessonBody: byId("lesson-body"),
  exampleCode: byId("example-code"),
  loadExample: byId("load-example"),
  exercisePrompt: byId("exercise-prompt"),
  editor: byId("editor"),
  runButton: byId("run-button"),
  checkButton: byId("check-button"),
  resetButton: byId("reset-button"),
  status: byId("status"),
  output: byId("output"),
  searchInput: byId("search-input"),
  bookOutline: byId("book-outline"),
  searchResults: byId("search-results"),
  bookCrumb: byId("book-crumb"),
  bookTitle: byId("book-title"),
  bookContent: byId("book-content"),
  bookPrev: byId("book-prev"),
  bookNext: byId("book-next"),
};

function element(tag, classes = "", text = "") {
  const node = document.createElement(tag);
  node.className = classes;
  node.textContent = text;
  return node;
}

function setActive(node, isActive, activeClasses, idleClasses) {
  const remove = isActive ? idleClasses : activeClasses;
  const add = isActive ? activeClasses : idleClasses;
  node.classList.remove(...remove.split(" "));
  node.classList.add(...add.split(" "));
}

function setDrawer(isOpen) {
  ui.sidebar.classList.toggle("-translate-x-full", !isOpen);
  ui.backdrop.hidden = !isOpen;
}

function closeDrawer() {
  setDrawer(false);
}

function insertAtCursor(text) {
  const insert = text === "Tab" ? "    " : text;
  ui.editor.setRangeText(insert, ui.editor.selectionStart, ui.editor.selectionEnd, "end");
  ui.editor.focus();
  ui.editor.dispatchEvent(new Event("input"));
}

function renderSymbolBar() {
  const classes = "shrink-0 rounded-md border border-stone-700 bg-stone-800 px-3 py-2 font-mono text-sm text-stone-100 active:bg-stone-700";
  const buttons = SYMBOLS.map((symbol) => {
    const button = element("button", classes, symbol);
    button.type = "button";
    button.addEventListener("pointerdown", (event) => event.preventDefault());
    button.addEventListener("click", () => insertAtCursor(symbol));
    return button;
  });
  ui.symbolBar.replaceChildren(...buttons);
}

function scrollToTop() {
  ui.mainScroll.scrollTo({ top: 0 });
}

function readStorage(key) {
  try {
    return JSON.parse(localStorage.getItem(key));
  } catch {
    return null;
  }
}

function writeStorage(key, value) {
  try {
    localStorage.setItem(key, JSON.stringify(value));
  } catch {
    return;
  }
}

function renderInline(parent, text) {
  text.split("`").forEach((part, index) => {
    const isCode = index % 2 === 1;
    const code = "rounded bg-stone-800 px-1.5 py-0.5 font-mono text-[0.85em] text-orange-300";
    parent.append(isCode ? element("code", code, part) : part);
  });
}

function looksLikeTerminalOutput(code) {
  return /^(\$ |> )/m.test(code) || /^\s*(Compiling|Finished|Running|error(\[|:)|warning:)/m.test(code);
}

function highlightRust(code) {
  const nodes = [];
  let cursor = 0;

  for (const match of code.matchAll(RUST_TOKENS)) {
    if (match.index > cursor) {
      nodes.push(code.slice(cursor, match.index));
    }

    const group = match.findIndex((value, index) => index > 0 && value !== undefined) - 1;
    nodes.push(element("span", TOKEN_CLASSES[group], match[0]));
    cursor = match.index + match[0].length;
  }

  nodes.push(code.slice(cursor));
  return nodes;
}

function setCode(pre, code) {
  pre.replaceChildren(...(looksLikeTerminalOutput(code) ? [code] : highlightRust(code)));
}

function renderRunOutput(container, run) {
  const parts = [];

  if (run.stdout) {
    parts.push(element("span", "text-stone-100", run.stdout));
  }

  if (run.stderr) {
    parts.push(element("span", "text-red-300", t(run.stderr)));
  }

  container.replaceChildren(...parts);
}

function buildLessonButton(lesson) {
  const isDone = state.completed.has(lesson.id);
  const isActive = lesson.id === state.currentLessonId;
  const base = "mb-0.5 flex w-full items-center gap-3 rounded-lg px-2.5 py-2 text-left text-sm transition hover:bg-stone-800";
  const button = element("button", isActive ? `${base} bg-stone-800 ring-1 ring-orange-500/40` : base);
  const badge = isDone
    ? "grid size-6 shrink-0 place-items-center rounded-full bg-emerald-500/20 text-xs font-bold text-emerald-300"
    : "grid size-6 shrink-0 place-items-center rounded-full bg-stone-950 text-xs text-stone-400";

  button.append(
    element("span", badge, isDone ? "✓" : String(lesson.id)),
    element("span", "flex-1", t(lesson.title)),
  );
  button.addEventListener("click", () => {
    closeDrawer();
    openLesson(lesson.id);
  });
  return button;
}

function renderLessonList() {
  ui.lessonList.replaceChildren(...state.lessons.map(buildLessonButton));

  const finished = state.completed.size;
  const total = state.lessons.length;
  ui.progressBar.style.width = `${(finished / total) * 100}%`;
  ui.progressLabel.textContent = t("{finished} of {total} exercises completed", { finished, total });
}

function currentLesson() {
  return state.lessons.find((lesson) => lesson.id === state.currentLessonId);
}

function clearResults() {
  ui.status.textContent = "";
  ui.output.replaceChildren();
}

function setStatus(message, kind) {
  const base = "mt-4 min-h-6 text-sm font-semibold whitespace-pre-wrap";
  ui.status.className = `${base} ${STATUS_COLORS[kind]}`;
  ui.status.textContent = message;
}

function renderLessonText(lesson) {
  ui.lessonTitle.textContent = `${lesson.id}. ${t(lesson.title)}`;
  ui.lessonSummary.textContent = t(lesson.summary);

  const paragraphs = lesson.paragraphs.map((text) => {
    const paragraph = element("p");
    renderInline(paragraph, t(text));
    return paragraph;
  });
  ui.lessonBody.replaceChildren(...paragraphs);
  ui.exercisePrompt.textContent = t(lesson.exercise.prompt);
}

function renderLesson(lesson) {
  renderLessonText(lesson);
  setCode(ui.exampleCode, lesson.example);
  ui.editor.value = state.drafts.get(lesson.id) ?? lesson.exercise.starter;
  clearResults();
}

function openLesson(id) {
  state.currentLessonId = id;
  renderLesson(currentLesson());
  renderLessonList();
  scrollToTop();
}

function setBusy(isBusy) {
  ui.runButton.disabled = isBusy;
  ui.checkButton.disabled = isBusy;
}

async function execute(label, task) {
  setBusy(true);
  setStatus(label, "info");

  try {
    await task();
  } catch (error) {
    setStatus(t(String(error)), "error");
  } finally {
    setBusy(false);
  }
}

function runCode() {
  return execute(t("Compiling and running..."), async () => {
    const run = await invoke("run_code", { code: ui.editor.value });
    renderRunOutput(ui.output, run);
    setStatus(run.success ? t("Program finished.") : t("Program failed."), run.success ? "ok" : "error");
  });
}

function checkExercise() {
  return execute(t("Checking your answer..."), async () => {
    const result = await invoke("check_exercise", {
      lessonId: state.currentLessonId,
      code: ui.editor.value,
    });
    renderRunOutput(ui.output, result.run);

    if (result.passed) {
      state.completed.add(state.currentLessonId);
      renderLessonList();
      setStatus(t("Correct! Exercise completed."), "ok");
      return;
    }

    setStatus(`${t("Not quite. The program should print:")}\n${result.expected}`, "error");
  });
}

function resetEditor() {
  const lesson = currentLesson();
  state.drafts.delete(lesson.id);
  ui.editor.value = lesson.exercise.starter;
  clearResults();
}

function loadExampleIntoEditor() {
  ui.editor.value = currentLesson().example;
  state.drafts.set(state.currentLessonId, ui.editor.value);
  clearResults();
}

function handleEditorKeys(event) {
  if (event.key === "Tab") {
    event.preventDefault();
    ui.editor.setRangeText("    ", ui.editor.selectionStart, ui.editor.selectionEnd, "end");
  }

  if (event.key === "Enter" && (event.ctrlKey || event.metaKey)) {
    event.preventDefault();
    runCode();
  }
}

function chapterLabel(chapter) {
  return chapter.number ? `${chapter.number}. ${chapter.title}` : chapter.title;
}

function sectionKey(chapterId, sectionId) {
  return `${chapterId}:${sectionId}`;
}

function buildSectionButton(chapter, section) {
  const label = section.id === 0 ? t("Overview") : section.title;
  const base = "block w-full rounded-md px-2.5 py-1.5 text-left text-sm hover:bg-stone-800";
  const button = element("button", base, label);
  button.classList.add(...NAV_IDLE.split(" "));
  button.addEventListener("click", () => {
    closeDrawer();
    openSection(chapter.id, section.id);
  });
  sectionButtons.set(sectionKey(chapter.id, section.id), button);
  return button;
}

function buildChapterNode(chapter) {
  if (chapter.sections.length === 1) {
    const button = buildSectionButton(chapter, chapter.sections[0]);
    button.textContent = chapterLabel(chapter);
    button.classList.add("font-semibold");
    return button;
  }

  const details = element("details", "mb-1");
  const summary = element(
    "summary",
    "cursor-pointer list-none rounded-lg px-2.5 py-2 text-sm font-semibold text-stone-100 hover:bg-stone-800",
    chapterLabel(chapter),
  );
  const list = element("div", "ml-3 border-l border-stone-800 pl-2");
  list.append(...chapter.sections.map((section) => buildSectionButton(chapter, section)));

  details.append(summary, list);
  chapterDetails.set(chapter.id, details);
  return details;
}

function renderOutline() {
  ui.bookOutline.replaceChildren(...state.outline.map(buildChapterNode));
}

function updateActiveSection() {
  const { chapterId, sectionId } = state.position;

  sectionButtons.forEach((button, key) => {
    setActive(button, key === sectionKey(chapterId, sectionId), NAV_ACTIVE, NAV_IDLE);
  });

  const details = chapterDetails.get(chapterId);

  if (details) {
    details.open = true;
  }
}

function buildParagraph(text) {
  const paragraph = element("p", "leading-7 text-stone-300");
  renderInline(paragraph, text);
  return paragraph;
}

function buildItem(text) {
  const row = element("div", "flex gap-3 leading-7 text-stone-300");
  const content = element("span", "flex-1");
  renderInline(content, text);
  row.append(element("span", "text-orange-400", "•"), content);
  return row;
}

function buildCaption(text) {
  const caption = element("p", "-mt-2 text-sm text-stone-400 italic");
  renderInline(caption, text);
  return caption;
}

function buildHeading(block) {
  return element("h3", HEADING_CLASSES[block.level], block.text);
}

function buildFigureNote() {
  const note = "rounded-lg border border-dashed border-stone-700 px-4 py-3 text-sm text-stone-500";
  return element("div", note, t("A diagram appears here in the printed book."));
}

function isRunnable(code) {
  const hasMain = /\bfn main\s*\(/.test(code);
  const isIncomplete = code.includes("--snip--") || /^(\$ |> )/m.test(code) || code.includes("rand::");
  return hasMain && !isIncomplete;
}

function buildToolbarButton(label, classes) {
  return element("button", `rounded-md px-2.5 py-1 text-xs font-semibold disabled:opacity-50 ${classes}`, label);
}

function buildCodeBlock(code) {
  const wrapper = element("div", "overflow-hidden rounded-xl border border-stone-800 bg-stone-950");
  const toolbar = element("div", "flex justify-end gap-2 border-b border-stone-800 bg-stone-900 px-3 py-1.5");
  const pre = element("pre", "overflow-x-auto p-4 font-mono text-[13px] leading-6 text-stone-200");
  const output = element(
    "pre",
    "hidden max-h-72 overflow-auto border-t border-stone-800 bg-stone-900 p-4 font-mono text-[13px] leading-6 whitespace-pre-wrap",
  );

  setCode(pre, code);

  const copyButton = buildToolbarButton(t("Copy"), "text-stone-300 hover:bg-stone-800");
  copyButton.addEventListener("click", () => copyCode(copyButton, code));
  toolbar.append(copyButton);

  if (isRunnable(code)) {
    const runButton = buildToolbarButton(t("Run"), "bg-orange-500 text-stone-950 hover:bg-orange-400");
    runButton.addEventListener("click", () => runBookCode(runButton, output, code));
    toolbar.append(runButton);
  }

  wrapper.append(toolbar, pre, output);
  return wrapper;
}

async function copyCode(button, code) {
  try {
    await navigator.clipboard.writeText(code);
    button.textContent = t("Copied");
  } catch {
    button.textContent = t("Copy failed");
  }

  setTimeout(() => {
    button.textContent = t("Copy");
  }, 1200);
}

async function runBookCode(button, output, code) {
  button.disabled = true;
  button.textContent = t("Running...");

  try {
    const run = await invoke("run_code", { code });
    renderRunOutput(output, run);
  } catch (error) {
    output.replaceChildren(element("span", "text-red-300", t(String(error))));
  }

  output.classList.remove("hidden");
  button.disabled = false;
  button.textContent = t("Run");
}

function buildBlock(block) {
  switch (block.type) {
    case "paragraph":
      return buildParagraph(block.text);
    case "heading":
      return buildHeading(block);
    case "item":
      return buildItem(block.text);
    case "caption":
      return buildCaption(block.text);
    case "code":
      return buildCodeBlock(block.code);
    default:
      return buildFigureNote();
  }
}

function visibleBlocks(section) {
  const [first, ...rest] = section.blocks;
  const repeatsTitle = first && first.type === "heading" && first.text === section.title;
  return repeatsTitle ? rest : section.blocks;
}

function orderIndex() {
  const { chapterId, sectionId } = state.position;
  return state.order.findIndex((entry) => entry.chapterId === chapterId && entry.sectionId === sectionId);
}

function renderPagerButton(button, entry, direction) {
  button.disabled = !entry;
  button.replaceChildren();

  if (entry) {
    button.append(
      element("span", "block text-xs text-stone-500", direction),
      element("span", "block font-semibold", entry.title),
    );
  }
}

function renderPager() {
  const index = orderIndex();
  renderPagerButton(ui.bookPrev, state.order[index - 1], t("Previous"));
  renderPagerButton(ui.bookNext, state.order[index + 1], t("Next"));
}

function openNeighbour(offset) {
  const entry = state.order[orderIndex() + offset];

  if (entry) {
    openSection(entry.chapterId, entry.sectionId);
  }
}

async function openSection(chapterId, sectionId, { keepScroll = false } = {}) {
  try {
    const section = await invoke("get_book_section", { chapterId, sectionId });
    state.position = { chapterId, sectionId };
    writeStorage(STORAGE_KEYS.position, state.position);

    ui.bookCrumb.textContent = chapterLabel(state.outline[chapterId]);
    ui.bookTitle.textContent = section.title;
    ui.bookContent.replaceChildren(...visibleBlocks(section).map(buildBlock));

    renderPager();
    updateActiveSection();

    if (!keepScroll) {
      scrollToTop();
    }
  } catch (error) {
    ui.bookContent.replaceChildren(element("p", "text-red-400", t(String(error))));
  }
}

function buildSearchHit(hit) {
  const button = element("button", "mb-1 w-full rounded-lg px-3 py-2 text-left hover:bg-stone-800");
  button.append(
    element("span", "block text-sm font-semibold text-stone-100", hit.section_title),
    element("span", "block text-xs text-orange-400", hit.chapter_title),
    element("span", "mt-1 line-clamp-2 block text-xs text-stone-400", hit.snippet),
  );
  button.addEventListener("click", () => {
    closeDrawer();
    openSection(hit.chapter_id, hit.section_id);
  });
  return button;
}

async function runSearch() {
  const query = ui.searchInput.value.trim();
  const isSearching = query.length >= 2;

  ui.searchResults.hidden = !isSearching;
  ui.bookOutline.hidden = isSearching;

  if (!isSearching) {
    return;
  }

  const hits = await invoke("search_book", { query });
  const empty = element("p", "px-3 py-2 text-sm text-stone-500", t("No results found."));
  ui.searchResults.replaceChildren(...(hits.length ? hits.map(buildSearchHit) : [empty]));
}

function handleSearchInput() {
  clearTimeout(searchTimer);
  searchTimer = setTimeout(runSearch, 250);
}

function showTab(tab) {
  const isLessons = tab === "lessons";
  state.tab = tab;
  writeStorage(STORAGE_KEYS.tab, tab);

  ui.lessonsPanel.hidden = !isLessons;
  ui.lessonView.hidden = !isLessons;
  ui.bookPanel.hidden = isLessons;
  ui.bookView.hidden = isLessons;

  setActive(ui.tabLessons, isLessons, TAB_ACTIVE, TAB_IDLE);
  setActive(ui.tabBook, !isLessons, TAB_ACTIVE, TAB_IDLE);
  scrollToTop();
}

async function openBookTab() {
  showTab("book");

  if (!state.position) {
    const saved = readStorage(STORAGE_KEYS.position);
    const target = saved && state.outline[saved.chapterId] ? saved : state.order[0];
    await openSection(target.chapterId, target.sectionId);
  }
}

function renderLanguageSwitch() {
  const buttons = I18n.languages.map((language) => {
    const button = element("button", "rounded-md px-2 py-1 transition", language.toUpperCase());
    button.type = "button";
    button.addEventListener("click", () => changeLanguage(language));
    setActive(button, language === I18n.language, LANG_ACTIVE, LANG_IDLE);
    return button;
  });
  ui.langSwitch.replaceChildren(...buttons);
}

async function refreshLanguage() {
  I18n.apply();
  renderLanguageSwitch();
  ui.bookNote.hidden = I18n.language === "en";

  if (!state.lessons.length) {
    return;
  }

  renderLessonList();
  renderLessonText(currentLesson());
  renderOutline();

  if (state.position) {
    await openSection(state.position.chapterId, state.position.sectionId, { keepScroll: true });
  }
}

function changeLanguage(language) {
  I18n.set(language);
  refreshLanguage();
}

function bindEvents() {
  ui.menuButton.addEventListener("click", () => setDrawer(true));
  ui.backdrop.addEventListener("click", closeDrawer);
  ui.tabLessons.addEventListener("click", () => showTab("lessons"));
  ui.tabBook.addEventListener("click", openBookTab);
  ui.runButton.addEventListener("click", runCode);
  ui.checkButton.addEventListener("click", checkExercise);
  ui.resetButton.addEventListener("click", resetEditor);
  ui.loadExample.addEventListener("click", loadExampleIntoEditor);
  ui.editor.addEventListener("keydown", handleEditorKeys);
  ui.editor.addEventListener("input", () => state.drafts.set(state.currentLessonId, ui.editor.value));
  ui.searchInput.addEventListener("input", handleSearchInput);
  ui.bookPrev.addEventListener("click", () => openNeighbour(-1));
  ui.bookNext.addEventListener("click", () => openNeighbour(1));
}

async function start() {
  I18n.init();
  bindEvents();
  renderLanguageSwitch();
  renderSymbolBar();
  ui.bookNote.hidden = I18n.language === "en";

  state.lessons = await invoke("list_lessons");
  state.completed = new Set(await invoke("get_progress"));
  state.outline = await invoke("list_book");
  state.order = state.outline.flatMap((chapter) =>
    chapter.sections.map((section) => ({
      chapterId: chapter.id,
      sectionId: section.id,
      title: section.title,
    })),
  );

  renderOutline();
  openLesson(state.lessons[0].id);

  if (readStorage(STORAGE_KEYS.tab) === "book") {
    await openBookTab();
    return;
  }

  showTab("lessons");
}

start();

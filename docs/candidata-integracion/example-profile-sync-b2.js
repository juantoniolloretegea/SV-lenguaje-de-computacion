"use strict";

(() => {
  const PROFILE_TEXT = {
    es: {
      explanation: "El perfil es explícito. Los ejemplos intactos cargan su variante homóloga; el código propio o modificado nunca se transforma automáticamente.",
      autoSwitched: profile => `Ejemplo de biblioteca intacto: cargada automáticamente su variante ${profile}.`,
      preserved: "Perfil cambiado. El código propio o modificado se ha conservado sin transformación.",
      modified: "Código modificado"
    },
    en: {
      explanation: "The profile is explicit. Unmodified library examples load their matching variant; custom or modified code is never transformed automatically.",
      autoSwitched: profile => `Unmodified library example: its matching ${profile} variant was loaded automatically.`,
      preserved: "Profile changed. Custom or modified code was preserved without transformation.",
      modified: "Modified source"
    }
  };

  const ui = () => document.documentElement.lang === "en" ? "en" : "es";
  const tx = () => PROFILE_TEXT[ui()];
  const editor = document.getElementById("source-editor");
  const fileName = document.getElementById("file-name");
  const status = document.getElementById("editor-status");
  const output = document.getElementById("result-output");
  const examplesList = document.getElementById("examples-list");
  const profileExplanation = document.querySelector('[data-i18n="profileExplanation"]');

  if (!editor || !fileName || !status || !examplesList) return;

  let examples = [];
  let baseline = null;
  let lastProfile = document.body.dataset.sourceProfile === "en" ? "en" : "es";

  function profileLabel(profile) {
    return profile === "es" ? "SVP-ES" : "SVP-EN";
  }

  function sourceFor(example, profile) {
    return profile === "es" ? example.source_es : example.source_en;
  }

  function matchExactLibraryExample() {
    return examples.find(example =>
      example.file_name === fileName.value &&
      (editor.value === example.source_es || editor.value === example.source_en)
    ) ?? null;
  }

  function profileOfSource(example) {
    if (editor.value === example.source_es) return "es";
    if (editor.value === example.source_en) return "en";
    return null;
  }

  function setBaselineFromCurrent() {
    const example = matchExactLibraryExample();
    if (!example) {
      baseline = null;
      updateModifiedMarker();
      return;
    }
    const profile = profileOfSource(example);
    baseline = {
      example,
      profile,
      fileName: example.file_name,
      source: sourceFor(example, profile)
    };
    updateModifiedMarker();
  }

  function captureBaselineIfNeeded() {
    if (!baseline && matchExactLibraryExample()) setBaselineFromCurrent();
  }

  function ensureModifiedMarker() {
    let wrap = fileName.parentElement;
    if (!wrap?.classList.contains("file-input-wrap-b2")) {
      wrap = document.createElement("span");
      wrap.className = "file-input-wrap-b2";
      fileName.parentNode.insertBefore(wrap, fileName);
      wrap.append(fileName);
    }
    let marker = wrap.querySelector(".source-modified-marker-b2");
    if (!marker) {
      marker = document.createElement("span");
      marker.className = "source-modified-marker-b2";
      marker.textContent = "*";
      marker.hidden = true;
      wrap.append(marker);
    }
    return marker;
  }

  function updateModifiedMarker() {
    const marker = ensureModifiedMarker();
    const modified = Boolean(baseline) &&
      (editor.value !== baseline.source || fileName.value !== baseline.fileName);
    marker.hidden = !modified;
    marker.title = tx().modified;
    marker.setAttribute("aria-label", tx().modified);
    fileName.classList.toggle("is-modified-b2", modified);
  }

  function updateExplanation() {
    if (profileExplanation) profileExplanation.textContent = tx().explanation;
    updateModifiedMarker();
  }

  function clearStaleResult() {
    if (output) output.textContent = "";
  }

  function switchMatchingExample(targetProfile) {
    const example = matchExactLibraryExample();
    if (!example) return false;
    const currentProfile = profileOfSource(example);
    if (!currentProfile || currentProfile === targetProfile) {
      setBaselineFromCurrent();
      return false;
    }

    editor.value = sourceFor(example, targetProfile);
    fileName.value = example.file_name;
    baseline = {
      example,
      profile: targetProfile,
      fileName: example.file_name,
      source: sourceFor(example, targetProfile)
    };
    editor.dispatchEvent(new Event("input", { bubbles: true }));
    clearStaleResult();
    updateModifiedMarker();
    status.textContent = tx().autoSwitched(profileLabel(targetProfile));
    return true;
  }

  function installStyles() {
    const style = document.createElement("style");
    style.textContent = `
      .file-input-wrap-b2 { position: relative; display: block; min-width: 0; }
      .file-input-wrap-b2 #file-name { width: 100%; padding-right: 28px; }
      .source-modified-marker-b2 {
        position: absolute; right: 10px; top: 50%; transform: translateY(-50%);
        color: var(--profile); font-weight: 900; font-family: var(--mono); pointer-events: none;
      }
    `;
    document.head.append(style);
  }

  installStyles();
  updateExplanation();

  fetch("examples.json", { cache: "no-store" })
    .then(response => {
      if (!response.ok) throw new Error(`examples HTTP ${response.status}`);
      return response.json();
    })
    .then(data => {
      examples = Array.isArray(data) ? data : [];
      setBaselineFromCurrent();
      setTimeout(captureBaselineIfNeeded, 250);
      setTimeout(captureBaselineIfNeeded, 1000);
    })
    .catch(() => {
      examples = [];
      baseline = null;
      updateModifiedMarker();
    });

  for (const button of document.querySelectorAll("[data-profile-choice]")) {
    button.addEventListener("click", () => {
      const targetProfile = button.dataset.profileChoice;
      if (targetProfile === lastProfile) return;
      lastProfile = targetProfile;
      queueMicrotask(() => {
        clearStaleResult();
        if (!switchMatchingExample(targetProfile)) {
          status.textContent = tx().preserved;
          updateModifiedMarker();
        }
        updateExplanation();
      });
    });
  }

  for (const button of document.querySelectorAll("[data-ui-choice]")) {
    button.addEventListener("click", () => queueMicrotask(updateExplanation));
  }

  editor.addEventListener("focus", captureBaselineIfNeeded);
  fileName.addEventListener("focus", captureBaselineIfNeeded);
  editor.addEventListener("input", updateModifiedMarker);
  fileName.addEventListener("input", updateModifiedMarker);

  examplesList.addEventListener("click", event => {
    if (!event.target.closest("button")) return;
    queueMicrotask(setBaselineFromCurrent);
  });
})();

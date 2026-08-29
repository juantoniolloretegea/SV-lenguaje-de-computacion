"use strict";

(() => {
  const stylesheet = document.createElement("link");
  stylesheet.rel = "stylesheet";
  stylesheet.href = "workspace-b2.css?v=b2-examples-centered-20260829-1";
  document.head.append(stylesheet);

  const staticI18n = document.createElement("script");
  staticI18n.src = "i18n-static-b2.js?v=b2-source-profile-history-20260829-2";
  staticI18n.defer = true;
  document.head.append(staticI18n);

  const requestedUi = new URLSearchParams(window.location.search).get("ui");

  window.addEventListener("load", () => {
    if (["es", "en"].includes(requestedUi)) {
      const uiButton = document.querySelector(`[data-ui-choice="${requestedUi}"]`);
      if (uiButton && uiButton.getAttribute("aria-pressed") !== "true") uiButton.click();
    }

    const profileSync = document.createElement("script");
    profileSync.src = "example-profile-sync-b2.js?v=b2-profile-sync-20260829-1";
    document.body.append(profileSync);
  }, { once: true });

  for (const uiButton of document.querySelectorAll("[data-ui-choice]")) {
    uiButton.addEventListener("click", () => {
      const lang = uiButton.dataset.uiChoice;
      const url = new URL(window.location.href);
      if (lang === "en") url.searchParams.set("ui", "en");
      else url.searchParams.delete("ui");
      history.replaceState(null, "", `${url.pathname}${url.search}${url.hash}`);
    });
  }

  const referenceTargets = {
    "GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md": "https://lectura-sv.itvia.online/lenguaje/?file=GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md",
    "IR_CANONICA_BIENFORMACION_SV_v0_3.md": "https://lectura-sv.itvia.online/lenguaje/?file=IR_CANONICA_BIENFORMACION_SV_v0_3.md"
  };
  for (const link of document.querySelectorAll("a.rail-link")) {
    const fileName = link.querySelector("small code")?.textContent?.trim();
    if (fileName && referenceTargets[fileName]) link.href = referenceTargets[fileName];
  }

  const assemblyShell = document.querySelector("#view-assembly .assembly-shell");
  const assemblyGrid = document.querySelector("#view-assembly .assembly-grid");
  const assemblyActions = document.querySelector("#view-assembly .assembly-actions");
  if (assemblyShell && assemblyGrid && assemblyActions) {
    assemblyActions.classList.add("assembly-actions-top");
    assemblyActions.setAttribute("role", "toolbar");
    assemblyActions.setAttribute("aria-label", "Acciones de ensamblaje / Assembly actions");
    assemblyShell.insertBefore(assemblyActions, assemblyGrid);
  }

  const buttons = [...document.querySelectorAll("[data-workspace-target]")];
  const views = [...document.querySelectorAll("[data-workspace-view]")];

  function activate(name, updateHash = true) {
    if (!views.some(view => view.dataset.workspaceView === name)) return;
    for (const view of views) view.hidden = view.dataset.workspaceView !== name;
    for (const button of buttons) {
      const active = button.dataset.workspaceTarget === name;
      button.classList.toggle("is-active", active);
      button.setAttribute("aria-pressed", String(active));
    }
    if (updateHash) history.replaceState(null, "", `#${name}`);
    const target = views.find(view => view.dataset.workspaceView === name);
    target?.scrollIntoView({ block: "start", behavior: "instant" });
  }

  for (const button of buttons) {
    button.addEventListener("click", () => activate(button.dataset.workspaceTarget));
  }

  const requested = location.hash.replace(/^#/, "");
  activate(["editor", "assembly"].includes(requested) ? requested : "editor", false);
})();
"use strict";

(() => {
  const COPY = {
    es: {
      brandSubtitle: "Beta B2 · programación en español e inglés y ensamblaje de archivos en el navegador.",
      statePrimary: "Beta B2 · experimental · no incorporada a la versión estable",
      stateSecondary: "Gramática 0.2 · IR 0.3 · Proyección 0.1.0 · versión de referencia",
      railWorkspace: "ENTORNO DE TRABAJO",
      railReference: "REFERENCIA",
      railAssurance: "DOCUMENTACIÓN",
      railDevelopment: "VERSIONES FUTURAS",
      railEditor: "Editor SVP",
      railAssembly: "Ensamblaje",
      grammar: "Gramática 0.2",
      reader: "abrir documento con traducción",
      programmingMetrics: "Metodología y métricas",
      futureBeta: "versión futura",
      editorEyebrow: "ENTORNO DE TRABAJO",
      editorHeading: "Editor SVP",
      editorIntro: "Compilación local con perfil lingüístico explícito. El idioma de la interfaz y el del código se gestionan de forma independiente.",
      sourceEs: "Español · SVP-ES",
      sourceEn: "English · SVP-EN",
      ideEyebrow: "VERSIÓN FUTURA · PROPUESTA",
      ideTitle: "Entorno de desarrollo propuesto",
      ideText: "Propuesta para organizar proyectos, varios archivos, pestañas, símbolos, diagnósticos estructurados, IR y trazas. Beta B2 no incorpora todavía estas funciones.",
      ideProject: "Proyecto",
      ideEditor: "Archivos / editor",
      ideDiagnostics: "Diagnósticos · IR · trazas",
      buildGrammar: "Gramática",
      navLabel: "Navegación del entorno de trabajo",
      configLabel: "Configuración",
      sourceLanguage: "Perfil del código",
      activeProfile: "Perfil lingüístico del código",
      sourceCode: "Código",
      run: "Compilar",
      execution: "Compilación local",
      architectureTitle: "Interpretación común de SVP-ES y SVP-EN",
      architectureText: "SVP-ES y SVP-EN se interpretan antes del análisis sintáctico y utilizan el mismo analizador, la misma IR y la misma semántica. Los identificadores, cadenas, comentarios y datos del usuario se conservan sin traducción.",
      reproducibility: "Datos técnicos de Beta B2",
      sourceCut: "Versión de referencia",
      betaStateValue: "Experimental · no incorporada a la versión estable",
      assemblyEyebrow: "ENTORNO DE TRABAJO · B2",
      assemblyTitle: "Ensamblaje de archivos",
      assemblyIntro: "Cada archivo .svp conserva su perfil y se analiza por separado. Los resultados se integran en una única IR y se validan conjuntamente. El contenido de los archivos no se concatena.",
      loadIndependentDemo: "Ejemplo ES+EN independiente",
      loadLinkedDemo: "Ejemplo ES→EN con referencia entre archivos",
      individualCheck: "Validación individual",
      assemblyCheck: "Validación conjunta",
      footerText: "La interfaz presenta los archivos y los resultados. La compilación y la validación del ensamblaje se realizan localmente en WebAssembly.",
      gateTitle: "Admisibilidad y evaluación",
      gateDescriptionFrom: "Compuerta con tabla de admisibilidad y operación de evaluación.",
      gateDescriptionTo: "Tabla explícita de admisibilidad y evaluación.",
      composeDescriptionFrom: "Composición de estructuras bajo el régimen de la gramática vigente.",
      composeDescriptionTo: "Composición de estructuras conforme a la gramática vigente."
    },
    en: {
      brandSubtitle: "Beta B2 · Spanish/English programming and file assembly in the browser.",
      statePrimary: "Beta B2 · experimental · not part of the stable release",
      stateSecondary: "Grammar 0.2 · IR 0.3 · Projection 0.1.0 · reference version",
      railWorkspace: "WORKSPACE",
      railReference: "REFERENCE",
      railAssurance: "DOCUMENTATION",
      railDevelopment: "FUTURE VERSIONS",
      railEditor: "SVP editor",
      railAssembly: "Assembly",
      grammar: "Grammar 0.2",
      reader: "open document with translation",
      programmingMetrics: "Methods and metrics",
      futureBeta: "future version",
      editorEyebrow: "WORKSPACE",
      editorHeading: "SVP editor",
      editorIntro: "Local compilation with an explicit language profile. Interface language and code language are managed independently.",
      sourceEs: "Spanish · SVP-ES",
      sourceEn: "English · SVP-EN",
      ideEyebrow: "FUTURE VERSION · PROPOSAL",
      ideTitle: "Proposed development environment",
      ideText: "Proposal for organizing projects, multiple files, tabs, symbols, structured diagnostics, IR and traces. Beta B2 does not yet provide these functions.",
      ideProject: "Project",
      ideEditor: "Files / editor",
      ideDiagnostics: "Diagnostics · IR · traces",
      buildGrammar: "Grammar",
      navLabel: "Workspace navigation",
      configLabel: "Configuration",
      sourceLanguage: "Code profile",
      activeProfile: "Code language profile",
      sourceCode: "Code",
      run: "Compile",
      execution: "Local compilation",
      architectureTitle: "Shared interpretation of SVP-ES and SVP-EN",
      architectureText: "SVP-ES and SVP-EN are interpreted before parsing and use the same parser, IR and semantics. User identifiers, strings, comments and data are preserved without translation.",
      reproducibility: "Beta B2 technical details",
      sourceCut: "Reference version",
      betaStateValue: "Experimental · not part of the stable release",
      assemblyEyebrow: "WORKSPACE · B2",
      assemblyTitle: "File assembly",
      assemblyIntro: "Each .svp file keeps its own profile and is analyzed separately. The results are integrated into one IR and validated together. File contents are not concatenated.",
      loadIndependentDemo: "Independent ES+EN example",
      loadLinkedDemo: "ES→EN example with cross-file reference",
      individualCheck: "Individual validation",
      assemblyCheck: "Joint validation",
      footerText: "The interface presents files and results. Compilation and assembly validation run locally in WebAssembly.",
      gateTitle: "Admissibility and evaluation",
      gateDescriptionFrom: "Gate with admissibility table and evaluation operation.",
      gateDescriptionTo: "Explicit admissibility table and evaluation.",
      composeDescriptionFrom: "Composition of structures under the current grammar regime.",
      composeDescriptionTo: "Composition of structures under the current grammar."
    }
  };

  const q = selector => document.querySelector(selector);
  const qa = selector => [...document.querySelectorAll(selector)];

  function setText(selector, value) {
    const node = q(selector);
    if (node && node.textContent !== value) node.textContent = value;
  }

  function setTextAll(selector, value) {
    for (const node of qa(selector)) if (node.textContent !== value) node.textContent = value;
  }

  function setReference(fileName, title, readerText) {
    for (const link of qa("a.rail-link")) {
      if (link.querySelector("small code")?.textContent?.trim() !== fileName) continue;
      const strong = link.querySelector("strong");
      const span = link.querySelector("span");
      if (strong && strong.textContent !== title) strong.textContent = title;
      if (span && span.textContent !== readerText) span.textContent = readerText;
    }
  }

  function setIdeButton(label) {
    const button = q('[data-workspace-target="ide"]');
    if (!button) return;
    const current = button.querySelector("span")?.textContent ?? "";
    if (current === label) return;
    button.replaceChildren(document.createTextNode("IDE "));
    const span = document.createElement("span");
    span.textContent = label;
    button.append(span);
  }

  function setDocumentLinks(lang) {
    const features = q('a.rail-link[data-i18n="features"]');
    const history = q('a.rail-link[data-i18n="history"]');
    if (features) features.href = lang === "en" ? "features.html" : "caracteristicas.html";
    if (history) history.href = lang === "en" ? "beta-history.html" : "historial-beta.html";

    const programming = qa("a.rail-link").find(link => {
      const href = link.getAttribute("href") ?? "";
      return href === "programacion-metricas.html" || href === "programming-metrics.html";
    });
    if (programming) programming.href = lang === "en" ? "programming-metrics.html" : "programacion-metricas.html";
  }

  function polishExamples(c) {
    const list = q("#examples-list");
    if (!list) return;
    for (const card of list.querySelectorAll(".example-card")) {
      const title = card.querySelector("h3");
      const description = card.querySelector("p");
      if (title && (title.textContent === "Compuerta y admisibilidad" || title.textContent === "Gate and admissibility")) {
        title.textContent = c.gateTitle;
      }
      if (description?.textContent.includes(c.gateDescriptionFrom)) {
        description.textContent = description.textContent.replace(c.gateDescriptionFrom, c.gateDescriptionTo);
      }
      if (description?.textContent.includes(c.composeDescriptionFrom)) {
        description.textContent = description.textContent.replace(c.composeDescriptionFrom, c.composeDescriptionTo);
      }
    }
  }

  function apply() {
    const lang = document.documentElement.lang === "en" ? "en" : "es";
    const c = COPY[lang];

    setText(".beta-brand p", c.brandSubtitle);
    setText(".beta-state-line strong", c.statePrimary);
    const secondary = q(".beta-state-line span");
    if (secondary) {
      const code = secondary.querySelector("code")?.textContent ?? "f6b704e819e7…";
      const expected = `${c.stateSecondary} ${code}`;
      if (secondary.textContent.trim() !== expected) {
        secondary.replaceChildren(document.createTextNode(`${c.stateSecondary} `));
        const codeNode = document.createElement("code");
        codeNode.textContent = code;
        secondary.append(codeNode);
      }
    }

    setText('.rail-group:nth-of-type(1) .rail-heading', c.railWorkspace);
    setText('.rail-group:nth-of-type(2) .rail-heading', c.railReference);
    setText('.rail-group:nth-of-type(3) .rail-heading', c.railAssurance);
    setText('.rail-group:nth-of-type(4) .rail-heading', c.railDevelopment);
    setText('[data-workspace-target="editor"]', c.railEditor);
    setText('[data-workspace-target="assembly"]', c.railAssembly);
    setIdeButton(c.futureBeta);

    setReference("GRAMATICA_SUPERFICIAL_MINIMA_SV_v0_2.md", c.grammar, c.reader);
    setReference("IR_CANONICA_BIENFORMACION_SV_v0_3.md", "IR 0.3", c.reader);
    setText('a.rail-link[href="programacion-metricas.html"], a.rail-link[href="programming-metrics.html"]', c.programmingMetrics);
    setDocumentLinks(lang);

    setText("#view-editor .view-heading .eyebrow", c.editorEyebrow);
    setText("#view-editor .view-heading h2", c.editorHeading);
    setText("#view-editor .view-heading p:not(.eyebrow)", c.editorIntro);

    const esProfile = q('[data-profile-choice="es"]');
    const enProfile = q('[data-profile-choice="en"]');
    if (esProfile) esProfile.textContent = c.sourceEs;
    if (enProfile) enProfile.textContent = c.sourceEn;

    setText('[data-i18n="sourceLanguage"]', c.sourceLanguage);
    setText('[data-i18n="activeProfile"]', c.activeProfile);
    setText('[data-i18n="sourceCode"]', c.sourceCode);
    setText('[data-i18n="run"]', c.run);
    setText('[data-i18n="execution"]', c.execution);
    setText('[data-i18n="architectureTitle"]', c.architectureTitle);
    setText('[data-i18n="architectureText"]', c.architectureText);
    setText('[data-i18n="reproducibility"]', c.reproducibility);
    setText('[data-i18n="sourceCut"]', c.sourceCut);
    setTextAll('[data-i18n="betaStateValue"]', c.betaStateValue);

    setText("#view-assembly .view-heading .eyebrow", c.assemblyEyebrow);
    setTextAll('[data-i18n="assemblyTitle"]', c.assemblyTitle);
    setText('[data-i18n="assemblyIntro"]', c.assemblyIntro);
    setText('[data-i18n="loadIndependentDemo"]', c.loadIndependentDemo);
    setText('[data-i18n="loadLinkedDemo"]', c.loadLinkedDemo);
    setTextAll('[data-i18n="individualCheck"]', c.individualCheck);
    setText('[data-i18n="assemblyCheck"]', c.assemblyCheck);
    setText('[data-i18n="footerText"]', c.footerText);

    setText("#view-ide .eyebrow", c.ideEyebrow);
    setText("#view-ide h2", c.ideTitle);
    setText("#view-ide > .ide-shell-placeholder > p:not(.eyebrow)", c.ideText);
    const ideCells = qa("#view-ide .ide-placeholder-grid div");
    if (ideCells[0]) ideCells[0].textContent = c.ideProject;
    if (ideCells[1]) ideCells[1].textContent = c.ideEditor;
    if (ideCells[2]) ideCells[2].textContent = c.ideDiagnostics;

    const grammarDt = q("#build-grammar")?.closest("div")?.querySelector("dt");
    if (grammarDt) grammarDt.textContent = c.buildGrammar;

    const rail = q(".laboratory-rail");
    if (rail) rail.setAttribute("aria-label", c.navLabel);
    const controls = q(".control-panel");
    if (controls) controls.setAttribute("aria-label", c.configLabel);

    polishExamples(c);
  }

  let scheduled = false;
  function scheduleApply() {
    if (scheduled) return;
    scheduled = true;
    queueMicrotask(() => {
      scheduled = false;
      apply();
    });
  }

  const langObserver = new MutationObserver(mutations => {
    if (mutations.some(mutation => mutation.type === "attributes" && mutation.attributeName === "lang")) scheduleApply();
  });
  langObserver.observe(document.documentElement, { attributes: true, attributeFilter: ["lang"] });

  const examples = q("#examples-list");
  if (examples) {
    const examplesObserver = new MutationObserver(scheduleApply);
    examplesObserver.observe(examples, { childList: true, subtree: true });
  }

  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", scheduleApply, { once: true });
  else scheduleApply();
  window.addEventListener("load", scheduleApply, { once: true });
})();

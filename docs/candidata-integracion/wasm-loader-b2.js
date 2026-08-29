"use strict";

(() => {
  const nativeFetch = window.fetch.bind(window);
  const wasmChunks = [
    "wasm-b2/part-00.b64",
    "wasm-b2/part-01.b64",
    "wasm-b2/part-02.b64",
    "wasm-b2/part-03.b64",
    "wasm-b2/part-04.b64",
    "wasm-b2/part-05.b64",
    "wasm-b2/p06-0.b64",
    "wasm-b2/p06-1.b64",
    "wasm-b2/p06-2.b64",
    "wasm-b2/p06-3.b64",
    "wasm-b2/part-07.b64",
    "wasm-b2/tail-00.b64",
    "wasm-b2/tail-01.b64",
    "wasm-b2/tail-02.b64",
    "wasm-b2/t-00.b64",
    "wasm-b2/t-01.b64",
    "wasm-b2/t-02.b64",
    "wasm-b2/t-03.b64",
    "wasm-b2/t-04.b64",
    "wasm-b2/t-05.b64",
    "wasm-b2/t-06.b64",
    "wasm-b2/t-07.b64"
  ];

  async function loadChunk(path) {
    const response = await nativeFetch(path, { cache: "default" });
    if (!response.ok) {
      throw new Error(`HTTP ${response.status} al cargar ${path}`);
    }
    const text = (await response.text()).replace(/\s+/g, "");
    if (!/^[A-Za-z0-9+/]*={0,2}$/.test(text)) {
      throw new Error(`fragmento Base64 no válido: ${path}`);
    }
    return text;
  }

  window.fetch = async (input, init) => {
    const url = typeof input === "string" ? input : (input?.url ?? "");
    if (!url.endsWith("sv_wasm_b2.wasm.gz.b64")) {
      return nativeFetch(input, init);
    }

    const pieces = await Promise.all(wasmChunks.map(loadChunk));
    const joined = pieces.join("");
    if (joined.length % 4 !== 0) {
      throw new Error(`longitud Base64 WebAssembly no válida: ${joined.length}`);
    }

    return new Response(joined, {
      status: 200,
      headers: {
        "Content-Type": "text/plain;charset=utf-8",
        "Cache-Control": "public, max-age=3600"
      }
    });
  };
})();

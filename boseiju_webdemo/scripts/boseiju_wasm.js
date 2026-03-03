/// Js handle for a string allocated on the web assembly land.
class WasmString {
  constructor(wasm, ptr, len) {
    this.wasm = wasm;
    this.ptr = ptr;
    this.len = len;
  }

  static fromJS(wasm, js_string) {
    const bytes = new TextEncoder().encode(js_string);
    const pointer = wasm.instance.exports.alloc(bytes.length);
    new Uint8Array(
      wasm.instance.exports.memory.buffer,
      pointer,
      bytes.length,
    ).set(bytes);
    return new WasmString(wasm, pointer, bytes.length);
  }

  static fromPtr(wasm, pointer) {
    const mem_view = new Uint8Array(wasm.instance.exports.memory.buffer);
    let string_length = 0;
    while (mem_view[pointer + string_length] !== 0) string_length++;
    return new WasmString(wasm, pointer, string_length + 1);
  }

  toJS() {
    const bytes = new Uint8Array(
      this.wasm.instance.exports.memory.buffer,
      this.ptr,
      this.len,
    );
    return new TextDecoder().decode(
      bytes[bytes.length - 1] === 0 ? bytes.slice(0, -1) : bytes,
    );
  }

  free() {
    this.wasm.instance.exports.free(this.ptr, this.len);
  }
}

async function load_wasm() {
  const wasm = await WebAssembly.instantiateStreaming(
    fetch("boseiju.wasm"),
    {},
  );

  return {
    preprocess(card_name, oracle_text) {
      /* card_name is optional and should not block parsing */
      if (card_name == "") {
        card_name = "~";
      }

      const wasm_card_name = WasmString.fromJS(wasm, card_name);
      const wasm_oracle_text = WasmString.fromJS(wasm, oracle_text);

      const preprocessed_ptr = wasm.instance.exports.lexer_preprocessor(
        wasm_card_name.ptr,
        wasm_card_name.len,
        wasm_oracle_text.ptr,
        wasm_oracle_text.len,
      );
      const wasm_preprocessed = WasmString.fromPtr(wasm, preprocessed_ptr);
      const preprocessed = wasm_preprocessed.toJS();

      wasm_card_name.free();
      wasm_oracle_text.free();
      wasm_preprocessed.free();

      return preprocessed;
    },

    lex(card_name, oracle_text) {
      /* card_name is optional and should not block parsing */
      if (card_name == "") {
        card_name = "~";
      }

      const wasm_card_name = WasmString.fromJS(wasm, card_name);
      const wasm_oracle_text = WasmString.fromJS(wasm, oracle_text);

      const lexed_ptr = wasm.instance.exports.lex(
        wasm_card_name.ptr,
        wasm_card_name.len,
        wasm_oracle_text.ptr,
        wasm_oracle_text.len,
      );
      const wasm_lexed = WasmString.fromPtr(wasm, lexed_ptr);
      const lexed = wasm_lexed.toJS();

      wasm_card_name.free();
      wasm_oracle_text.free();
      wasm_lexed.free();

      console.log(lexed);
      const json_result = JSON.parse(lexed);
      if (json_result.err) throw json_result.err;

      return json_result.tokens;
    },

    parse_oracle_text(card_name, oracle_text) {
      /* card_name is optional and should not block parsing */
      if (card_name == "") {
        card_name = "~";
      }

      const encoder = new TextEncoder();
      const card_name_bytes = encoder.encode(card_name);
      const oracle_text_bytes = encoder.encode(oracle_text);

      /* Allocate inputs memory */
      const card_name_len = card_name_bytes.length;
      const card_name_ptr = wasm.instance.exports.alloc(card_name_len);
      const card_name_mem = new Uint8Array(
        wasm.instance.exports.memory.buffer,
        card_name_ptr,
        card_name_len,
      );
      card_name_mem.set(card_name_bytes);

      const oracle_text_len = oracle_text_bytes.length;
      const oracle_text_ptr = wasm.instance.exports.alloc(oracle_text_len);
      const oracle_text_mem = new Uint8Array(
        wasm.instance.exports.memory.buffer,
        oracle_text_ptr,
        oracle_text_len,
      );
      oracle_text_mem.set(oracle_text_bytes);

      const result_ptr = wasm.instance.exports.parse_oracle_text(
        card_name_ptr,
        card_name_len,
        oracle_text_ptr,
        oracle_text_len,
      );

      /* Read back the output (null-terminated) */
      const full_mem_view = new Uint8Array(wasm.instance.exports.memory.buffer);
      const result_bytes = [];

      let i = result_ptr;
      while (full_mem_view[i] !== 0) {
        result_bytes.push(full_mem_view[i]);
        i++;
      }

      /* +1 for null terminating byte */
      const result_len = result_bytes.length + 1;
      const result = new TextDecoder().decode(new Uint8Array(result_bytes));

      wasm.instance.exports.free(card_name_ptr, card_name_len);
      wasm.instance.exports.free(oracle_text_ptr, oracle_text_len);
      // Fixme?
      wasm.instance.exports.free(result_ptr, result_len);

      return result;
    },
  };
}

function create_error_html(error) {
  return `<p class="error">${error}</p>`;
}

function render_preprocessed(text) {
  const NL_ICON = "⏎";
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/\n/g, `<span class="new-line">${NL_ICON}</span>`);
}

function render_tokens(tokens, total_length) {
  const row = document.createElement("div");
  row.className = "token-row";

  // Fill grid with empty cells so spans work
  row.style.gridTemplateColumns = `repeat(${total_length}, 1ch)`;

  for (const tok of tokens) {
    const box = document.createElement("div");
    box.className = "token";
    box.style.gridColumn = `${tok.span.start + 1} / span ${tok.span.length}`;
    box.textContent = Object.keys(tok.kind)[0];
    row.appendChild(box);
  }

  return row;
}

function fill_grid_with_preprocessed(grid, preprocessed) {
  const NL_ICON = "⏎";
  const SPACE_WIDTH = 1;

  grid.innerHTML = "";
  grid.style.gridTemplateColumns = `repeat(${preprocessed.length}, 1ch)`;

  for (let i = 0; i < preprocessed.length; i++) {
    const cell = document.createElement("div");
    cell.className = "char";
    cell.style.gridColumn = i + 1;
    cell.style.gridRow = 1;
    cell.dataset.index = i;

    const ch = preprocessed[i];
    if (ch === "\n") {
      cell.textContent = NL_ICON;
      cell.classList.add("new-line");
    } else {
      cell.textContent = ch;
    }

    grid.appendChild(cell);
  }
}

function fill_grid_with_tokens(grid, tokens) {
  for (const tok of tokens) {
    const box = document.createElement("div");
    box.className = "token";
    box.style.gridRow = 2;
    box.style.gridColumn = `${tok.span.start + 1} / span ${tok.span.length}`;

    const [kind, variant] = Object.entries(tok.kind)[0];
    box.textContent = kind;
    box.title = `${kind} :: ${variant}`;
    grid.appendChild(box);
  }
}

/* When the WASM file is loaded, add the event listeners */
load_wasm().then((wasm_module) => {
  const card_name_input = document.getElementById("card_name_input");
  const oracle_text_input = document.getElementById("oracle_text_input");
  const grid = document.getElementById("token_grid");

  function update_output() {
    const preprocessed = wasm_module.preprocess(
      card_name_input.value,
      oracle_text_input.value,
    );
    const tokens = wasm_module.lex(
      card_name_input.value,
      oracle_text_input.value,
    );

    fill_grid_with_preprocessed(grid, preprocessed);
    fill_grid_with_tokens(grid, tokens);
  }

  update_output();
  card_name_input.addEventListener("input", update_output);
  oracle_text_input.addEventListener("input", update_output);
});

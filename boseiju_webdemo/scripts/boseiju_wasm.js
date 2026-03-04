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

      const json_result = JSON.parse(lexed);
      if (json_result.err) throw json_result.err;

      return json_result.tokens;
    },

    parse(card_name, oracle_text) {
      /* card_name is optional and should not block parsing */
      if (card_name == "") {
        card_name = "~";
      }

      const wasm_card_name = WasmString.fromJS(wasm, card_name);
      const wasm_oracle_text = WasmString.fromJS(wasm, oracle_text);

      const lexed_ptr = wasm.instance.exports.parse(
        wasm_card_name.ptr,
        wasm_card_name.len,
        wasm_oracle_text.ptr,
        wasm_oracle_text.len,
      );
      const wasm_parsed = WasmString.fromPtr(wasm, lexed_ptr);
      const parsed = wasm_parsed.toJS();

      wasm_card_name.free();
      wasm_oracle_text.free();
      wasm_parsed.free();

      const json_result = JSON.parse(parsed);
      if (json_result.err) throw json_result.err;

      return json_result.nodes;
    },
  };
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
    if (ch == "\n") {
      cell.textContent = NL_ICON;
      cell.classList.add("new-line");
    } else {
      cell.textContent = ch;
    }

    grid.appendChild(cell);
  }
}

function fill_grid_with_tokens(grid, tokens) {
  if (!tokens || tokens.length == 0) return;

  grid.style.gridTemplateRows = `repeat(2, auto)`;

  for (const tok of tokens) {
    const box = document.createElement("div");
    box.className = "token";
    box.style.gridRow = 2;
    box.style.gridColumn = `${tok.start + 1} / ${tok.end + 1}`;
    box.textContent = tok.display_text;
    box.title = tok.hover_text;

    grid.appendChild(box);
  }
}

function add_error_row(grid, layer, error) {
  const row = layer + 2; // 1 = chars, 2 = layer 0

  const start = error.start;
  const end = error.end;

  const currentRows = getComputedStyle(grid).gridTemplateRows.split(" ").length;
  if (row > currentRows) {
    grid.style.gridTemplateRows = `repeat(${row}, auto)`;
  }

  // Red underline span
  const underline = document.createElement("div");
  underline.className = "error-underline";
  underline.style.gridRow = row;
  underline.style.gridColumn = `${start + 1} / ${end + 1}`;
  grid.appendChild(underline);

  // Error message (row below)
  const message = document.createElement("div");
  message.className = "error-message";
  message.style.gridRow = row + 1;
  message.style.gridColumn = `1 / -1`;
  message.textContent = error.message;
  grid.appendChild(message);

  grid.style.gridTemplateRows = `repeat(${row + 1}, auto)`;
}

function fill_grid_with_nodes(grid, nodes) {
  if (!nodes || nodes.length === 0) return;

  const maxLayer = Math.max(...nodes.map((n) => n.layer));
  const orangeHue = 28;
  grid.style.gridTemplateRows = `repeat(${1 + maxLayer + 1}, auto)`;

  for (const node of nodes) {
    // Ignore invalid spans
    if (node.end <= node.start) continue;

    const box = document.createElement("div");
    box.className = "node";

    box.style.gridRow = node.layer + 2;
    box.style.gridColumn = `${node.start + 1} / ${node.end + 1}`;
    box.textContent = node.display_text;
    box.title = node.hover_text;

    const t = node.layer / (maxLayer || 1);
    const lightness = 80 - t * 25; // darker as layers grow
    box.style.background = `hsl(${orangeHue}, 85%, ${lightness}%)`;

    grid.appendChild(box);
  }
}

/* When the WASM file is loaded, add the event listeners */
load_wasm().then((wasm_module) => {
  const card_name_input = document.getElementById("card_name_input");
  const oracle_text_input = document.getElementById("oracle_text_input");
  const grid = document.getElementById("token_grid");
  const random_btn = document.getElementById("random_card_btn");

  function update_output() {
    const name = card_name_input.value;
    const oracle = oracle_text_input.value;

    const preprocessed = wasm_module.preprocess(name, oracle);
    fill_grid_with_preprocessed(grid, preprocessed);

    let tokens;
    try {
      tokens = wasm_module.lex(name, oracle);
    } catch (err) {
      add_error_row(grid, 0, err);
      return;
    }

    fill_grid_with_tokens(grid, tokens);

    let parsed;
    try {
      parsed = wasm_module.parse(name, oracle);
    } catch (err) {
      add_error_row(grid, 1, err);
      return;
    }

    fill_grid_with_nodes(grid, parsed);
  }

  function apply_random_card() {
    const index = Math.floor(Math.random() * EXAMPLE_CARDS.length);
    const card = EXAMPLE_CARDS[index];

    card_name_input.value = card.name;
    oracle_text_input.value = card.oracle;
    update_output();
  }

  apply_random_card();
  card_name_input.addEventListener("input", update_output);
  oracle_text_input.addEventListener("input", update_output);
  random_btn.addEventListener("click", apply_random_card);
});

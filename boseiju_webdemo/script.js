async function load_wasm() {
  const wasm = await WebAssembly.instantiateStreaming(
    fetch("boseiju.wasm"),
    {},
  );

  return {
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

function create_ability_tree_html(json_ab_tree) {
  function build_recursive_tree(node_name, rest) {
    if (Array.isArray(rest)) {
      var list_elems = "";
      for (const [i, elem] of rest.entries()) {
        list_elems += `${build_recursive_tree("Elem " + i, elem)}`;
      }
      return `<details open><summary>${node_name}</summary>${list_elems}</details>`;
    } else if (rest !== null && typeof rest === "object") {
      var list_elems = "";
      for (const [key, value] of Object.entries(rest)) {
        list_elems += build_recursive_tree(key, value);
      }
      return `<details open><summary>${node_name}</summary>${list_elems}</details>`;
    } else {
      return rest.toString();
    }
  }

  const tree = build_recursive_tree("Ability Tree", json_ab_tree.abilities);
  return `<div class="tree">${tree}</div>`;
}

function create_error_html(error) {
  return `<p class="error">${error}</p>`;
}

function create_html(parser_result) {
  if (parser_result.startsWith('{"abilities"')) {
    const json_ab_tree = JSON.parse(parser_result);
    return create_ability_tree_html(json_ab_tree);
  } else {
    return create_error_html(parser_result);
  }
}

/* When the WASM file is loaded, add the event listeners */
load_wasm().then((wasm_module) => {
  const card_name_input = document.getElementById("card_name_input");
  const oracle_text_input = document.getElementById("oracle_text_input");
  const ability_tree_output = document.getElementById("ability_tree_output");

  function update_output() {
    const result = wasm_module.parse_oracle_text(
      card_name_input.value,
      oracle_text_input.value,
    );
    ability_tree_output.innerHTML = create_html(result);
  }

  card_name_input.addEventListener("input", update_output);
  oracle_text_input.addEventListener("input", update_output);
});

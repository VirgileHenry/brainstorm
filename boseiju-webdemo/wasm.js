async function loadWasm() {
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

      const result_len = result_bytes.length;
      const result = new TextDecoder().decode(new Uint8Array(result_bytes));

      wasm.instance.exports.free(card_name_ptr, card_name_len);
      wasm.instance.exports.free(oracle_text_ptr, oracle_text_len);
      /* +1 for null terminating byte */
      //wasm.instance.exports.free(result_ptr, result_len + 1);

      return result;
    },
  };
}

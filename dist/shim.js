var __defProp = Object.defineProperty;
var __name = (target, value) => __defProp(target, "name", { value, configurable: true });

// build/index.js
import { WorkerEntrypoint as wt } from "cloudflare:workers";
import K from "./7a5ce4b37b50b5d272c307d35af25d5b775ac656-index_bg.wasm";
var v = class {
  static {
    __name(this, "v");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, rt.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_containerstartupoptions_free(t, 0);
  }
  get enableInternet() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let t = o.__wbg_get_containerstartupoptions_enableInternet(this.__wbg_ptr);
    return t === 16777215 ? void 0 : t !== 0;
  }
  get entrypoint() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let t = o.__wbg_get_containerstartupoptions_entrypoint(this.__wbg_ptr);
    var e = ct(t[0], t[1]).slice();
    return o.__wbindgen_free(t[0], t[1] * 4, 4), e;
  }
  get env() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.__wbg_get_containerstartupoptions_env(this.__wbg_ptr);
  }
  set enableInternet(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    o.__wbg_set_containerstartupoptions_enableInternet(this.__wbg_ptr, f(t) ? 16777215 : t ? 1 : 0);
  }
  set entrypoint(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let e = ut(t, o.__wbindgen_malloc), n = w;
    o.__wbg_set_containerstartupoptions_entrypoint(this.__wbg_ptr, e, n);
  }
  set env(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    o.__wbg_set_containerstartupoptions_env(this.__wbg_ptr, t);
  }
};
Symbol.dispose && (v.prototype[Symbol.dispose] = v.prototype.free);
var x = class {
  static {
    __name(this, "x");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, _t.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_intounderlyingbytesource_free(t, 0);
  }
  get autoAllocateChunkSize() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr) >>> 0;
  }
  cancel() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let t = this.__destroy_into_raw();
    o.intounderlyingbytesource_cancel(t);
  }
  pull(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.intounderlyingbytesource_pull(this.__wbg_ptr, t);
  }
  start(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    o.intounderlyingbytesource_start(this.__wbg_ptr, t);
  }
  get type() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let t = o.intounderlyingbytesource_type(this.__wbg_ptr);
    return tt[t];
  }
};
Symbol.dispose && (x.prototype[Symbol.dispose] = x.prototype.free);
var I = class {
  static {
    __name(this, "I");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, ot.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_intounderlyingsink_free(t, 0);
  }
  abort(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let e = this.__destroy_into_raw();
    return o.intounderlyingsink_abort(e, t);
  }
  close() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let t = this.__destroy_into_raw();
    return o.intounderlyingsink_close(t);
  }
  write(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.intounderlyingsink_write(this.__wbg_ptr, t);
  }
};
Symbol.dispose && (I.prototype[Symbol.dispose] = I.prototype.free);
var E = class {
  static {
    __name(this, "E");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, it.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_intounderlyingsource_free(t, 0);
  }
  cancel() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let t = this.__destroy_into_raw();
    o.intounderlyingsource_cancel(t);
  }
  pull(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.intounderlyingsource_pull(this.__wbg_ptr, t);
  }
};
Symbol.dispose && (E.prototype[Symbol.dispose] = E.prototype.free);
var m = class r {
  static {
    __name(this, "r");
  }
  static __wrap(t) {
    t = t >>> 0;
    let e = Object.create(r.prototype);
    return e.__wbg_ptr = t, e.__wbg_inst = i, q.register(e, { ptr: t, instance: i }, e), e;
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, q.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_minifyconfig_free(t, 0);
  }
  get css() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.__wbg_get_minifyconfig_css(this.__wbg_ptr) !== 0;
  }
  get html() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.__wbg_get_minifyconfig_html(this.__wbg_ptr) !== 0;
  }
  get js() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.__wbg_get_minifyconfig_js(this.__wbg_ptr) !== 0;
  }
  set css(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    o.__wbg_set_minifyconfig_css(this.__wbg_ptr, t);
  }
  set html(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    o.__wbg_set_minifyconfig_html(this.__wbg_ptr, t);
  }
  set js(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    o.__wbg_set_minifyconfig_js(this.__wbg_ptr, t);
  }
};
Symbol.dispose && (m.prototype[Symbol.dispose] = m.prototype.free);
var R = class {
  static {
    __name(this, "R");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, st.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_r2range_free(t, 0);
  }
  get length() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let t = o.__wbg_get_r2range_length(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  get offset() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let t = o.__wbg_get_r2range_offset(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  get suffix() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let t = o.__wbg_get_r2range_suffix(this.__wbg_ptr);
    return t[0] === 0 ? void 0 : t[1];
  }
  set length(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    o.__wbg_set_r2range_length(this.__wbg_ptr, !f(t), f(t) ? 0 : t);
  }
  set offset(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    o.__wbg_set_r2range_offset(this.__wbg_ptr, !f(t), f(t) ? 0 : t);
  }
  set suffix(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    o.__wbg_set_r2range_suffix(this.__wbg_ptr, !f(t), f(t) ? 0 : t);
  }
};
Symbol.dispose && (R.prototype[Symbol.dispose] = R.prototype.free);
var S = class {
  static {
    __name(this, "S");
  }
  __destroy_into_raw() {
    let t = this.__wbg_ptr;
    return this.__wbg_ptr = 0, D.unregister(this), t;
  }
  free() {
    let t = this.__destroy_into_raw();
    o.__wbg_sgproxystate_free(t, 0);
  }
  alarm() {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.sgproxystate_alarm(this.__wbg_ptr);
  }
  fetch(t) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.sgproxystate_fetch(this.__wbg_ptr, t);
  }
  constructor(t, e) {
    let n = o.sgproxystate_new(t, e);
    return this.__wbg_ptr = n >>> 0, this.__wbg_inst = i, D.register(this, { ptr: n >>> 0, instance: i }, this), this;
  }
  webSocketClose(t, e, n, _) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    let s = p(n, o.__wbindgen_malloc, o.__wbindgen_realloc), u = w;
    return o.sgproxystate_webSocketClose(this.__wbg_ptr, t, e, s, u, _);
  }
  webSocketError(t, e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.sgproxystate_webSocketError(this.__wbg_ptr, t, e);
  }
  webSocketMessage(t, e) {
    if (this.__wbg_inst !== void 0 && this.__wbg_inst !== i) throw new Error("Invalid stale object from previous Wasm instance");
    return o.sgproxystate_webSocketMessage(this.__wbg_ptr, t, e);
  }
};
Symbol.dispose && (S.prototype[Symbol.dispose] = S.prototype.free);
function V() {
  i++, y = null, j = null, typeof numBytesDecoded < "u" && (numBytesDecoded = 0), typeof w < "u" && (w = 0), o = new WebAssembly.Instance(K, J()).exports, o.__wbindgen_start();
}
__name(V, "V");
function $(r2, t, e) {
  return o.fetch(r2, t, e);
}
__name($, "$");
function C(r2) {
  o.setPanicHook(r2);
}
__name(C, "C");
function J() {
  return { __proto__: null, "./index_bg.js": { __proto__: null, __wbg_Error_83742b46f01ce22d: /* @__PURE__ */ __name(function(t, e) {
    return Error(g(t, e));
  }, "__wbg_Error_83742b46f01ce22d"), __wbg_Number_a5a435bd7bbec835: /* @__PURE__ */ __name(function(t) {
    return Number(t);
  }, "__wbg_Number_a5a435bd7bbec835"), __wbg_String_8564e559799eccda: /* @__PURE__ */ __name(function(t, e) {
    let n = String(e), _ = p(n, o.__wbindgen_malloc, o.__wbindgen_realloc), s = w;
    b().setInt32(t + 4, s, true), b().setInt32(t + 0, _, true);
  }, "__wbg_String_8564e559799eccda"), __wbg___wbindgen_bigint_get_as_i64_447a76b5c6ef7bda: /* @__PURE__ */ __name(function(t, e) {
    let n = e, _ = typeof n == "bigint" ? n : void 0;
    b().setBigInt64(t + 8, f(_) ? BigInt(0) : _, true), b().setInt32(t + 0, !f(_), true);
  }, "__wbg___wbindgen_bigint_get_as_i64_447a76b5c6ef7bda"), __wbg___wbindgen_boolean_get_c0f3f60bac5a78d1: /* @__PURE__ */ __name(function(t) {
    let e = t, n = typeof e == "boolean" ? e : void 0;
    return f(n) ? 16777215 : n ? 1 : 0;
  }, "__wbg___wbindgen_boolean_get_c0f3f60bac5a78d1"), __wbg___wbindgen_debug_string_5398f5bb970e0daa: /* @__PURE__ */ __name(function(t, e) {
    let n = T(e), _ = p(n, o.__wbindgen_malloc, o.__wbindgen_realloc), s = w;
    b().setInt32(t + 4, s, true), b().setInt32(t + 0, _, true);
  }, "__wbg___wbindgen_debug_string_5398f5bb970e0daa"), __wbg___wbindgen_in_41dbb8413020e076: /* @__PURE__ */ __name(function(t, e) {
    return t in e;
  }, "__wbg___wbindgen_in_41dbb8413020e076"), __wbg___wbindgen_is_bigint_e2141d4f045b7eda: /* @__PURE__ */ __name(function(t) {
    return typeof t == "bigint";
  }, "__wbg___wbindgen_is_bigint_e2141d4f045b7eda"), __wbg___wbindgen_is_function_3c846841762788c1: /* @__PURE__ */ __name(function(t) {
    return typeof t == "function";
  }, "__wbg___wbindgen_is_function_3c846841762788c1"), __wbg___wbindgen_is_object_781bc9f159099513: /* @__PURE__ */ __name(function(t) {
    let e = t;
    return typeof e == "object" && e !== null;
  }, "__wbg___wbindgen_is_object_781bc9f159099513"), __wbg___wbindgen_is_string_7ef6b97b02428fae: /* @__PURE__ */ __name(function(t) {
    return typeof t == "string";
  }, "__wbg___wbindgen_is_string_7ef6b97b02428fae"), __wbg___wbindgen_is_undefined_52709e72fb9f179c: /* @__PURE__ */ __name(function(t) {
    return t === void 0;
  }, "__wbg___wbindgen_is_undefined_52709e72fb9f179c"), __wbg___wbindgen_jsval_eq_ee31bfad3e536463: /* @__PURE__ */ __name(function(t, e) {
    return t === e;
  }, "__wbg___wbindgen_jsval_eq_ee31bfad3e536463"), __wbg___wbindgen_jsval_loose_eq_5bcc3bed3c69e72b: /* @__PURE__ */ __name(function(t, e) {
    return t == e;
  }, "__wbg___wbindgen_jsval_loose_eq_5bcc3bed3c69e72b"), __wbg___wbindgen_number_get_34bb9d9dcfa21373: /* @__PURE__ */ __name(function(t, e) {
    let n = e, _ = typeof n == "number" ? n : void 0;
    b().setFloat64(t + 8, f(_) ? 0 : _, true), b().setInt32(t + 0, !f(_), true);
  }, "__wbg___wbindgen_number_get_34bb9d9dcfa21373"), __wbg___wbindgen_string_get_395e606bd0ee4427: /* @__PURE__ */ __name(function(t, e) {
    let n = e, _ = typeof n == "string" ? n : void 0;
    var s = f(_) ? 0 : p(_, o.__wbindgen_malloc, o.__wbindgen_realloc), u = w;
    b().setInt32(t + 4, u, true), b().setInt32(t + 0, s, true);
  }, "__wbg___wbindgen_string_get_395e606bd0ee4427"), __wbg___wbindgen_throw_6ddd609b62940d55: /* @__PURE__ */ __name(function(t, e) {
    throw new Error(g(t, e));
  }, "__wbg___wbindgen_throw_6ddd609b62940d55"), __wbg__wbg_cb_unref_6b5b6b8576d35cb1: /* @__PURE__ */ __name(function(t) {
    t._wbg_cb_unref();
  }, "__wbg__wbg_cb_unref_6b5b6b8576d35cb1"), __wbg_append_608dfb635ee8998f: /* @__PURE__ */ __name(function() {
    return c(function(t, e, n, _, s) {
      t.append(g(e, n), g(_, s));
    }, arguments);
  }, "__wbg_append_608dfb635ee8998f"), __wbg_arrayBuffer_be5e37a0f7e6636d: /* @__PURE__ */ __name(function() {
    return c(function(t) {
      return t.arrayBuffer();
    }, arguments);
  }, "__wbg_arrayBuffer_be5e37a0f7e6636d"), __wbg_body_9fd07dcbb5b77be9: /* @__PURE__ */ __name(function(t) {
    let e = t.body;
    return f(e) ? 0 : d(e);
  }, "__wbg_body_9fd07dcbb5b77be9"), __wbg_body_ac1dad652946e6da: /* @__PURE__ */ __name(function(t) {
    let e = t.body;
    return f(e) ? 0 : d(e);
  }, "__wbg_body_ac1dad652946e6da"), __wbg_buffer_60b8043cd926067d: /* @__PURE__ */ __name(function(t) {
    return t.buffer;
  }, "__wbg_buffer_60b8043cd926067d"), __wbg_byobRequest_6342e5f2b232c0f9: /* @__PURE__ */ __name(function(t) {
    let e = t.byobRequest;
    return f(e) ? 0 : d(e);
  }, "__wbg_byobRequest_6342e5f2b232c0f9"), __wbg_byteLength_607b856aa6c5a508: /* @__PURE__ */ __name(function(t) {
    return t.byteLength;
  }, "__wbg_byteLength_607b856aa6c5a508"), __wbg_byteOffset_b26b63681c83856c: /* @__PURE__ */ __name(function(t) {
    return t.byteOffset;
  }, "__wbg_byteOffset_b26b63681c83856c"), __wbg_call_2d781c1f4d5c0ef8: /* @__PURE__ */ __name(function() {
    return c(function(t, e, n) {
      return t.call(e, n);
    }, arguments);
  }, "__wbg_call_2d781c1f4d5c0ef8"), __wbg_call_e133b57c9155d22c: /* @__PURE__ */ __name(function() {
    return c(function(t, e) {
      return t.call(e);
    }, arguments);
  }, "__wbg_call_e133b57c9155d22c"), __wbg_cancel_79b3bea07a1028e7: /* @__PURE__ */ __name(function(t) {
    return t.cancel();
  }, "__wbg_cancel_79b3bea07a1028e7"), __wbg_catch_d7ed0375ab6532a5: /* @__PURE__ */ __name(function(t, e) {
    return t.catch(e);
  }, "__wbg_catch_d7ed0375ab6532a5"), __wbg_cause_f02a23068e3256fa: /* @__PURE__ */ __name(function(t) {
    return t.cause;
  }, "__wbg_cause_f02a23068e3256fa"), __wbg_cf_3ad13ab095ee9a26: /* @__PURE__ */ __name(function() {
    return c(function(t) {
      let e = t.cf;
      return f(e) ? 0 : d(e);
    }, arguments);
  }, "__wbg_cf_3ad13ab095ee9a26"), __wbg_cf_c5a23ee8e524d1e1: /* @__PURE__ */ __name(function() {
    return c(function(t) {
      let e = t.cf;
      return f(e) ? 0 : d(e);
    }, arguments);
  }, "__wbg_cf_c5a23ee8e524d1e1"), __wbg_clone_b3f1c3a4cc3fb7c8: /* @__PURE__ */ __name(function() {
    return c(function(t) {
      return t.clone();
    }, arguments);
  }, "__wbg_clone_b3f1c3a4cc3fb7c8"), __wbg_close_690d36108c557337: /* @__PURE__ */ __name(function() {
    return c(function(t) {
      t.close();
    }, arguments);
  }, "__wbg_close_690d36108c557337"), __wbg_close_737b4b1fbc658540: /* @__PURE__ */ __name(function() {
    return c(function(t) {
      t.close();
    }, arguments);
  }, "__wbg_close_737b4b1fbc658540"), __wbg_constructor_b66dd7209f26ae23: /* @__PURE__ */ __name(function(t) {
    return t.constructor;
  }, "__wbg_constructor_b66dd7209f26ae23"), __wbg_done_08ce71ee07e3bd17: /* @__PURE__ */ __name(function(t) {
    return t.done;
  }, "__wbg_done_08ce71ee07e3bd17"), __wbg_enqueue_ec3552838b4b7fbf: /* @__PURE__ */ __name(function() {
    return c(function(t, e) {
      t.enqueue(e);
    }, arguments);
  }, "__wbg_enqueue_ec3552838b4b7fbf"), __wbg_entries_5b8fe91cea59610e: /* @__PURE__ */ __name(function(t) {
    return t.entries();
  }, "__wbg_entries_5b8fe91cea59610e"), __wbg_entries_e8a20ff8c9757101: /* @__PURE__ */ __name(function(t) {
    return Object.entries(t);
  }, "__wbg_entries_e8a20ff8c9757101"), __wbg_error_8d9a8e04cd1d3588: /* @__PURE__ */ __name(function(t) {
    console.error(t);
  }, "__wbg_error_8d9a8e04cd1d3588"), __wbg_error_a6fa202b58aa1cd3: /* @__PURE__ */ __name(function(t, e) {
    let n, _;
    try {
      n = t, _ = e, console.error(g(t, e));
    } finally {
      o.__wbindgen_free(n, _, 1);
    }
  }, "__wbg_error_a6fa202b58aa1cd3"), __wbg_error_cfce0f619500de52: /* @__PURE__ */ __name(function(t, e) {
    console.error(t, e);
  }, "__wbg_error_cfce0f619500de52"), __wbg_fetch_61f8a2c6a2c3af27: /* @__PURE__ */ __name(function() {
    return c(function(t, e) {
      return t.fetch(e);
    }, arguments);
  }, "__wbg_fetch_61f8a2c6a2c3af27"), __wbg_fetch_b967ec80e0e1eff8: /* @__PURE__ */ __name(function(t, e, n, _) {
    return t.fetch(g(e, n), _);
  }, "__wbg_fetch_b967ec80e0e1eff8"), __wbg_fetch_d77cded604d729e9: /* @__PURE__ */ __name(function(t, e, n) {
    return t.fetch(e, n);
  }, "__wbg_fetch_d77cded604d729e9"), __wbg_getRandomValues_76dfc69825c9c552: /* @__PURE__ */ __name(function() {
    return c(function(t, e) {
      globalThis.crypto.getRandomValues(P(t, e));
    }, arguments);
  }, "__wbg_getRandomValues_76dfc69825c9c552"), __wbg_getReader_9facd4f899beac89: /* @__PURE__ */ __name(function() {
    return c(function(t) {
      return t.getReader();
    }, arguments);
  }, "__wbg_getReader_9facd4f899beac89"), __wbg_get_326e41e095fb2575: /* @__PURE__ */ __name(function() {
    return c(function(t, e) {
      return Reflect.get(t, e);
    }, arguments);
  }, "__wbg_get_326e41e095fb2575"), __wbg_get_3ef1eba1850ade27: /* @__PURE__ */ __name(function() {
    return c(function(t, e) {
      return Reflect.get(t, e);
    }, arguments);
  }, "__wbg_get_3ef1eba1850ade27"), __wbg_get_a867a94064ecd263: /* @__PURE__ */ __name(function() {
    return c(function(t, e, n, _) {
      let s = e.get(g(n, _));
      var u = f(s) ? 0 : p(s, o.__wbindgen_malloc, o.__wbindgen_realloc), a = w;
      b().setInt32(t + 4, a, true), b().setInt32(t + 0, u, true);
    }, arguments);
  }, "__wbg_get_a867a94064ecd263"), __wbg_get_a8ee5c45dabc1b3b: /* @__PURE__ */ __name(function(t, e) {
    return t[e >>> 0];
  }, "__wbg_get_a8ee5c45dabc1b3b"), __wbg_get_c487d4dc23893b6a: /* @__PURE__ */ __name(function() {
    return c(function(t, e, n) {
      return t.get(g(e, n));
    }, arguments);
  }, "__wbg_get_c487d4dc23893b6a"), __wbg_get_done_d0ab690f8df5501f: /* @__PURE__ */ __name(function(t) {
    let e = t.done;
    return f(e) ? 16777215 : e ? 1 : 0;
  }, "__wbg_get_done_d0ab690f8df5501f"), __wbg_get_e086b35efbcd30db: /* @__PURE__ */ __name(function() {
    return c(function(t, e) {
      return t.get(e);
    }, arguments);
  }, "__wbg_get_e086b35efbcd30db"), __wbg_get_unchecked_329cfe50afab7352: /* @__PURE__ */ __name(function(t, e) {
    return t[e >>> 0];
  }, "__wbg_get_unchecked_329cfe50afab7352"), __wbg_get_value_548ae6adf5a174e4: /* @__PURE__ */ __name(function(t) {
    return t.value;
  }, "__wbg_get_value_548ae6adf5a174e4"), __wbg_get_with_ref_key_6412cf3094599694: /* @__PURE__ */ __name(function(t, e) {
    return t[e];
  }, "__wbg_get_with_ref_key_6412cf3094599694"), __wbg_headers_eb2234545f9ff993: /* @__PURE__ */ __name(function(t) {
    return t.headers;
  }, "__wbg_headers_eb2234545f9ff993"), __wbg_headers_fc8c672cd757e0fd: /* @__PURE__ */ __name(function(t) {
    return t.headers;
  }, "__wbg_headers_fc8c672cd757e0fd"), __wbg_idFromName_d07a17acff22c4b5: /* @__PURE__ */ __name(function() {
    return c(function(t, e, n) {
      return t.idFromName(g(e, n));
    }, arguments);
  }, "__wbg_idFromName_d07a17acff22c4b5"), __wbg_instanceof_ArrayBuffer_101e2bf31071a9f6: /* @__PURE__ */ __name(function(t) {
    let e;
    try {
      e = t instanceof ArrayBuffer;
    } catch {
      e = false;
    }
    return e;
  }, "__wbg_instanceof_ArrayBuffer_101e2bf31071a9f6"), __wbg_instanceof_Error_4691a5b466e32a80: /* @__PURE__ */ __name(function(t) {
    let e;
    try {
      e = t instanceof Error;
    } catch {
      e = false;
    }
    return e;
  }, "__wbg_instanceof_Error_4691a5b466e32a80"), __wbg_instanceof_ReadableStream_3becfcf3df22ee1a: /* @__PURE__ */ __name(function(t) {
    let e;
    try {
      e = t instanceof ReadableStream;
    } catch {
      e = false;
    }
    return e;
  }, "__wbg_instanceof_ReadableStream_3becfcf3df22ee1a"), __wbg_instanceof_Response_9b4d9fd451e051b1: /* @__PURE__ */ __name(function(t) {
    let e;
    try {
      e = t instanceof Response;
    } catch {
      e = false;
    }
    return e;
  }, "__wbg_instanceof_Response_9b4d9fd451e051b1"), __wbg_instanceof_Uint8Array_740438561a5b956d: /* @__PURE__ */ __name(function(t) {
    let e;
    try {
      e = t instanceof Uint8Array;
    } catch {
      e = false;
    }
    return e;
  }, "__wbg_instanceof_Uint8Array_740438561a5b956d"), __wbg_isArray_33b91feb269ff46e: /* @__PURE__ */ __name(function(t) {
    return Array.isArray(t);
  }, "__wbg_isArray_33b91feb269ff46e"), __wbg_isSafeInteger_ecd6a7f9c3e053cd: /* @__PURE__ */ __name(function(t) {
    return Number.isSafeInteger(t);
  }, "__wbg_isSafeInteger_ecd6a7f9c3e053cd"), __wbg_iterator_d8f549ec8fb061b1: /* @__PURE__ */ __name(function() {
    return Symbol.iterator;
  }, "__wbg_iterator_d8f549ec8fb061b1"), __wbg_json_23d07e6730d48b96: /* @__PURE__ */ __name(function() {
    return c(function(t) {
      return t.json();
    }, arguments);
  }, "__wbg_json_23d07e6730d48b96"), __wbg_keys_b75cee3388cca4aa: /* @__PURE__ */ __name(function(t) {
    return t.keys();
  }, "__wbg_keys_b75cee3388cca4aa"), __wbg_length_b3416cf66a5452c8: /* @__PURE__ */ __name(function(t) {
    return t.length;
  }, "__wbg_length_b3416cf66a5452c8"), __wbg_length_ea16607d7b61445b: /* @__PURE__ */ __name(function(t) {
    return t.length;
  }, "__wbg_length_ea16607d7b61445b"), __wbg_method_23aa7d0d6ec9a08f: /* @__PURE__ */ __name(function(t, e) {
    let n = e.method, _ = p(n, o.__wbindgen_malloc, o.__wbindgen_realloc), s = w;
    b().setInt32(t + 4, s, true), b().setInt32(t + 0, _, true);
  }, "__wbg_method_23aa7d0d6ec9a08f"), __wbg_minifyconfig_new: /* @__PURE__ */ __name(function(t) {
    return m.__wrap(t);
  }, "__wbg_minifyconfig_new"), __wbg_name_0bfa6ee19bce1bf9: /* @__PURE__ */ __name(function(t) {
    return t.name;
  }, "__wbg_name_0bfa6ee19bce1bf9"), __wbg_new_0837727332ac86ba: /* @__PURE__ */ __name(function() {
    return c(function() {
      return new Headers();
    }, arguments);
  }, "__wbg_new_0837727332ac86ba"), __wbg_new_227d7c05414eb861: /* @__PURE__ */ __name(function() {
    return new Error();
  }, "__wbg_new_227d7c05414eb861"), __wbg_new_49d5571bd3f0c4d4: /* @__PURE__ */ __name(function() {
    return /* @__PURE__ */ new Map();
  }, "__wbg_new_49d5571bd3f0c4d4"), __wbg_new_5f486cdf45a04d78: /* @__PURE__ */ __name(function(t) {
    return new Uint8Array(t);
  }, "__wbg_new_5f486cdf45a04d78"), __wbg_new_a70fbab9066b301f: /* @__PURE__ */ __name(function() {
    return new Array();
  }, "__wbg_new_a70fbab9066b301f"), __wbg_new_ab79df5bd7c26067: /* @__PURE__ */ __name(function() {
    return new Object();
  }, "__wbg_new_ab79df5bd7c26067"), __wbg_new_d15cb560a6a0e5f0: /* @__PURE__ */ __name(function(t, e) {
    return new Error(g(t, e));
  }, "__wbg_new_d15cb560a6a0e5f0"), __wbg_new_typed_aaaeaf29cf802876: /* @__PURE__ */ __name(function(t, e) {
    try {
      var n = { a: t, b: e }, _ = /* @__PURE__ */ __name((u, a) => {
        let l = n.a;
        n.a = 0;
        try {
          return Z(l, n.b, u, a);
        } finally {
          n.a = l;
        }
      }, "_");
      return new Promise(_);
    } finally {
      n.a = n.b = 0;
    }
  }, "__wbg_new_typed_aaaeaf29cf802876"), __wbg_new_with_byte_offset_and_length_b2ec5bf7b2f35743: /* @__PURE__ */ __name(function(t, e, n) {
    return new Uint8Array(t, e >>> 0, n >>> 0);
  }, "__wbg_new_with_byte_offset_and_length_b2ec5bf7b2f35743"), __wbg_new_with_length_825018a1616e9e55: /* @__PURE__ */ __name(function(t) {
    return new Uint8Array(t >>> 0);
  }, "__wbg_new_with_length_825018a1616e9e55"), __wbg_new_with_opt_buffer_source_and_init_cbf3b8468cedbba9: /* @__PURE__ */ __name(function() {
    return c(function(t, e) {
      return new Response(t, e);
    }, arguments);
  }, "__wbg_new_with_opt_buffer_source_and_init_cbf3b8468cedbba9"), __wbg_new_with_opt_readable_stream_and_init_15b79ab5fa39d080: /* @__PURE__ */ __name(function() {
    return c(function(t, e) {
      return new Response(t, e);
    }, arguments);
  }, "__wbg_new_with_opt_readable_stream_and_init_15b79ab5fa39d080"), __wbg_new_with_opt_str_and_init_a1ea8e111a765950: /* @__PURE__ */ __name(function() {
    return c(function(t, e, n) {
      return new Response(t === 0 ? void 0 : g(t, e), n);
    }, arguments);
  }, "__wbg_new_with_opt_str_and_init_a1ea8e111a765950"), __wbg_new_with_str_and_init_b4b54d1a819bc724: /* @__PURE__ */ __name(function() {
    return c(function(t, e, n) {
      return new Request(g(t, e), n);
    }, arguments);
  }, "__wbg_new_with_str_and_init_b4b54d1a819bc724"), __wbg_next_11b99ee6237339e3: /* @__PURE__ */ __name(function() {
    return c(function(t) {
      return t.next();
    }, arguments);
  }, "__wbg_next_11b99ee6237339e3"), __wbg_next_e01a967809d1aa68: /* @__PURE__ */ __name(function(t) {
    return t.next;
  }, "__wbg_next_e01a967809d1aa68"), __wbg_now_16f0c993d5dd6c27: /* @__PURE__ */ __name(function() {
    return Date.now();
  }, "__wbg_now_16f0c993d5dd6c27"), __wbg_parse_8d2820c01c23fd9b: /* @__PURE__ */ __name(function(t, e) {
    return Date.parse(g(t, e));
  }, "__wbg_parse_8d2820c01c23fd9b"), __wbg_prototypesetcall_d62e5099504357e6: /* @__PURE__ */ __name(function(t, e, n) {
    Uint8Array.prototype.set.call(P(t, e), n);
  }, "__wbg_prototypesetcall_d62e5099504357e6"), __wbg_put_bc9617e063a74715: /* @__PURE__ */ __name(function() {
    return c(function(t, e, n, _) {
      return t.put(g(e, n), _);
    }, arguments);
  }, "__wbg_put_bc9617e063a74715"), __wbg_queueMicrotask_0c399741342fb10f: /* @__PURE__ */ __name(function(t) {
    return t.queueMicrotask;
  }, "__wbg_queueMicrotask_0c399741342fb10f"), __wbg_queueMicrotask_a082d78ce798393e: /* @__PURE__ */ __name(function(t) {
    queueMicrotask(t);
  }, "__wbg_queueMicrotask_a082d78ce798393e"), __wbg_read_7f593a961a7f80ed: /* @__PURE__ */ __name(function(t) {
    return t.read();
  }, "__wbg_read_7f593a961a7f80ed"), __wbg_releaseLock_ef7766a5da654ff8: /* @__PURE__ */ __name(function(t) {
    t.releaseLock();
  }, "__wbg_releaseLock_ef7766a5da654ff8"), __wbg_resolve_ae8d83246e5bcc12: /* @__PURE__ */ __name(function(t) {
    return Promise.resolve(t);
  }, "__wbg_resolve_ae8d83246e5bcc12"), __wbg_respond_e286ee502e7cf7e4: /* @__PURE__ */ __name(function() {
    return c(function(t, e) {
      t.respond(e >>> 0);
    }, arguments);
  }, "__wbg_respond_e286ee502e7cf7e4"), __wbg_set_282384002438957f: /* @__PURE__ */ __name(function(t, e, n) {
    t[e >>> 0] = n;
  }, "__wbg_set_282384002438957f"), __wbg_set_6be42768c690e380: /* @__PURE__ */ __name(function(t, e, n) {
    t[e] = n;
  }, "__wbg_set_6be42768c690e380"), __wbg_set_7eaa4f96924fd6b3: /* @__PURE__ */ __name(function() {
    return c(function(t, e, n) {
      return Reflect.set(t, e, n);
    }, arguments);
  }, "__wbg_set_7eaa4f96924fd6b3"), __wbg_set_8c0b3ffcf05d61c2: /* @__PURE__ */ __name(function(t, e, n) {
    t.set(P(e, n));
  }, "__wbg_set_8c0b3ffcf05d61c2"), __wbg_set_bf7251625df30a02: /* @__PURE__ */ __name(function(t, e, n) {
    return t.set(e, n);
  }, "__wbg_set_bf7251625df30a02"), __wbg_set_body_a3d856b097dfda04: /* @__PURE__ */ __name(function(t, e) {
    t.body = e;
  }, "__wbg_set_body_a3d856b097dfda04"), __wbg_set_cache_ec7e430c6056ebda: /* @__PURE__ */ __name(function(t, e) {
    t.cache = et[e];
  }, "__wbg_set_cache_ec7e430c6056ebda"), __wbg_set_e09648bea3f1af1e: /* @__PURE__ */ __name(function() {
    return c(function(t, e, n, _, s) {
      t.set(g(e, n), g(_, s));
    }, arguments);
  }, "__wbg_set_e09648bea3f1af1e"), __wbg_set_headers_3c8fecc693b75327: /* @__PURE__ */ __name(function(t, e) {
    t.headers = e;
  }, "__wbg_set_headers_3c8fecc693b75327"), __wbg_set_headers_bf56980ea1a65acb: /* @__PURE__ */ __name(function(t, e) {
    t.headers = e;
  }, "__wbg_set_headers_bf56980ea1a65acb"), __wbg_set_method_8c015e8bcafd7be1: /* @__PURE__ */ __name(function(t, e, n) {
    t.method = g(e, n);
  }, "__wbg_set_method_8c015e8bcafd7be1"), __wbg_set_redirect_c7b340412376b11a: /* @__PURE__ */ __name(function(t, e) {
    t.redirect = nt[e];
  }, "__wbg_set_redirect_c7b340412376b11a"), __wbg_set_signal_0cebecb698f25d21: /* @__PURE__ */ __name(function(t, e) {
    t.signal = e;
  }, "__wbg_set_signal_0cebecb698f25d21"), __wbg_set_status_b80d37d9d23276c4: /* @__PURE__ */ __name(function(t, e) {
    t.status = e;
  }, "__wbg_set_status_b80d37d9d23276c4"), __wbg_stack_3b0d974bbf31e44f: /* @__PURE__ */ __name(function(t, e) {
    let n = e.stack, _ = p(n, o.__wbindgen_malloc, o.__wbindgen_realloc), s = w;
    b().setInt32(t + 4, s, true), b().setInt32(t + 0, _, true);
  }, "__wbg_stack_3b0d974bbf31e44f"), __wbg_static_accessor_GLOBAL_8adb955bd33fac2f: /* @__PURE__ */ __name(function() {
    let t = typeof global > "u" ? null : global;
    return f(t) ? 0 : d(t);
  }, "__wbg_static_accessor_GLOBAL_8adb955bd33fac2f"), __wbg_static_accessor_GLOBAL_THIS_ad356e0db91c7913: /* @__PURE__ */ __name(function() {
    let t = typeof globalThis > "u" ? null : globalThis;
    return f(t) ? 0 : d(t);
  }, "__wbg_static_accessor_GLOBAL_THIS_ad356e0db91c7913"), __wbg_static_accessor_SELF_f207c857566db248: /* @__PURE__ */ __name(function() {
    let t = typeof self > "u" ? null : self;
    return f(t) ? 0 : d(t);
  }, "__wbg_static_accessor_SELF_f207c857566db248"), __wbg_static_accessor_WINDOW_bb9f1ba69d61b386: /* @__PURE__ */ __name(function() {
    let t = typeof window > "u" ? null : window;
    return f(t) ? 0 : d(t);
  }, "__wbg_static_accessor_WINDOW_bb9f1ba69d61b386"), __wbg_status_318629ab93a22955: /* @__PURE__ */ __name(function(t) {
    return t.status;
  }, "__wbg_status_318629ab93a22955"), __wbg_storage_e4166e2e53c3e693: /* @__PURE__ */ __name(function() {
    return c(function(t) {
      return t.storage;
    }, arguments);
  }, "__wbg_storage_e4166e2e53c3e693"), __wbg_then_098abe61755d12f6: /* @__PURE__ */ __name(function(t, e) {
    return t.then(e);
  }, "__wbg_then_098abe61755d12f6"), __wbg_then_9e335f6dd892bc11: /* @__PURE__ */ __name(function(t, e, n) {
    return t.then(e, n);
  }, "__wbg_then_9e335f6dd892bc11"), __wbg_toString_fca8b5e46235cfb4: /* @__PURE__ */ __name(function(t) {
    return t.toString();
  }, "__wbg_toString_fca8b5e46235cfb4"), __wbg_url_b6f96880b733816c: /* @__PURE__ */ __name(function(t, e) {
    let n = e.url, _ = p(n, o.__wbindgen_malloc, o.__wbindgen_realloc), s = w;
    b().setInt32(t + 4, s, true), b().setInt32(t + 0, _, true);
  }, "__wbg_url_b6f96880b733816c"), __wbg_value_21fc78aab0322612: /* @__PURE__ */ __name(function(t) {
    return t.value;
  }, "__wbg_value_21fc78aab0322612"), __wbg_view_f68a712e7315f8b2: /* @__PURE__ */ __name(function(t) {
    let e = t.view;
    return f(e) ? 0 : d(e);
  }, "__wbg_view_f68a712e7315f8b2"), __wbg_webSocket_5f67380bd2dbf430: /* @__PURE__ */ __name(function() {
    return c(function(t) {
      let e = t.webSocket;
      return f(e) ? 0 : d(e);
    }, arguments);
  }, "__wbg_webSocket_5f67380bd2dbf430"), __wbindgen_cast_0000000000000001: /* @__PURE__ */ __name(function(t, e) {
    return H(t, e, o.wasm_bindgen__closure__destroy__hf4e98ce8a7227b4c, X);
  }, "__wbindgen_cast_0000000000000001"), __wbindgen_cast_0000000000000002: /* @__PURE__ */ __name(function(t, e) {
    return H(t, e, o.wasm_bindgen__closure__destroy__h17bf4eaa5d216ff9, Y);
  }, "__wbindgen_cast_0000000000000002"), __wbindgen_cast_0000000000000003: /* @__PURE__ */ __name(function(t) {
    return t;
  }, "__wbindgen_cast_0000000000000003"), __wbindgen_cast_0000000000000004: /* @__PURE__ */ __name(function(t, e) {
    return g(t, e);
  }, "__wbindgen_cast_0000000000000004"), __wbindgen_cast_0000000000000005: /* @__PURE__ */ __name(function(t) {
    return BigInt.asUintN(64, t);
  }, "__wbindgen_cast_0000000000000005"), __wbindgen_cast_0000000000000006: /* @__PURE__ */ __name(function(t, e) {
    var n = P(t, e).slice();
    return o.__wbindgen_free(t, e * 1, 1), n;
  }, "__wbindgen_cast_0000000000000006"), __wbindgen_init_externref_table: /* @__PURE__ */ __name(function() {
    let t = o.__wbindgen_externrefs, e = t.grow(4);
    t.set(0, void 0), t.set(e + 0, void 0), t.set(e + 1, null), t.set(e + 2, true), t.set(e + 3, false);
  }, "__wbindgen_init_externref_table") } };
}
__name(J, "J");
function X(r2, t, e) {
  o.wasm_bindgen__convert__closures_____invoke__hd4c18b7524f02476(r2, t, e);
}
__name(X, "X");
function Y(r2, t, e) {
  let n = o.wasm_bindgen__convert__closures_____invoke__h2a3ae95996c0969c(r2, t, e);
  if (n[1]) throw ft(n[0]);
}
__name(Y, "Y");
function Z(r2, t, e, n) {
  o.wasm_bindgen__convert__closures_____invoke__h4a9d2138e3739ee5(r2, t, e, n);
}
__name(Z, "Z");
var tt = ["bytes"];
var et = ["default", "no-store", "reload", "no-cache", "force-cache", "only-if-cached"];
var nt = ["follow", "error", "manual"];
var i = 0;
var rt = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: r2, instance: t }) => {
  t === i && o.__wbg_containerstartupoptions_free(r2 >>> 0, 1);
});
var _t = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: r2, instance: t }) => {
  t === i && o.__wbg_intounderlyingbytesource_free(r2 >>> 0, 1);
});
var ot = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: r2, instance: t }) => {
  t === i && o.__wbg_intounderlyingsink_free(r2 >>> 0, 1);
});
var it = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: r2, instance: t }) => {
  t === i && o.__wbg_intounderlyingsource_free(r2 >>> 0, 1);
});
var q = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: r2, instance: t }) => {
  t === i && o.__wbg_minifyconfig_free(r2 >>> 0, 1);
});
var st = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: r2, instance: t }) => {
  t === i && o.__wbg_r2range_free(r2 >>> 0, 1);
});
var D = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry(({ ptr: r2, instance: t }) => {
  t === i && o.__wbg_sgproxystate_free(r2 >>> 0, 1);
});
function d(r2) {
  let t = o.__externref_table_alloc();
  return o.__wbindgen_externrefs.set(t, r2), t;
}
__name(d, "d");
var N = typeof FinalizationRegistry > "u" ? { register: /* @__PURE__ */ __name(() => {
}, "register"), unregister: /* @__PURE__ */ __name(() => {
}, "unregister") } : new FinalizationRegistry((r2) => {
  r2.instance === i && r2.dtor(r2.a, r2.b);
});
function T(r2) {
  let t = typeof r2;
  if (t == "number" || t == "boolean" || r2 == null) return `${r2}`;
  if (t == "string") return `"${r2}"`;
  if (t == "symbol") {
    let _ = r2.description;
    return _ == null ? "Symbol" : `Symbol(${_})`;
  }
  if (t == "function") {
    let _ = r2.name;
    return typeof _ == "string" && _.length > 0 ? `Function(${_})` : "Function";
  }
  if (Array.isArray(r2)) {
    let _ = r2.length, s = "[";
    _ > 0 && (s += T(r2[0]));
    for (let u = 1; u < _; u++) s += ", " + T(r2[u]);
    return s += "]", s;
  }
  let e = /\[object ([^\]]+)\]/.exec(toString.call(r2)), n;
  if (e && e.length > 1) n = e[1];
  else return toString.call(r2);
  if (n == "Object") try {
    return "Object(" + JSON.stringify(r2) + ")";
  } catch {
    return "Object";
  }
  return r2 instanceof Error ? `${r2.name}: ${r2.message}
${r2.stack}` : n;
}
__name(T, "T");
function ct(r2, t) {
  r2 = r2 >>> 0;
  let e = b(), n = [];
  for (let _ = r2; _ < r2 + 4 * t; _ += 4) n.push(o.__wbindgen_externrefs.get(e.getUint32(_, true)));
  return o.__externref_drop_slice(r2, t), n;
}
__name(ct, "ct");
function P(r2, t) {
  return r2 = r2 >>> 0, k().subarray(r2 / 1, r2 / 1 + t);
}
__name(P, "P");
var y = null;
function b() {
  return (y === null || y.buffer.detached === true || y.buffer.detached === void 0 && y.buffer !== o.memory.buffer) && (y = new DataView(o.memory.buffer)), y;
}
__name(b, "b");
function g(r2, t) {
  return r2 = r2 >>> 0, at(r2, t);
}
__name(g, "g");
var j = null;
function k() {
  return (j === null || j.byteLength === 0) && (j = new Uint8Array(o.memory.buffer)), j;
}
__name(k, "k");
function c(r2, t) {
  try {
    return r2.apply(this, t);
  } catch (e) {
    let n = d(e);
    o.__wbindgen_exn_store(n);
  }
}
__name(c, "c");
function f(r2) {
  return r2 == null;
}
__name(f, "f");
function H(r2, t, e, n) {
  let _ = { a: r2, b: t, cnt: 1, dtor: e, instance: i }, s = /* @__PURE__ */ __name((...u) => {
    if (_.instance !== i) throw new Error("Cannot invoke closure from previous WASM instance");
    _.cnt++;
    let a = _.a;
    _.a = 0;
    try {
      return n(a, _.b, ...u);
    } finally {
      _.a = a, s._wbg_cb_unref();
    }
  }, "s");
  return s._wbg_cb_unref = () => {
    --_.cnt === 0 && (_.dtor(_.a, _.b), _.a = 0, N.unregister(_));
  }, N.register(s, _, _), s;
}
__name(H, "H");
function ut(r2, t) {
  let e = t(r2.length * 4, 4) >>> 0;
  for (let n = 0; n < r2.length; n++) {
    let _ = d(r2[n]);
    b().setUint32(e + 4 * n, _, true);
  }
  return w = r2.length, e;
}
__name(ut, "ut");
function p(r2, t, e) {
  if (e === void 0) {
    let a = W.encode(r2), l = t(a.length, 1) >>> 0;
    return k().subarray(l, l + a.length).set(a), w = a.length, l;
  }
  let n = r2.length, _ = t(n, 1) >>> 0, s = k(), u = 0;
  for (; u < n; u++) {
    let a = r2.charCodeAt(u);
    if (a > 127) break;
    s[_ + u] = a;
  }
  if (u !== n) {
    u !== 0 && (r2 = r2.slice(u)), _ = e(_, n, n = u + r2.length * 3, 1) >>> 0;
    let a = k().subarray(_ + u, _ + n), l = W.encodeInto(r2, a);
    u += l.written, _ = e(_, n, u, 1) >>> 0;
  }
  return w = u, _;
}
__name(p, "p");
function ft(r2) {
  let t = o.__wbindgen_externrefs.get(r2);
  return o.__externref_table_dealloc(r2), t;
}
__name(ft, "ft");
var G = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
G.decode();
function at(r2, t) {
  return G.decode(k().subarray(r2, r2 + t));
}
__name(at, "at");
var W = new TextEncoder();
"encodeInto" in W || (W.encodeInto = function(r2, t) {
  let e = W.encode(r2);
  return t.set(e), { read: r2.length, written: e.length };
});
var w = 0;
var bt = new WebAssembly.Instance(K, J());
var o = bt.exports;
o.__wbindgen_start();
Error.stackTraceLimit = 100;
var A = false;
function Q() {
  C && C(function(r2) {
    let t = new Error("Rust panic: " + r2);
    console.error("Critical", t), A = true;
  });
}
__name(Q, "Q");
Q();
var z = 0;
function L() {
  A && (console.log("Reinitializing Wasm application"), V(), A = false, Q(), z++);
}
__name(L, "L");
addEventListener("error", (r2) => {
  B(r2.error);
});
function B(r2) {
  r2 instanceof WebAssembly.RuntimeError && (console.error("Critical", r2), A = true);
}
__name(B, "B");
var O = class extends wt {
  static {
    __name(this, "O");
  }
};
O.prototype.fetch = function(t) {
  return $.call(this, t, this.env, this.ctx);
};
var dt = { set: /* @__PURE__ */ __name((r2, t, e, n) => Reflect.set(r2.instance, t, e, n), "set"), has: /* @__PURE__ */ __name((r2, t) => Reflect.has(r2.instance, t), "has"), deleteProperty: /* @__PURE__ */ __name((r2, t) => Reflect.deleteProperty(r2.instance, t), "deleteProperty"), apply: /* @__PURE__ */ __name((r2, t, e) => Reflect.apply(r2.instance, t, e), "apply"), construct: /* @__PURE__ */ __name((r2, t, e) => Reflect.construct(r2.instance, t, e), "construct"), getPrototypeOf: /* @__PURE__ */ __name((r2) => Reflect.getPrototypeOf(r2.instance), "getPrototypeOf"), setPrototypeOf: /* @__PURE__ */ __name((r2, t) => Reflect.setPrototypeOf(r2.instance, t), "setPrototypeOf"), isExtensible: /* @__PURE__ */ __name((r2) => Reflect.isExtensible(r2.instance), "isExtensible"), preventExtensions: /* @__PURE__ */ __name((r2) => Reflect.preventExtensions(r2.instance), "preventExtensions"), getOwnPropertyDescriptor: /* @__PURE__ */ __name((r2, t) => Reflect.getOwnPropertyDescriptor(r2.instance, t), "getOwnPropertyDescriptor"), defineProperty: /* @__PURE__ */ __name((r2, t, e) => Reflect.defineProperty(r2.instance, t, e), "defineProperty"), ownKeys: /* @__PURE__ */ __name((r2) => Reflect.ownKeys(r2.instance), "ownKeys") };
var h = { construct(r2, t, e) {
  try {
    L();
    let n = { instance: Reflect.construct(r2, t, e), instanceId: z, ctor: r2, args: t, newTarget: e };
    return new Proxy(n, { ...dt, get(_, s, u) {
      _.instanceId !== z && (_.instance = Reflect.construct(_.ctor, _.args, _.newTarget), _.instanceId = z);
      let a = Reflect.get(_.instance, s, u);
      return typeof a != "function" ? a : a.constructor === Function ? new Proxy(a, { apply(l, M, U) {
        L();
        try {
          return l.apply(M, U);
        } catch (F) {
          throw B(F), F;
        }
      } }) : new Proxy(a, { async apply(l, M, U) {
        L();
        try {
          return await l.apply(M, U);
        } catch (F) {
          throw B(F), F;
        }
      } });
    } });
  } catch (n) {
    throw A = true, n;
  }
} };
var ht = new Proxy(O, h);
var yt = new Proxy(v, h);
var mt = new Proxy(x, h);
var vt = new Proxy(I, h);
var xt = new Proxy(E, h);
var It = new Proxy(m, h);
var Et = new Proxy(R, h);
var Rt = new Proxy(S, h);
export {
  yt as ContainerStartupOptions,
  mt as IntoUnderlyingByteSource,
  vt as IntoUnderlyingSink,
  xt as IntoUnderlyingSource,
  It as MinifyConfig,
  Et as R2Range,
  Rt as SgproxyState,
  ht as default
};
//# sourceMappingURL=shim.js.map

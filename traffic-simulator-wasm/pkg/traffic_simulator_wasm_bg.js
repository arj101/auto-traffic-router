import * as wasm from './traffic_simulator_wasm_bg.wasm';

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
*/
export class Simulator {

    static __wrap(ptr) {
        const obj = Object.create(Simulator.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_simulator_free(ptr);
    }
    /**
    * @returns {StatsManager}
    */
    get stats() {
        const ret = wasm.__wbg_get_simulator_stats(this.ptr);
        return StatsManager.__wrap(ret);
    }
    /**
    * @param {StatsManager} arg0
    */
    set stats(arg0) {
        _assertClass(arg0, StatsManager);
        var ptr0 = arg0.ptr;
        arg0.ptr = 0;
        wasm.__wbg_set_simulator_stats(this.ptr, ptr0);
    }
    /**
    * @returns {Simulator}
    */
    static new() {
        const ret = wasm.simulator_new();
        return Simulator.__wrap(ret);
    }
    /**
    * @param {number} n
    */
    spawn_vehicles(n) {
        wasm.simulator_spawn_vehicles(this.ptr, n);
    }
    /**
    * @param {number} scale
    * @param {number} density_coeff
    * @param {number} vel_coeff
    */
    tick(scale, density_coeff, vel_coeff) {
        wasm.simulator_tick(this.ptr, scale, density_coeff, vel_coeff);
    }
    /**
    * @param {number} id
    * @param {number} x
    * @param {number} y
    * @param {number | undefined} weight
    */
    create_intersection(id, x, y, weight) {
        wasm.simulator_create_intersection(this.ptr, id, x, y, !isLikeNone(weight), isLikeNone(weight) ? 0 : weight);
    }
    /**
    * @param {number} n1
    * @param {number} n2
    */
    create_road(n1, n2) {
        wasm.simulator_create_road(this.ptr, n1, n2);
    }
    /**
    * @param {number} n1
    * @param {number} n2
    */
    delete_road(n1, n2) {
        wasm.simulator_delete_road(this.ptr, n1, n2);
    }
    /**
    * @param {number} id
    */
    delete_intersection(id) {
        wasm.simulator_delete_intersection(this.ptr, id);
    }
    /**
    * @returns {number}
    */
    get_vehicle_render_buff_ptr() {
        const ret = wasm.simulator_get_vehicle_render_buff_ptr(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    get_vehicle_render_buff_len() {
        const ret = wasm.simulator_get_vehicle_render_buff_len(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {Uint32Array}
    */
    get_map_render_data() {
        const ret = wasm.simulator_get_map_render_data(this.ptr);
        return takeObject(ret);
    }
}
/**
*/
export class StatsManager {

    static __wrap(ptr) {
        const obj = Object.create(StatsManager.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_statsmanager_free(ptr);
    }
    /**
    * @returns {number}
    */
    get completed_vehicle_count() {
        const ret = wasm.__wbg_get_statsmanager_completed_vehicle_count(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set completed_vehicle_count(arg0) {
        wasm.__wbg_set_statsmanager_completed_vehicle_count(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get avg_flux() {
        const ret = wasm.__wbg_get_statsmanager_avg_flux(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set avg_flux(arg0) {
        wasm.__wbg_set_statsmanager_avg_flux(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get flux_avg_clear_threshold() {
        const ret = wasm.__wbg_get_statsmanager_flux_avg_clear_threshold(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set flux_avg_clear_threshold(arg0) {
        wasm.__wbg_set_statsmanager_flux_avg_clear_threshold(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get avg_vel() {
        const ret = wasm.__wbg_get_statsmanager_avg_vel(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set avg_vel(arg0) {
        wasm.__wbg_set_statsmanager_avg_vel(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get vel_avg_clear_threshold() {
        const ret = wasm.__wbg_get_statsmanager_vel_avg_clear_threshold(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set vel_avg_clear_threshold(arg0) {
        wasm.__wbg_set_statsmanager_vel_avg_clear_threshold(this.ptr, arg0);
    }
}

export function __wbg_new_abda76e883ba8a5f() {
    const ret = new Error();
    return addHeapObject(ret);
};

export function __wbg_stack_658279fe44541cf6(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_error_f851667af71bcfc6(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(arg0, arg1);
    }
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbg_crypto_e1d53a1d73fb10b8(arg0) {
    const ret = getObject(arg0).crypto;
    return addHeapObject(ret);
};

export function __wbindgen_is_object(arg0) {
    const val = getObject(arg0);
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};

export function __wbg_process_038c26bf42b093f8(arg0) {
    const ret = getObject(arg0).process;
    return addHeapObject(ret);
};

export function __wbg_versions_ab37218d2f0b24a8(arg0) {
    const ret = getObject(arg0).versions;
    return addHeapObject(ret);
};

export function __wbg_node_080f4b19d15bc1fe(arg0) {
    const ret = getObject(arg0).node;
    return addHeapObject(ret);
};

export function __wbindgen_is_string(arg0) {
    const ret = typeof(getObject(arg0)) === 'string';
    return ret;
};

export function __wbg_require_78a3dcfbdba9cbce() { return handleError(function () {
    const ret = module.require;
    return addHeapObject(ret);
}, arguments) };

export function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export function __wbg_msCrypto_6e7d3e1f92610cbb(arg0) {
    const ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
};

export function __wbg_getRandomValues_805f1c3d65988a5a() { return handleError(function (arg0, arg1) {
    getObject(arg0).getRandomValues(getObject(arg1));
}, arguments) };

export function __wbg_randomFillSync_6894564c2c334c42() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
}, arguments) };

export function __wbindgen_is_function(arg0) {
    const ret = typeof(getObject(arg0)) === 'function';
    return ret;
};

export function __wbg_newnoargs_b5b063fc6c2f0376(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbg_call_97ae9d8645dc388b() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_self_6d479506f72c6a71() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_window_f2557cc78490aceb() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_globalThis_7f206bda628d5286() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_global_ba75c50d1cf384f4() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };

export function __wbindgen_is_undefined(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};

export function __wbg_call_168da88779e35f61() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_buffer_3f3d764d4747d564(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};

export function __wbg_new_8c3f0052272a457a(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};

export function __wbg_set_83db9690f9353e79(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};

export function __wbg_length_9e1ae1900cb0fbd5(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};

export function __wbg_newwithlength_f5933855e4f48a19(arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_subarray_58ad4efbb5bcb886(arg0, arg1, arg2) {
    const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_newwithlength_df70128b5db7393e(arg0) {
    const ret = new Uint32Array(arg0 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_setindex_da2d8ce2f156d4c7(arg0, arg1, arg2) {
    getObject(arg0)[arg1 >>> 0] = arg2 >>> 0;
};

export function __wbindgen_object_clone_ref(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_memory() {
    const ret = wasm.memory;
    return addHeapObject(ret);
};


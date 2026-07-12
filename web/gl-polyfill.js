// GL polyfill for a macroquad/miniquad version skew.
//
// macroquad 0.4.14 pins miniquad =0.4.8, but the mq_js_bundle.js it ships is
// missing six WebGL wrapper functions that miniquad 0.4.8's wasm imports for
// render-target / renderbuffer support. Without them, the bundle installs
// warn-only stubs, glCheckFramebufferStatus() returns undefined, and
// miniquad's `assert!(fb_status != 0)` panics ("unreachable") the moment the
// game creates its 320x200 render target — a black screen on every device.
//
// These implementations are copied verbatim from miniquad 0.4.8's js/gl.js and
// reference the bundle's globals (gl, GL, getArray). This script must run after
// mq_js_bundle.js and before load() so add_missing_functions_stabs() sees them.
(function () {
    if (typeof importObject === "undefined" || !importObject.env) {
        console.error("gl-polyfill: importObject not ready; load order is wrong");
        return;
    }
    var env = importObject.env;

    function add(name, fn) {
        if (env[name] === undefined) {
            env[name] = fn;
        }
    }

    add("glCheckFramebufferStatus", function (target) {
        return gl.checkFramebufferStatus(target);
    });

    add("glFramebufferRenderbuffer", function (target, attachment, renderbuffertarget, renderbuffer) {
        GL.validateGLObjectID(GL.renderbuffers, renderbuffer, "glFramebufferRenderbuffer", "renderbuffer");
        gl.framebufferRenderbuffer(target, attachment, renderbuffertarget, GL.renderbuffers[renderbuffer]);
    });

    add("glDeleteRenderbuffers", function (n, renderbuffers) {
        for (var i = 0; i < n; i++) {
            var id = getArray(renderbuffers + i * 4, Uint32Array, 1)[0];
            var buffer = GL.renderbuffers[id];
            if (!buffer) continue;
            gl.deleteRenderbuffer(buffer);
            buffer.name = 0;
            GL.renderbuffers[id] = null;
        }
    });

    // The three below are WebGL2-only (MSAA-resolve helpers). miniquad imports
    // and calls them, but macroquad may run on a WebGL1 context where they don't
    // exist. The stock bundle stubbed every missing import to a no-op; we do the
    // same here so a WebGL1 context degrades gracefully instead of throwing,
    // while still using the real call when WebGL2 is available.
    add("glRenderbufferStorageMultisample", function (target, samples, internalformat, width, height) {
        if (gl.renderbufferStorageMultisample) {
            gl.renderbufferStorageMultisample(target, samples, internalformat, width, height);
        }
    });

    add("glReadBuffer", function (source) {
        if (gl.readBuffer) {
            gl.readBuffer(source);
        }
    });

    add("glBlitFramebuffer", function (srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter) {
        if (gl.blitFramebuffer) {
            gl.blitFramebuffer(srcX0, srcY0, srcX1, srcY1, dstX0, dstY0, dstX1, dstY1, mask, filter);
        }
    });
})();

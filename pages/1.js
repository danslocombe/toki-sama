(window.webpackJsonp=window.webpackJsonp||[]).push([[1],[,function(e,t,n){"use strict";n.r(t);var r=n(2);const o=window.location.search;new URLSearchParams(o).get("q");var c,u=document.getElementById("entry"),i=document.getElementById("results");Promise.all([fetch("/pu.csv").then(e=>e.text()),fetch("/nimi_pu.txt").then(e=>e.text()),fetch("/compounds.txt").then(e=>e.text()),fetch("/generated_day2.tsv").then(e=>e.text())]).then(([e,t,n,o])=>{c=new r.a(e,t,n,o),console.log("Finished search init!"),u.removeAttribute("disabled"),u.setAttribute("placeholder","teacher"),u.focus(),u.addEventListener("input",e=>{t=c.search(e.target.value),console.log(t),i.innerHTML=t;var t})})},function(e,t,n){"use strict";(function(e){n.d(t,"a",(function(){return g})),n.d(t,"d",(function(){return v})),n.d(t,"b",(function(){return x})),n.d(t,"c",(function(){return _})),n.d(t,"e",(function(){return m}));var r=n(4);let o=new("undefined"==typeof TextDecoder?(0,e.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});o.decode();let c=null;function u(){return null!==c&&c.buffer===r.f.buffer||(c=new Uint8Array(r.f.buffer)),c}function i(e,t){return o.decode(u().subarray(e,e+t))}const f=new Array(32).fill(void 0);f.push(void 0,null,!0,!1);let l=f.length;function s(e){return f[e]}function a(e){const t=s(e);return function(e){e<36||(f[e]=l,l=e)}(e),t}let d=0;let h=new("undefined"==typeof TextEncoder?(0,e.require)("util").TextEncoder:TextEncoder)("utf-8");const p="function"==typeof h.encodeInto?function(e,t){return h.encodeInto(e,t)}:function(e,t){const n=h.encode(e);return t.set(n),{read:e.length,written:n.length}};function b(e,t,n){if(void 0===n){const n=h.encode(e),r=t(n.length);return u().subarray(r,r+n.length).set(n),d=n.length,r}let r=e.length,o=t(r);const c=u();let i=0;for(;i<r;i++){const t=e.charCodeAt(i);if(t>127)break;c[o+i]=t}if(i!==r){0!==i&&(e=e.slice(i)),o=n(o,r,r=i+3*e.length);const t=u().subarray(o+i,o+r);i+=p(e,t).written}return d=i,o}let w=null;function y(){return null!==w&&w.buffer===r.f.buffer||(w=new Int32Array(r.f.buffer)),w}class g{static __wrap(e){const t=Object.create(g.prototype);return t.ptr=e,t}__destroy_into_raw(){const e=this.ptr;return this.ptr=0,e}free(){const e=this.__destroy_into_raw();r.a(e)}constructor(e,t,n,o){var c=b(e,r.d,r.e),u=d,i=b(t,r.d,r.e),f=d,l=b(n,r.d,r.e),s=d,a=b(o,r.d,r.e),h=d,p=r.g(c,u,i,f,l,s,a,h);return g.__wrap(p)}search(e){try{const u=r.b(-16);var t=b(e,r.d,r.e),n=d;r.h(u,this.ptr,t,n);var o=y()[u/4+0],c=y()[u/4+1];return i(o,c)}finally{r.b(16),r.c(o,c)}}}function v(e,t){return function(e){l===f.length&&f.push(f.length+1);const t=l;return l=f[t],f[t]=e,t}(i(e,t))}function x(e){console.log(s(e))}function _(e){a(e)}function m(e,t){throw new Error(i(e,t))}}).call(this,n(3)(e))},function(e,t){e.exports=function(e){if(!e.webpackPolyfill){var t=Object.create(e);t.children||(t.children=[]),Object.defineProperty(t,"loaded",{enumerable:!0,get:function(){return t.l}}),Object.defineProperty(t,"id",{enumerable:!0,get:function(){return t.i}}),Object.defineProperty(t,"exports",{enumerable:!0}),t.webpackPolyfill=1}return t}},function(e,t,n){"use strict";var r=n.w[e.i];e.exports=r;n(2);r.i()}]]);
var ffi = require('ffi');

var lib = ffi.Library('../target/release/libembed', {
  'run': ['void', []]
});

lib.run();

console.log("done!");

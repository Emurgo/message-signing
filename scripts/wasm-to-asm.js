const fs = require('fs')

const paths = [
  './rust/pkg/cardano_message_signing_bg.js',
  './rust/pkg/cardano_message_signing.js'
]

paths.forEach((path) => {
  fs.readFile(path, 'utf8', (err,data) => {
    if (err) {
      return console.log(err);
    }

    const  result = data.replace(/_bg.wasm/g, '.asm.js');

    fs.writeFile(path, result, 'utf8', (err) => {
      if (err) return console.log(err);
    });
  });
})

fs.unlinkSync('./rust/pkg/cardano_message_signing_bg.wasm')

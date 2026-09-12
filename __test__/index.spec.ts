import { existsSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import test from 'ava'

import { Database } from '../index'

const require = createRequire(import.meta.url)
const rootDir = join(dirname(fileURLToPath(import.meta.url)), '..')
const hasLocalWasm =
  existsSync(join(rootDir, 'package-template.wasm32-wasi.wasm')) ||
  existsSync(join(rootDir, 'package-template.wasm32-wasi.debug.wasm'))

const DATABASE_NAME = "db.bin";

test.afterEach('Cleanup', async () => {
  // Delete `DATABASE_NAME`
});

test('Open database', async (t) => {
  const db = await Database.open("db.bin");


});

test('Create collection', async (t) => {

});

test('List collections', async (t) => {
  
});

test('Rename collection', async (t) => {
  
});

test('Delete collection', async (t) => {
  
});

test('Delete collection', async (t) => {
  
});

test('Insert document in the collection', async (t) => {
  
});

test('Find one document in the collection', async (t) => {
  
});

test('Find all documents in the collection', async (t) => {
  
});

test('Update document in the collection', async (t) => {
  
});

test('Delete document in the collection', async (t) => {
  
});


// test('sync function from native code', (t) => {
//   const fixture = 42
//   t.is(plus100(fixture), fixture + 100)
// })

// test('@napi-rs/wasm-runtime is resolvable for the generated WASI loader', (t) => {
//   t.notThrows(() => require.resolve('@napi-rs/wasm-runtime'))
// })

// const testWasiBinding = hasLocalWasm ? test : test.skip

// testWasiBinding('generated WASI binding loads from a local wasm artifact', (t) => {
//   const binding = require(join(rootDir, 'package-template.wasi.cjs')) as {
//     plus100: (n: number) => number
//   }
//   t.is(binding.plus100(42), 142)
// })

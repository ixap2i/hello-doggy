import React from "react";
import * as wasm from "record-memo";
// import { useEffect, useState } from 'react';

var testarr = new Array();
async function test(): Promise<String> {
  var test2 = await wasm.run();
  console.log(test2.message);
  const str = test2.message;
  testarr.push(str);
  return str;
}
const t = test().then(() => console.log(testarr));
const t2 = test().then(() => { return testarr[0] });

const App = () => {
  const test = wasm.greet();
  console.log(t2)
  return (
    <div>
      <div>wasm-react-test</div>
    </div>
  );
};

export default App;
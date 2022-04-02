import React from "react";
import * as wasm from "record-memo";

const App = () => {
  const test = wasm.greet();
  return (
    <div>
        <div>wasm-react-test</div>
        <div>${test}</div>
    </div>
  );
};

export default App;
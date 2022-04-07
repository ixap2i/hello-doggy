import React from "react";
import * as wasm from "record-memo";

const App = () => {
  const test = wasm.greet();
  const test2 = wasm.db_conn();
  return (
    <div>
        <div>wasm-react-test</div>
        <div>${test2}</div>
    </div>
  );
};

export default App;
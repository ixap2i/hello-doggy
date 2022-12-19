import React from "react";
import * as wasm from "hello-doggy";

const App = () => {
  type Dog = {
    status: string;
    message: string;
  };
  var arr: Array<Dog> = [];
  function req_dog(): Promise<Dog> {
    return wasm.run().then((data: Dog) => {
      arr.push(data);
      return data;
    })
  }
  async function get_dog_data() {
    const result = await req_dog();
    return result;
  }

  get_dog_data().then((d) => {
    localStorage.setItem('imageUrl', d.message);
  })

  const dogImgUrl = localStorage.imageUrl;

  function DogImg() {
    return <img src={dogImgUrl}></img>
  }
  return (
    <div>
      <div>wasm-react-test</div>
      <DogImg></DogImg>
    </div>
  );
};

export default App;
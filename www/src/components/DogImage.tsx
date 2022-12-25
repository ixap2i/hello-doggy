import React from "react";
import * as wasm from "hello-doggy";
import Dog from "./types/DogImage";

class DogImage extends React.Component<{}, {srcUrl: string}> {
  constructor(props: any) {
    super(props);
    this.state = {
      srcUrl: ''
    };
  }

  requestDogImage() {
    return wasm.run().then((data: Dog) => {
      this.setState({
        srcUrl: data.message
      });
    })
  }

  componentDidMount() {
    this.requestDogImage();
  }

  render() {
    return (
      <div>
        <img src={this.state.srcUrl} alt="a dog image" />
      </div>
    );
  }
}

export default DogImage;
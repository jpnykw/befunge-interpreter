import * as React from "react";
import * as ReactDOM from "react-dom";
import styled from 'styled-components';

import { Button } from '@material-ui/core';

import Input from "../components/Input";

const App: React.FC<{}> = () => {
  return (
    <div>
      <Input
        label="Source Code"
        value='64+"!dlroW ,olleH">:#,_@'
        id="Code"
      />

      <Input
        label="Input"
        value=""
        id="Input"
      />

      <Button color="primary" id="Submit">Run</Button>
    </div>
  );
};

ReactDOM.render(<App />, document.getElementById("app"));

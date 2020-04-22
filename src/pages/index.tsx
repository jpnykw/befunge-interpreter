import * as React from "react";
import * as ReactDOM from "react-dom";
import styled from 'styled-components';

import {
  Button,
  Checkbox,
  Container,
  Grid
} from '@material-ui/core';

import Input from "../components/Input";
import Output from "../components/Output";

const App: React.FC<{}> = () => {
  const [state, setState] = React.useState(false);
  const toggle = () => setState(!state);

  return (
    <Container>
      <Grid container spacing={3} alignItems="center" justify="center">
        <Grid item xs={12}>
          <Input
            label="Source Code"
            value='64+"!dlroW ,olleH">:#,_@'
            id="Code"
          />
        </Grid>

        <Grid item xs={12}>
          <Input
            label="Input"
            value=""
            id="Input"
          />
        </Grid>

        <Checkbox checked={state} onChange={toggle} />

        <Button color="primary" id="Submit">Run</Button>

        <Grid item xs={12}>
          <Output
            label="Output"
            id="Output"
          />
        </Grid>
      </Grid>
    </Container>
  );
};

ReactDOM.render(<App />, document.getElementById("app"));

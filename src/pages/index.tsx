import * as React from "react";
import * as ReactDOM from "react-dom";
import styled from 'styled-components';

import {
  Box,
  Button,
  Checkbox,
  Container,
  Collapse,
  Grid
} from '@material-ui/core';

import {
  ExpandLess,
  ExpandMore,
} from '@material-ui/icons';

import Input from "../components/Input";
import Output from "../components/Output";
import Option from "../components/Option";

const App: React.FC<{}> = () => {
  const [open, setOpen] = React.useState(false);
  const toggle = () => setOpen(!open);

  return (
    <Container>
      <Grid container spacing={3} alignItems="center" justify="center">
        <Grid item xs={12}>
          <Input
            label="Source Code"
            value='64+"!dlroW ,olleH">:#,_@'
            id="code"
          />
        </Grid>

        <Grid item xs={12}>
          <Input
            label="Input"
            value=""
            id="input"
          />
        </Grid>

        <StyledButton color="primary" id="run">
          Run
        </StyledButton>

        {/*
        <Grid item xs={12} onClick={toggle}>
          <Box textAlign="center">
            {open ? <ExpandLess /> : <ExpandMore />}
          </Box>
        </Grid>

        <Grid item xs={12}>
          <Box textAlign="center">
            <Collapse in={open}>
              <Option />
            </Collapse>
          </Box>
        </Grid>
        */}

        <Grid item xs={12}>
          <Output
            label="Output"
            id="output"
          />
        </Grid>
      </Grid>
    </Container>
  );
};

const StyledButton = styled(Button)`
  font-size: 20px;
  margin: 0 10px;
`;

ReactDOM.render(<App />, document.getElementById("app"));

import * as React from "react";

import { TextField } from '@material-ui/core';

interface InputProps {
  label: String;
  value: String;
};

const Input: React.FC<InputProps> = (props) => {
  return (
    <TextField
      id="outlined-multiline-static"
      label={props.label}
      multiline
      rows={5}
      defaultValue={props.value}
      variant="outlined"
    />
  );
}

export default Input;

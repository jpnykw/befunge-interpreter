import * as React from "react";

import { TextField } from '@material-ui/core';

interface InputProps {
  label: String;
  id: string;
};

const Input: React.FC<InputProps> = (props) => {
  return (
    <TextField
      id={props.id}
      label={props.label}
      multiline
      rows={5}
      variant="outlined"
      InputProps={{
        readOnly: true,
      }}
      InputLabelProps={{
        shrink: true,
      }}
    />
  );
}

export default Input;

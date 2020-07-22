import * as React from "react";
import styled from 'styled-components';

import { TextField } from '@material-ui/core';

interface InputProps {
  label: String;
  value: String;
  id: string;
};

const Input: React.FC<InputProps> = (props) => {
  return (
    <TextField
      id={props.id}
      label={props.label}
      multiline
      rows={5}
      defaultValue={props.value}
      variant="outlined"
    />
  );
}

export default Input;

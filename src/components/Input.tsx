import * as React from "react";
import styled from 'styled-components';

import { TextField } from '@material-ui/core';

interface InputProps {
  label: String;
  value: String;
  id: string;
};

const StyledTextField = styled(TextField)`
  width: 100%;
`;

const Input: React.FC<InputProps> = (props) => {
  return (
    <StyledTextField
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

import * as React from 'react';
import styled from 'styled-components';

import { TextField } from '@material-ui/core';

interface OutputProps {
  label: String;
  id: string;
};

const StyledTextField = styled(TextField)`
  width: 100%;
`;

const Input: React.FC<OutputProps> = (props) => {
  return (
    <StyledTextField
      id={props.id}
      label={props.label}
      multiline
      rows={5}
      variant='outlined'
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


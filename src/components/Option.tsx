import * as React from "react";
import styled from 'styled-components';

import {
  Box,
  Checkbox,
  List,
  ListItem,
  ListItemIcon,
  ListItemText
} from '@material-ui/core';

const Option: React.FC<{}> = () => {
  const [use, setUse] = React.useState(false);
  const toggle = () => setUse(!use);

  return (
      <List>
        <ListItem>
          <ListItemIcon
            onClick={toggle}
          >
            <Checkbox
              disableRipple
              checked={use}
            />
          </ListItemIcon>
          <ListItemText
            primary='Enable step-run mode'
          />
        </ListItem>
      </List>
  );
}

export default Option;

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
        <StyledListItem>
          <ListItemIcon
            onClick={toggle}
          >
            <Checkbox
              disableRipple
              checked={use}
              value={use}
              id="mode"
            />
          </ListItemIcon>
          <ListItemText
            primary='Enable step-run mode'
          />
        </StyledListItem>
      </List>
  );
}

const StyledListItem: typeof ListItem = styled(ListItem)`
  width: fit-content;
  margin: 0 auto;
`;

export default Option;

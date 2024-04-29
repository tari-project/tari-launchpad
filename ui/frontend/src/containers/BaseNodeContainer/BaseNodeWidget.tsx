import { Button, Typography, Box, MenuItem } from '@mui/material';
import { StyledPaper } from '../../components/StyledComponents';
import { useTheme } from '@mui/material/styles';
import t from '../../locales';
import typography from '../../styles/styles/typography';
import { styled } from '@mui/material/styles';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import { useState } from 'react';
import { networkOptions } from './constants';

export const MiningBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(2),
}));

function BaseNodeWidget() {
  const theme = useTheme();
  const [network, setNetwork] = useState(networkOptions[0].value);

  const handleChange = (event: SelectChangeEvent) => {
    setNetwork(event.target.value as string);
    console.log(event.target.value);
  };

  return (
    <StyledPaper>
      <MiningBox>
        <Box
          style={{
            display: 'flex',
            justifyContent: 'space-between',
            gap: theme.spacing(3),
          }}
        >
          <Box
            style={{
              display: 'flex',
              flexDirection: 'column',
              gap: theme.spacing(1),
            }}
          >
            <Typography variant="h3" sx={typography.header}>
              {t.baseNode.title}
            </Typography>
            <Typography variant="body1" sx={typography.defaultMedium}>
              {t.baseNode.tari_network_label}
            </Typography>
          </Box>
        </Box>
        <Select
          labelId="network-select-label"
          id="network-select"
          value={network}
          label="Network"
          onChange={handleChange}
          placeholder="Network"
        >
          {networkOptions.map(({ label, value, key }) => (
            <MenuItem value={value} key={key}>
              {label}
            </MenuItem>
          ))}
        </Select>
        <Button
          variant="contained"
          onClick={() => console.log('Start Base Node')}
        >
          {t.baseNode.start}
        </Button>
        <Box>
          <Typography variant="body1" sx={typography.smallHeavy}>
            {t.baseNode.blockInfo.height}
            <span style={typography.smallMedium}>blockNo</span>
          </Typography>
          <Typography variant="body1" sx={typography.smallHeavy}>
            {t.baseNode.blockInfo.time}
            <span style={typography.smallMedium}>time</span>
          </Typography>
          <Typography variant="body1" sx={typography.smallHeavy}>
            {t.baseNode.blockInfo.status}
            <span style={typography.smallMedium}>status</span>
          </Typography>
        </Box>
      </MiningBox>
    </StyledPaper>
  );
}

export default BaseNodeWidget;

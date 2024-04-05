import { Typography, TextField } from '@mui/material';
import SubHeading from '../UI/SubHeading';
import typography from '../../styles/styles/typography';
import { SettingsBox } from '../UI/StyledComponents';

function MiningSettings() {
  return (
    <>
      <Typography variant="h3" style={typography.subheader}>
        Mining Settings
      </Typography>
      <SettingsBox>
        <SubHeading text="Expert" />
        <TextField placeholder="Monero Mining Address" />
        <TextField placeholder="SHA3 Threads" />
        <TextField placeholder="RandomX Threads" />
        <TextField placeholder="Monero Node URL" />
        <TextField placeholder="Wallet Payment Address" />
      </SettingsBox>
    </>
  );
}

export default MiningSettings;

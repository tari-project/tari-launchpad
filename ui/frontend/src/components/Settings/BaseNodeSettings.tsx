import { Typography, TextField } from '@mui/material';
import SubHeading from '../UI/SubHeading';
import typography from '../../styles/styles/typography';
import { SettingsBox } from '../UI/StyledComponents';

function BaseNodeSettings() {
  return (
    <>
      <Typography variant="h3" style={typography.subheader}>
        Base Node Settings
      </Typography>
      <SettingsBox>
        <SubHeading text="Expert" />
        <TextField placeholder="Root Folder" />
      </SettingsBox>
      <SettingsBox>
        <SubHeading text="QR Code" />
      </SettingsBox>
    </>
  );
}

export default BaseNodeSettings;

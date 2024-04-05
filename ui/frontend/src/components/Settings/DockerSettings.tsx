import { Typography, TextField } from '@mui/material';
import SubHeading from '../UI/SubHeading';
import typography from '../../styles/styles/typography';
import { SettingsBox } from '../UI/StyledComponents';

function DockerSettings() {
  return (
    <>
      <Typography variant="h3" style={typography.subheader}>
        Docker Settings
      </Typography>
      <SettingsBox>
        <SubHeading text="Expert" />
        <TextField placeholder="Docker Tag" />
        <TextField placeholder="Docker Registry" />
      </SettingsBox>
      <SettingsBox>
        <SubHeading text="Image Statuses" />
      </SettingsBox>
    </>
  );
}

export default DockerSettings;

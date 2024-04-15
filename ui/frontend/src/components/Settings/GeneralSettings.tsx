import { Typography, Switch } from '@mui/material';
import SubHeading from '../UI/SubHeading';
import typography from '../../styles/styles/typography';
import { SettingsBox } from '../UI/StyledComponents';
import { FormGroup, FormControlLabel } from '@mui/material';
import useGeneralSettingsStore from '../../store/generalSettingsStore';

function GeneralSettings() {
  const { runOnStartup, setRunOnStartup, mineOnStartup, setMineOnStartup } =
    useGeneralSettingsStore();
  return (
    <>
      <Typography variant="h3" style={typography.subheader}>
        General Settings
      </Typography>
      <SettingsBox>
        <SubHeading text="Advanced" />
        <FormGroup>
          <FormControlLabel
            control={
              <Switch
                checked={runOnStartup}
                onChange={() => setRunOnStartup(!runOnStartup)}
                inputProps={{ 'aria-label': 'run on startup' }}
                style={{
                  marginRight: '4px',
                }}
              />
            }
            label="Run on startup"
            labelPlacement="end"
          />
        </FormGroup>
        <FormGroup>
          <FormControlLabel
            control={
              <Switch
                checked={mineOnStartup}
                onChange={() => setMineOnStartup(!mineOnStartup)}
                inputProps={{ 'aria-label': 'start mining on startup' }}
                style={{
                  marginRight: '4px',
                }}
              />
            }
            label="Start mining on startup"
            labelPlacement="end"
          />
        </FormGroup>
      </SettingsBox>
    </>
  );
}

export default GeneralSettings;

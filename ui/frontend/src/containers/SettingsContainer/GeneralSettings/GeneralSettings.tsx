import { Typography, Switch } from '@mui/material';
import SubHeading from '../../../components/SubHeading';
import typography from '../../../styles/styles/typography';
import { SettingsBox } from '../../../components/StyledComponents';
import { FormGroup, FormControlLabel } from '@mui/material';
import useAppStateStore from '../../../store/appStateStore';

function GeneralSettings() {
  const { runOnStartup, setRunOnStartup, mineOnStartup, setMineOnStartup } =
    useAppStateStore();
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

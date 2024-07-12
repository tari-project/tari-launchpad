import Stack from '@mui/material/Stack';
import Button from '@mui/material/Button';
import Switch from '@mui/material/Switch';
import FormGroup from '@mui/material/FormGroup';
import FormControlLabel from '@mui/material/FormControlLabel';
import useAppStateStore from '../../store/appStateStore';
import SvgSetting from '../../styles/Icons/Setting2';
import { useShallow } from 'zustand/react/shallow';

function HeaderButtons({
  open,
  handleDrawerClose,
  handleDrawerOpen,
}: {
  open: boolean;
  handleDrawerClose: () => void;
  handleDrawerOpen: () => void;
}) {
  const { setOpenSettings } = useAppStateStore(
    useShallow((state) => ({
      setOpenSettings: state.setOpenSettings,
    }))
  );
  function handleOpenSettings() {
    setOpenSettings(true);
  }

  const SettingsButton = () => {
    return (
      <Button
        onClick={handleOpenSettings}
        size="medium"
        startIcon={<SvgSetting />}
        style={{
          color: open ? '#fff' : 'inherit',
        }}
      >
        Settings
      </Button>
    );
  };

  const ExpertViewToggle = () => {
    return (
      <FormGroup>
        <FormControlLabel
          control={
            <Switch
              checked={open}
              onChange={open ? handleDrawerClose : handleDrawerOpen}
              inputProps={{ 'aria-label': 'toggle expert view' }}
              style={{
                marginRight: '4px',
              }}
            />
          }
          label={
            <span
              style={{
                color: open ? '#fff' : 'inherit',
              }}
            >
              Expert View
            </span>
          }
          labelPlacement="end"
        />
      </FormGroup>
    );
  };

  return (
    <Stack direction="row" spacing={1} alignItems="center">
      <SettingsButton />
      <ExpertViewToggle />
    </Stack>
  );
}

export default HeaderButtons;

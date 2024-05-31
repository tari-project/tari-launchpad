import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
} from '@mui/material';
import useAppStateStore from '../../store/appStateStore';
import { exit } from '@tauri-apps/api/process';
import typography from '../../styles/styles/typography';

function DockerDialog() {
  const { openDockerWarning, setOpenDockerWarning } = useAppStateStore(
    (state) => ({
      openDockerWarning: state.openDockerWarning,
      setOpenDockerWarning: state.setOpenDockerWarning,
    })
  );

  async function openDockerInstall(evt: any) {
    evt.preventDefault();
    open('https://docs.docker.com/engine/install/');
  }

  async function handleDockerClose() {
    setOpenDockerWarning(false);
    await exit(1);
  }

  return (
    <Dialog
      open={openDockerWarning}
      onClose={handleDockerClose}
      aria-labelledby="alert-dialog-title"
      aria-describedby="alert-dialog-description"
    >
      <DialogTitle id="alert-dialog-title">Docker is not running</DialogTitle>
      <DialogContent>
        <DialogContentText
          id="alert-dialog-description"
          sx={typography.smallMedium}
          pr={3}
          pl={3}
        >
          Tari Launchpad requires Docker to be running. Please start Docker and
          try again. If you don't have Docker installed, you can download it
          from{' '}
          <a onClick={(evt) => openDockerInstall(evt)} href="#">
            here
          </a>
          .
        </DialogContentText>
      </DialogContent>
      <DialogActions>
        <Button variant="contained" onClick={handleDockerClose}>
          Exit
        </Button>
      </DialogActions>
    </Dialog>
  );
}

export default DockerDialog;

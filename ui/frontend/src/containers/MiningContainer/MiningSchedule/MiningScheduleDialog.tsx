import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
} from '@mui/material';
import useAppStateStore from '../../../store/appStateStore';
import typography from '../../../styles/styles/typography';
import t from '../../../locales';

function MiningScheduleDialog() {
  const { openSchedule, setOpenSchedule } = useAppStateStore();

  const handleScheduleClose = () => {
    setOpenSchedule(false);
  };

  return (
    <Dialog
      open={openSchedule}
      onClose={handleScheduleClose}
      aria-labelledby="alert-dialog-title"
      aria-describedby="alert-dialog-description"
    >
      <DialogTitle id="alert-dialog-title">
        {t.mining.scheduling.title}
      </DialogTitle>
      <DialogContent>
        <DialogContentText
          id="alert-dialog-description"
          sx={typography.smallMedium}
          pr={3}
          pl={3}
        >
          {t.mining.scheduling.launchpadOpen}.
        </DialogContentText>
      </DialogContent>
      <DialogActions>
        <Button variant="contained" onClick={handleScheduleClose}>
          Close
        </Button>
      </DialogActions>
    </Dialog>
  );
}

export default MiningScheduleDialog;

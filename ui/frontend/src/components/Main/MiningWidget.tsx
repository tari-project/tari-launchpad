import { Button, Typography, Box } from '@mui/material';
import useMiningStore from '../../store/miningStore';
import { StyledPaper } from '../UI/StyledComponents';
import { useTheme } from '@mui/material/styles';
import gradients from '../../styles/styles/gradients';

function MiningWidget() {
  const { isMining, setIsMining } = useMiningStore();
  const theme = useTheme();
  return (
    <StyledPaper
      style={{
        background: isMining ? gradients.tari : theme.palette.background.paper,
      }}
    >
      <Box
        style={{
          display: 'flex',
          flexDirection: 'column',
          gap: theme.spacing(2),
        }}
      >
        <Typography variant="h3">Mining </Typography>
        <Typography variant="body1">Mining text goes here </Typography>
        <Button
          variant="contained"
          color="primary"
          onClick={() => setIsMining(!isMining)}
        >
          {isMining ? 'Stop Mining' : 'Start Mining'}
        </Button>
      </Box>
    </StyledPaper>
  );
}

export default MiningWidget;

import { Button, Typography, Box } from '@mui/material';
import useMergedMiningStore from '../../store/mergedMiningStore';
import { StyledPaper } from '../UI/StyledComponents';
import { useTheme } from '@mui/material/styles';
import gradients from '../../styles/styles/gradients';

function MergedMiningWidget() {
  const { isMergedMining, setIsMergedMining } = useMergedMiningStore();
  const theme = useTheme();
  return (
    <StyledPaper
      style={{
        background: isMergedMining
          ? gradients.merged
          : theme.palette.background.paper,
      }}
    >
      <Box
        style={{
          display: 'flex',
          flexDirection: 'column',
          gap: theme.spacing(2),
        }}
      >
        <Typography variant="h3">Merged Mining</Typography>
        <Typography variant="body1">Merged mining text goes here </Typography>
        <Button
          variant="contained"
          color="primary"
          onClick={() => setIsMergedMining(!isMergedMining)}
        >
          {isMergedMining ? 'Stop Mining' : 'Start Mining'}
        </Button>
      </Box>
    </StyledPaper>
  );
}

export default MergedMiningWidget;

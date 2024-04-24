import { Typography, Box, Chip, Button } from '@mui/material';
import useMergedMiningStore from '../../store/mergedMiningStore';
import { useTheme } from '@mui/material/styles';
import gradients from '../../styles/styles/gradients';
import t from '../../locales';
import typography from '../../styles/styles/typography';
import SvgTariSignet from '../../styles/Icons/TariSignet';
import SvgMoneroSignet from '../../styles/Icons/MoneroSignet';
import SvgQuestion from '../../styles/Icons/Question';
import { StyledIconButton } from '../UI/StyledComponents';
import { useSnackbar } from 'notistack';
import GradientBox from '../UI/GradientBox';

function MergedMiningWidget() {
  const { isMergedMining, setIsMergedMining } = useMergedMiningStore();
  const theme = useTheme();
  const { enqueueSnackbar } = useSnackbar();

  const mergedMiningHelp = () => {
    enqueueSnackbar(`${t.mergedMiningHelp.message1}`, {
      key: 'mergedMiningHelp.message1',
      persist: true,
    });
  };

  return (
    <GradientBox isActive={isMergedMining} gradient={gradients.merged}>
      <Box
        style={{
          display: 'flex',
          justifyContent: 'space-between',
          gap: theme.spacing(3),
        }}
      >
        <Box
          style={{
            display: 'flex',
            flexDirection: 'column',
            gap: theme.spacing(1),
          }}
        >
          <Box>
            <Chip
              label={
                <span>
                  <strong>{t.common.phrases.readyToSet}</strong>
                </span>
              }
              color="info"
            />
          </Box>
          <Box
            style={{
              display: 'flex',
              gap: theme.spacing(1),
              alignItems: 'center',
            }}
          >
            <Typography variant="h3" sx={typography.header}>
              {t.common.miningType.merged}
            </Typography>
            <StyledIconButton onClick={() => mergedMiningHelp()}>
              <SvgQuestion color={theme.palette.primary.main} />
            </StyledIconButton>
          </Box>
          <Typography variant="body2" sx={typography.defaultMedium}>
            {t.mining.setup.description}{' '}
            <span style={typography.defaultHeavy}>
              {t.mining.setup.descriptionBold}
            </span>
          </Typography>
        </Box>
        <Box
          style={{
            display: 'flex',
            flexDirection: 'column',
            gap: theme.spacing(2),
          }}
        >
          <SvgMoneroSignet
            color={theme.palette.divider}
            width="80px"
            height="80px"
          />
          <SvgTariSignet
            color={theme.palette.divider}
            width="80px"
            height="80px"
          />
        </Box>
      </Box>
      <Button variant="contained" disabled>
        {t.mining.actions.setupAndStartMining}
      </Button>
      <Button
        variant="contained"
        onClick={() => setIsMergedMining(!isMergedMining)}
      >
        {isMergedMining ? 'Stop Mining' : 'Start Mining'}
      </Button>
    </GradientBox>
  );
}

export default MergedMiningWidget;

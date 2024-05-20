import { Typography, Box } from '@mui/material';
import MiningWidget from './MiningBox/MiningWidget';
import MergeMiningWidget from './MiningBoxMerged/MergeMiningWidget';
import {
  StyledIconButton,
  TabInnerBox,
} from '../../components/StyledComponents';
import SvgQuestion from '../../styles/Icons/Question';
import SvgStar from '../../styles/Icons/Star';
import typography from '../../styles/styles/typography';
import { useTheme } from '@mui/material/styles';
import { useSnackbar } from 'notistack';
import t from '../../locales';
import MiningFooter from './components/MiningFooter';

function MiningTab() {
  const theme = useTheme();
  const { enqueueSnackbar } = useSnackbar();
  return (
    <TabInnerBox>
      <Box
        style={{
          width: '100%',
          display: 'flex',
          flexDirection: 'column',
          gap: theme.spacing(3),
        }}
      >
        <Box
          style={{
            display: 'flex',
            flexDirection: 'row',
            alignItems: 'center',
          }}
        >
          <SvgStar
            style={{
              height: '24px',
              width: '24px',
              marginRight: theme.spacing(1),
            }}
          />
          <Typography
            style={{ marginTop: theme.spacing(0.5) }}
            variant="h3"
            sx={typography.defaultHeavy}
          >
            {t.mining.headerTips.oneStepAway}{' '}
            <span style={typography.defaultUnder}>
              {t.mining.headerTips.wantToKnowMore}
            </span>
          </Typography>
          <StyledIconButton
            onClick={() =>
              enqueueSnackbar(`${t.cryptoMiningHelp.message1}`, {
                key: 'cryptoMiningHelp.message1',
                persist: true,
              })
            }
          >
            <SvgQuestion />
          </StyledIconButton>
        </Box>
        <Box
          style={{
            display: 'grid',
            gap: theme.spacing(3),
            gridTemplateColumns: 'repeat(auto-fill, minmax(400px, 1fr))',
          }}
        >
          <MiningWidget />
          <MergeMiningWidget />
        </Box>
        <MiningFooter />
      </Box>
    </TabInnerBox>
  );
}

export default MiningTab;

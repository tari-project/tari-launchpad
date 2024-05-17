import { Typography, Box } from '@mui/material';
import BaseNodeWidget from './BaseNodeWidget';
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
import BaseNodeFooter from './BaseNodeFooter';

function BaseNodeTab() {
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
          alignItems: 'center',
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
            {t.baseNode.helpMessages.howItWorks.tip.text}{' '}
            <span style={typography.defaultUnder}>
              {t.baseNode.helpMessages.howItWorks.tip.cta}
            </span>
          </Typography>
          <StyledIconButton
            onClick={() =>
              enqueueSnackbar(
                `${t.baseNode.helpMessages.howItWorks.allowsYou}: 
                ${t.baseNode.helpMessages.howItWorks.affordances[0]},
                ${t.baseNode.helpMessages.howItWorks.affordances[1]},
                ${t.baseNode.helpMessages.howItWorks.affordances[2]}.
                ${t.baseNode.helpMessages.howItWorks.thankYou}
                ${t.baseNode.helpMessages.howItWorks.yourContribution}
                `,
                {
                  key: 'baseNode.helpMessages.howItWorks.allowsYou',
                  persist: true,
                }
              )
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
          <BaseNodeWidget />
          <BaseNodeFooter />
        </Box>
      </Box>
    </TabInnerBox>
  );
}

export default BaseNodeTab;

import { MessageBox } from '../../TBotContainer/styles';
import { useSnackbar } from 'notistack';
import { Button, Typography } from '@mui/material';
import { StyledIconButton } from '../../../components/StyledComponents';
import SvgQuestion from '../../../styles/Icons/Question';
import typography from '../../../styles/styles/typography';
import { useTheme } from '@mui/material/styles';
import t from '../../../locales';

export const CryptoMiningHelp = () => {
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const key = 'crypto-mining-help';
  const handleClick = () => {
    enqueueSnackbar(
      <MessageBox>
        <Typography sx={typography.defaultMedium}>
          {t.cryptoMiningHelp.message1}
        </Typography>
        <Button variant="contained" onClick={() => closeSnackbar(key)}>
          {t.common.phrases.gotIt}
        </Button>
      </MessageBox>,
      {
        key: key,
        persist: false,
      }
    );
  };

  return (
    <StyledIconButton onClick={handleClick}>
      <SvgQuestion />
    </StyledIconButton>
  );
};

export const MergedMiningHelp = ({
  miningStatus,
}: {
  miningStatus: string;
}) => {
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const theme = useTheme();

  const key = 'merged-mining-help';
  const handleClick = () => {
    enqueueSnackbar(
      <MessageBox>
        <Typography sx={typography.defaultHeavy}>
          {t.mergedMiningHelp.message1}
        </Typography>
        <Typography sx={typography.defaultMedium}>
          {t.mergedMiningHelp.message2}
        </Typography>
        <Button variant="contained" onClick={() => closeSnackbar(key)}>
          {t.common.phrases.gotIt}
        </Button>
      </MessageBox>,
      {
        key: key,
        persist: false,
      }
    );
  };

  return (
    <StyledIconButton onClick={handleClick}>
      <SvgQuestion
        color={
          miningStatus === 'inactive' ? theme.palette.primary.main : '#fff'
        }
      />
    </StyledIconButton>
  );
};

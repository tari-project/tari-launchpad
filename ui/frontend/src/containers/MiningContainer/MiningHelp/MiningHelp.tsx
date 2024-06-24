import { useState, useEffect } from 'react';
import { HelpTextBox } from '../styles';
import SvgStar from '../../../styles/Icons/Star';
import { useTheme } from '@mui/material/styles';
import { Typography } from '@mui/material';
import typography from '../../../styles/styles/typography';
import { CryptoMiningHelp } from './Messages';
import t from '../../../locales';
import useAppStateStore from '../../../store/appStateStore';
import { ShaMiningStatus, MergeMiningStatus } from '../../../store/types';

function MiningHelp() {
  const { containers } = useAppStateStore((state) => ({
    containers: state.containers,
  }));
  const [message, setMessage] = useState<string>('');
  const theme = useTheme();

  useEffect(() => {
    switch (true) {
      case containers?.sha3Miner?.status === ShaMiningStatus.ACTIVE &&
        containers?.mmProxy?.status === MergeMiningStatus.ACTIVE:
        setMessage(t.mining.headerTips.tariMoneroOn);
        break;
      case containers?.sha3Miner?.status === ShaMiningStatus.ACTIVE:
        setMessage(t.mining.headerTips.tariOn);
        break;
      case containers?.mmProxy?.status === MergeMiningStatus.ACTIVE:
        setMessage(t.mining.headerTips.moneroOn);
        break;
      default:
        setMessage(t.mining.headerTips.oneStepAway);
        break;
    }
  }, [containers?.sha3Miner?.status, containers?.mmProxy?.status]);

  return (
    <HelpTextBox>
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
        {message}{' '}
        <span style={typography.defaultUnder}>
          {t.mining.headerTips.wantToKnowMore}
        </span>
      </Typography>
      <CryptoMiningHelp />
    </HelpTextBox>
  );
}

export default MiningHelp;

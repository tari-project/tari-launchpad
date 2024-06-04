import { useEffect, useState } from 'react';
import { Typography } from '@mui/material';
import SvgStar from '../../../styles/Icons/Star';
import typography from '../../../styles/styles/typography';
import { useTheme } from '@mui/material/styles';
import t from '../../../locales';
import { BaseNodeHelpMessage } from './Messages';
import { HelpTextBox } from '../styles';
import useAppStateStore from '../../../store/appStateStore';
import { BaseNodeStatus } from '../../../store/types';

function BaseNodeHelp() {
  const theme = useTheme();
  const [message, setMessage] = useState<string>('');
  const { containers } = useAppStateStore((state) => ({
    containers: state.containers,
  }));

  useEffect(() => {
    switch (true) {
      case containers.baseNode?.status === BaseNodeStatus.ACTIVE:
        setMessage(t.baseNode.helpMessages.alreadyRunning);
        break;
      default:
        setMessage(t.baseNode.helpMessages.howItWorks.tip.text);
        break;
    }
  }, [containers?.baseNode?.status]);

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
          {t.baseNode.helpMessages.howItWorks.tip.cta}
        </span>
      </Typography>
      <BaseNodeHelpMessage />
    </HelpTextBox>
  );
}

export default BaseNodeHelp;

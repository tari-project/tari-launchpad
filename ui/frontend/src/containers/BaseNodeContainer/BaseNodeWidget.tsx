import { Button, Typography, Box } from '@mui/material';
import { LabelBoxVertical } from '../../components/StyledComponents';
import t from '../../locales';
import typography from '../../styles/styles/typography';
import useAppStateStore from '../../store/appStateStore';
import { BaseNodeStatus } from '../../store/types';
import {
  StatusChip,
  TransparentButton,
} from '../../components/StyledComponents';
import { BaseNodeBox, ContentBox, DefaultBox } from './styles';
import { useEffect, useState } from 'react';

type Status = 'inactive' | 'pending' | 'active';

function BaseNodeWidget() {
  const { appState, containers, startBaseNode, stopBaseNode, network } =
    useAppStateStore((state) => ({
      appState: state.appState,
      containers: state.containers,
      startBaseNode: state.startBaseNode,
      stopBaseNode: state.stopBaseNode,
      network: state.network,
    }));
  const [baseNodeStatus, setBaseNodeStatus] = useState<Status>('inactive');

  function start() {
    startBaseNode();
  }

  function stop() {
    stopBaseNode();
  }

  useEffect(() => {
    if (
      containers.baseNode?.status === BaseNodeStatus.WAITING ||
      containers.baseNode?.status === BaseNodeStatus.SHUTTINGDOWN ||
      containers.baseNode?.status === BaseNodeStatus.STARTING ||
      containers.baseNode?.status === BaseNodeStatus.PENDING ||
      containers.baseNode?.status === BaseNodeStatus.SYNCING ||
      containers.baseNode?.status === BaseNodeStatus.HEADERSYNC
    ) {
      setBaseNodeStatus('pending');
    } else if (containers.baseNode?.status === BaseNodeStatus.ACTIVE) {
      setBaseNodeStatus('active');
    } else {
      setBaseNodeStatus('inactive');
    }
  }, [containers.baseNode?.status]);

  const BaseNodeTitle = () => {
    return (
      <Box
        style={{
          display: 'flex',
          flexDirection: 'row',
          alignItems: 'center',
          gap: 8,
          justifyContent: 'space-between',
          width: '100%',
        }}
      >
        <Typography variant="h3" sx={typography.header}>
          {t.baseNode.title}
        </Typography>
        {containers.baseNode?.status && (
          <StatusChip
            label={
              <span>
                <strong>{containers.baseNode?.status}</strong>
              </span>
            }
            color={
              containers.baseNode?.status === BaseNodeStatus.ACTIVE
                ? 'success'
                : 'info'
            }
          />
        )}
      </Box>
    );
  };

  const BaseNodeNetwork = () => {
    return (
      <LabelBoxVertical>
        <Typography variant="body1" sx={typography.defaultMedium}>
          {t.baseNode.tari_network_label}: {network}
        </Typography>
      </LabelBoxVertical>
    );
  };

  const BaseNodeInfo = () => {
    return (
      <Box>
        <Typography variant="body1" sx={typography.smallHeavy}>
          {t.baseNode.blockInfo.height}
          <span style={typography.smallMedium}>
            {appState?.node?.chain_height || 'Not found'}
          </span>
        </Typography>
        <Typography variant="body1" sx={typography.smallHeavy}>
          {t.baseNode.blockInfo.time}
          <span style={typography.smallMedium}>time</span>
        </Typography>
        <Typography variant="body1" sx={typography.smallHeavy}>
          {t.baseNode.blockInfo.status}
          <span style={typography.smallMedium}>
            {appState?.node?.sync_status || 'Not found'}
          </span>
        </Typography>
      </Box>
    );
  };

  switch (baseNodeStatus) {
    case 'active':
      return (
        <BaseNodeBox>
          <ContentBox>
            <BaseNodeTitle />
            <BaseNodeNetwork />
          </ContentBox>
          <ContentBox>
            <TransparentButton onClick={stop}>
              {t.baseNode.stop}
            </TransparentButton>
            <BaseNodeInfo />
          </ContentBox>
        </BaseNodeBox>
      );
    case 'pending':
      return (
        <BaseNodeBox>
          <ContentBox>
            <BaseNodeTitle />
            <BaseNodeNetwork />
          </ContentBox>
          <ContentBox>
            <TransparentButton onClick={stop}>
              {t.baseNode.stop}
            </TransparentButton>
            <BaseNodeInfo />
          </ContentBox>
        </BaseNodeBox>
      );
    case 'inactive':
    default:
      return (
        <DefaultBox>
          <ContentBox>
            <BaseNodeTitle />
            <BaseNodeNetwork />
          </ContentBox>
          <ContentBox>
            <Button variant="contained" onClick={start}>
              {t.baseNode.start}
            </Button>
            <BaseNodeInfo />
          </ContentBox>
        </DefaultBox>
      );
  }
}

export default BaseNodeWidget;

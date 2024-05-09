// import { useEffect } from 'react';
import { Button, Typography, Box, CircularProgress } from '@mui/material';
import { LabelBoxVertical } from '../../components/StyledComponents';
import t from '../../locales';
import typography from '../../styles/styles/typography';
import useAppStateStore from '../../store/appStateStore';
import { BaseNodeStatus } from '../../store/types';
import {
  StatusChip,
  DefaultBox,
  GradientBox,
  TransparentButton,
} from '../../components/StyledComponents';

function BaseNodeWidget() {
  const { appState, containers, startBaseNode, stopBaseNode } =
    useAppStateStore();

  function start() {
    startBaseNode();
  }

  function stop() {
    stopBaseNode();
  }

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
          {t.baseNode.tari_network_label}:{' '}
          {appState?.config?.settings?.saved_settings?.tari_network ||
            'Not found'}
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

  const BaseNode = () => {
    switch (containers.baseNode?.status) {
      case BaseNodeStatus.WAITING:
      case BaseNodeStatus.SHUTTINGDOWN:
      case BaseNodeStatus.STARTING:
      case BaseNodeStatus.PENDING:
        return (
          <GradientBox>
            <BaseNodeTitle />
            <BaseNodeNetwork />
            <Box>
              <CircularProgress />
            </Box>
            <Box>
              <TransparentButton onClick={stop}>
                Stop Base Node
              </TransparentButton>
            </Box>
            <BaseNodeInfo />
          </GradientBox>
        );
      case BaseNodeStatus.ACTIVE:
        return (
          <GradientBox>
            <BaseNodeTitle />
            <BaseNodeNetwork />
            <Box>
              <TransparentButton onClick={stop}>
                Stop Base Node
              </TransparentButton>
            </Box>
            <BaseNodeInfo />
          </GradientBox>
        );
      case BaseNodeStatus.INACTIVE:
      default:
        return (
          <DefaultBox>
            <BaseNodeTitle />
            <BaseNodeNetwork />
            <Button variant="contained" onClick={start}>
              Start Base Node
            </Button>
            <BaseNodeInfo />
          </DefaultBox>
        );
    }
  };

  return <>{<BaseNode />}</>;
}

export default BaseNodeWidget;

import { Typography, Divider, Box } from '@mui/material';
import { styled } from '@mui/material/styles';
import useAppStateStore from '../../../../store/appStateStore';
import typography from '../../../../styles/styles/typography';

const CustomGrid = styled(Box)(({ theme }) => ({
  display: 'grid',
  gridTemplateColumns: '1fr 1fr 100px',
  gridGap: theme.spacing(2),
  width: '100%',
}));

const CustomGridContainer = styled(Box)(({ theme }) => ({
  display: 'flex',
  width: '100%',
  flexDirection: 'column',
  gap: theme.spacing(2),
}));

function Containers() {
  const containers = useAppStateStore((state) => state.containers);
  const items = [
    {
      container: 'Tor',
      status: containers ? containers.tor?.status : '...',
      cpu: containers ? containers.tor?.stats?.cpu?.toFixed(2) : '...',
    },
    {
      container: 'Minotari Node',
      status: containers ? containers.baseNode?.status : '...',
      cpu: containers ? containers.baseNode?.stats?.cpu?.toFixed(2) : '...',
    },
    {
      container: 'Sha3 Miner',
      status: containers ? containers.sha3Miner?.status : '...',
      cpu: containers ? containers.sha3Miner?.stats?.cpu?.toFixed(2) : '...',
    },
    {
      container: 'Shared Volume',
      status: containers ? containers.sharedVolume?.status : '...',
      cpu: containers ? containers.sharedVolume?.stats?.cpu?.toFixed(2) : '...',
    },
    {
      container: 'Merge Mining Proxy',
      status: containers ? containers.mmProxy?.status : '...',
      cpu: containers ? containers.mmProxy?.stats?.cpu?.toFixed(2) : '...',
    },
    {
      container: 'Loki',
      status: containers?.loki?.status,
      cpu: containers ? containers.loki?.stats?.cpu?.toFixed(2) : '...',
    },
    {
      container: 'Grafana',
      status: containers?.grafana?.status,
      cpu: containers ? containers.grafana?.stats?.cpu?.toFixed(2) : '...',
    },
    {
      container: 'Xmrig',
      status: containers?.xmrig?.status,
      cpu: containers ? containers.xmrig?.stats?.cpu?.toFixed(2) : '...',
    },
  ];

  const renderItems = items.map((item) => {
    return (
      <>
        <Divider />
        <CustomGrid>
          <Typography variant="h6" sx={typography.smallHeavy}>
            {item.container}
          </Typography>
          <Typography variant="body2" sx={typography.smallMedium}>
            {item.status}
          </Typography>
          <Typography variant="body2" sx={typography.smallMedium}>
            {item.cpu}
          </Typography>
        </CustomGrid>
      </>
    );
  });

  return (
    <CustomGridContainer>
      <CustomGrid>
        <Typography variant="h5" sx={typography.defaultHeavy}>
          Container
        </Typography>
        <Typography variant="h5" sx={typography.defaultHeavy}>
          Status
        </Typography>
        <Typography variant="h5" sx={typography.defaultHeavy}>
          CPU
        </Typography>
      </CustomGrid>
      {renderItems}
    </CustomGridContainer>
  );
}

export default Containers;

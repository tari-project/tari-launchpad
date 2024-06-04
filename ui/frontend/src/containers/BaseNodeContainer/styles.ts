import { styled } from '@mui/material/styles';
import { Box } from '@mui/material';
import gradients from '../../styles/styles/gradients';

const minHeight = 350;

export const FooterBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(2),
  alignItems: 'flex-start',
}));

export const DefaultBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(2),
  background: theme.palette.background.paper,
  padding: theme.spacing(3),
  border: `1px solid ${theme.palette.divider}`,
  borderRadius: theme.spacing(1),
  alignItems: 'flex-start',
  justifyContent: 'space-between',
  minHeight: minHeight,
}));

export const InfoBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(1),
  background:
    theme.palette.mode === 'dark'
      ? theme.palette.background.paper
      : theme.palette.background.default,
  width: '100%',
  maxWidth: 400,
  padding: theme.spacing(3),
  borderRadius: 8,
  alignItems: 'flex-start',
}));

export const BaseNodeBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'flex-start',
  justifyContent: 'space-between',
  gap: theme.spacing(2),
  background:
    theme.palette.mode === 'dark' ? gradients.baseNodeDark : gradients.baseNode,
  padding: theme.spacing(3),
  borderRadius: theme.spacing(1),
  minHeight: minHeight,
  // Force dark mode
  ...((theme.palette.mode === 'light' || theme.palette.mode === 'dark') && {
    color: '#fff',
    '& MuiTypography-root': {
      color: '#fff',
    },
    '& MuiCircularProgress-root': {
      color: 'rgba(255, 255, 255, 0.5)',
    },
  }),
}));

export const ContentBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(2),
  alignItems: 'flex-start',
  width: '100%',
}));

export const HelpTextBox = styled(Box)(() => ({
  display: 'flex',
  flexDirection: 'row',
  alignItems: 'center',
}));

export const TabContainer = styled(Box)(({ theme }) => ({
  width: '100%',
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(3),
  alignItems: 'center',
}));

export const ItemsContainer = styled(Box)(({ theme }) => ({
  display: 'grid',
  gap: theme.spacing(3),
  gridTemplateColumns: 'repeat(auto-fill, minmax(400px, 1fr))',
}));

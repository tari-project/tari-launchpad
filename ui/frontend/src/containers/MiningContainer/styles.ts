import { styled } from '@mui/material/styles';
import { Box, CircularProgress } from '@mui/material';
import gradients from '../../styles/styles/gradients';
import colors from '../../styles/styles/colors';

const minHeight = 320;

export const CircularProgressLight = styled(CircularProgress)(({}) => ({
  color: colors.light.textSecondary,
}));

export const FooterBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(0),
  alignItems: 'flex-start',
}));

export const ContentBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(2),
  alignItems: 'flex-start',
  width: '100%',
}));

export const MiningBoxOuter = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'row',
  justifyContent: 'space-between',
  gap: theme.spacing(3),
  background: theme.palette.background.paper,
  padding: theme.spacing(3),
  border: `1px solid ${theme.palette.divider}`,
  borderRadius: theme.spacing(1),
  minHeight: minHeight,
}));

export const MiningBoxInner = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(2),
  alignItems: 'flex-start',
  justifyContent: 'space-between',
  width: '100%',
  flexGrow: 1,
}));

export const ShaMiningBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'row',
  justifyContent: 'space-between',
  background:
    theme.palette.mode === 'dark' ? gradients.tariDark : gradients.tari,
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

export const MergeMiningBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'row',
  justifyContent: 'space-between',
  background:
    theme.palette.mode === 'dark' ? gradients.mergedDark : gradients.merged,
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

export const MiningButtonBox = styled(Box)(({ theme }) => ({
  color: 'rgba(255, 255, 255, 0.8)',
  backgroundColor: 'rgba(255, 255, 255, 0.22)',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'flex-start',
  gap: theme.spacing(2),
  padding: `2px 16px 2px 24px`,
  borderRadius: theme.spacing(1),
  '&:hover': {
    backgroundColor: 'rgba(255, 255, 255, 0.35)',
  },
}));

export const AmountBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'row',
  gap: theme.spacing(0.5),
  alignItems: 'baseline',
}));

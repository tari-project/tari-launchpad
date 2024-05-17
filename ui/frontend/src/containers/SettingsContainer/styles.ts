import { styled } from '@mui/material/styles';
import { Box, Tab } from '@mui/material';

export const SettingsBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(3),
}));

export const SettingsContainer = styled(Box)(({ theme }) => ({
  paddingBottom: theme.spacing(4),
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(4),
}));

export const SettingsTab = styled(Tab)(({ theme }) => ({
  borderRadius: '6px 0 0 6px',
  alignItems: 'flex-start',
  color: theme.palette.primary.dark,
  fontSize: '14px',
  '&.Mui-selected': {
    backgroundColor: theme.palette.divider,
    color: theme.palette.primary.main,
    fontFamily: '"AvenirHeavy", sans-serif',
  },
}));

export const SettingsPanel = styled(Box)(({ theme }) => ({
  display: 'flex',
  height: 500,
  width: '100%',
  padding: `${theme.spacing(5)} ${theme.spacing(10)}`,
  flexDirection: 'column',
  gap: theme.spacing(5),
}));

export const ScrollBarBox = styled(Box)(({ theme }) => ({
  overflowY: 'scroll',
  scrollbarWidth: 'thin',
  scrollbarColor:
    theme.palette.mode === 'light'
      ? `${theme.palette.grey[300]} transparent`
      : `${theme.palette.grey[800]} transparent`,
  '&::-webkit-scrollbar': {
    width: '8px',
  },
  '&::-webkit-scrollbar-track': {
    backgroundColor: 'transparent',
  },
  '&::-webkit-scrollbar-thumb': {
    backgroundColor:
      theme.palette.mode === 'light'
        ? theme.palette.grey[300]
        : theme.palette.grey[800],
    borderRadius: '4px',
  },
}));

export const ThemeSwitchBox = styled(Box)(({ theme }) => ({
  padding: `${theme.spacing(2)} ${theme.spacing(3)}`,
  display: 'flex',
  justifyContent: 'center',
  alignItems: 'center',
}));

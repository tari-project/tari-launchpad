import { styled } from '@mui/material/styles';
import Box from '@mui/material/Box';

export const MenuContainer = styled(Box)(({ theme }) => ({
  position: 'fixed',
  top: '10px',
  right: theme.spacing(2),
  zIndex: 1000,
}));

export const Main = styled('main', {
  shouldForwardProp: (prop) => prop !== 'open',
})<{
  open?: boolean;
  contentWidth?: string;
  drawerWidth: number;
}>(({ theme, open, contentWidth, drawerWidth }) => ({
  flexGrow: 1,
  backgroundColor:
    theme.palette.mode === 'light'
      ? theme.palette.background.paper
      : theme.palette.background.default,
  minHeight: '100vh',
  padding: theme.spacing(3),
  transition: theme.transitions.create('margin', {
    easing: theme.transitions.easing.sharp,
    duration: theme.transitions.duration.leavingScreen,
  }),
  marginRight: -drawerWidth,
  ...(open && {
    transition: theme.transitions.create('margin', {
      easing: theme.transitions.easing.easeOut,
      duration: theme.transitions.duration.enteringScreen,
    }),
    marginRight: 0,
  }),
  width: contentWidth === 'fullScreen' ? '100%' : 'calc(100% - 600px)',
  position: 'relative',
}));

export const DrawerHeader = styled('div')(({ theme }) => ({
  display: 'flex',
  alignItems: 'center',
  ...theme.mixins.toolbar,
  justifyContent: 'space-between',
  position: 'sticky',
  top: 0,
  zIndex: 10,
}));

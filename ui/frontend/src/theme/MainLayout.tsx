import { useState, useEffect } from 'react';
import { styled } from '@mui/material/styles';
import Box from '@mui/material/Box';
import Drawer from '@mui/material/Drawer';
import CssBaseline from '@mui/material/CssBaseline';
import Divider from '@mui/material/Divider';
import Button from '@mui/material/Button';
import Switch from '@mui/material/Switch';
import FormGroup from '@mui/material/FormGroup';
import FormControlLabel from '@mui/material/FormControlLabel';
import './theme.css';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { light, dark, componentSettings } from './tokens';
import useThemeStore from '../store/themeStore';
import ExpertViewTabs from '../containers/Dashboard/ExpertView/ExpertViewTabs';
import { Container } from '@mui/material';
import TariLogo from '../assets/tari-logo';
import { SnackbarProvider } from 'notistack';
// import { SnackbarCloseButton } from '../containers/TBotContainer/TBot';
// import { CustomSnackbarContent } from '../containers/TBotContainer/TBot';
import { MaterialDesignContent } from 'notistack';
import Fade from '../components/Fade';
import useAppStateStore from '../store/appStateStore';
import SvgMonitor from '../styles/Icons/Monitor';
import typography from '../styles/styles/typography';
import SvgSetting from '../styles/Icons/Setting2';
import { DrawerHeader, Main, MenuContainer } from './styles';
import { useShallow } from 'zustand/react/shallow';

const StyledMaterialDesignContent = styled(MaterialDesignContent)(
  ({ theme }) => ({
    '&.notistack-MuiContent-info': {
      backgroundColor: theme.palette.background.paper,
      border: `1px solid rgba(255,255,255,0.03)`,
      color: theme.palette.text.primary,
      boxShadow: 'none',
      borderRadius: '8px',
    },
    '&.notistack-MuiContent-default': {
      backgroundColor:
        theme.palette.mode === 'light'
          ? theme.palette.background.paper
          : 'rgba(255,255,255,0.04)',
      color: theme.palette.text.primary,
      boxShadow: 'none',
      width: '400px',
    },
    '&.notistack-SnackbarContainer': {
      border: `1px solid ${theme.palette.divider}`,
      color: theme.palette.text.primary,
      boxShadow: 'none',
      width: 600,
    },
  })
);

export default function MainLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const [open, setOpen] = useState(true);
  const [contentWidth, setContentWidth] = useState<'normal' | 'fullScreen'>(
    'normal'
  );
  const [drawerWidth, setDrawerWidth] = useState(window.innerWidth * 0.5);
  const { setOpenSettings } = useAppStateStore(
    useShallow((state) => ({
      setOpenSettings: state.setOpenSettings,
    }))
  );
  const { themeMode } = useThemeStore(
    useShallow((state) => ({
      themeMode: state.themeMode,
    }))
  );
  const headerHeight = 64;

  const themeOptions = (mode: string) => {
    return mode === 'light' ? light : dark;
  };

  const theme = createTheme({
    ...themeOptions(themeMode),
    ...componentSettings,
  });

  const darkTheme = createTheme({
    ...dark,
    ...componentSettings,
  });

  const handleDrawerOpen = () => {
    setOpen(true);
  };

  const handleDrawerClose = () => {
    setOpen(false);
    if (contentWidth === 'fullScreen') {
      setContentWidth('normal');
    }
  };

  const handleFullScreenToggle = () => {
    if (contentWidth === 'fullScreen') {
      setContentWidth('normal');
    } else {
      setContentWidth('fullScreen');
    }
  };

  function handleOpenSettings() {
    setOpenSettings(true);
  }

  useEffect(() => {
    const handleResize = () => {
      setDrawerWidth(window.innerWidth * 0.5);
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, []);

  const ExpertViewToggle = () => {
    return (
      <FormGroup>
        <FormControlLabel
          control={
            <Switch
              checked={open}
              onChange={open ? handleDrawerClose : handleDrawerOpen}
              inputProps={{ 'aria-label': 'toggle expert view' }}
              style={{
                marginRight: '4px',
              }}
            />
          }
          label="Expert View"
          labelPlacement="end"
        />
      </FormGroup>
    );
  };

  const Menu = () => {
    return (
      <Box
        style={{
          display: 'flex',
          flexDirection: 'row',
          gap: theme.spacing(3),
          alignItems: 'center',
        }}
      >
        <ThemeProvider theme={theme}>
          <Button
            onClick={handleOpenSettings}
            size="medium"
            startIcon={<SvgSetting />}
            style={{
              color: open ? '#fff' : 'inherit',
            }}
          >
            Settings
          </Button>
        </ThemeProvider>
        <ExpertViewToggle />
      </Box>
    );
  };

  return (
    <ThemeProvider theme={open ? darkTheme : theme}>
      <SnackbarProvider
        anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
        maxSnack={2}
        preventDuplicate
        TransitionComponent={Fade}
        autoHideDuration={20000}
        Components={{
          success: StyledMaterialDesignContent,
          error: StyledMaterialDesignContent,
          info: StyledMaterialDesignContent,
          default: StyledMaterialDesignContent,
        }}
        classes={{
          containerRoot:
            open || themeMode === 'dark'
              ? 'notistack-container-dark'
              : 'notistack-container-light',
        }}
      >
        <ThemeProvider theme={theme}>
          <Box
            sx={{
              display: 'flex',
            }}
          >
            <CssBaseline enableColorScheme />
            <Box
              style={{
                height: headerHeight,
                width: '100%',
                background:
                  themeMode === 'light'
                    ? theme.palette.background.default
                    : theme.palette.background.paper,
                position: 'fixed',
                zIndex: 300,
                padding: theme.spacing(2),
              }}
            >
              <TariLogo fill={theme.palette.text.primary} />
            </Box>
            <ThemeProvider
              theme={open || themeMode === 'dark' ? darkTheme : theme}
            >
              <MenuContainer>
                <Menu />
              </MenuContainer>
            </ThemeProvider>
            <Main open={open} contentWidth={contentWidth} drawerWidth={0}>
              <DrawerHeader />
              <Container>{children}</Container>
            </Main>
            <ThemeProvider theme={darkTheme}>
              <Drawer
                sx={{
                  width: contentWidth === 'fullScreen' ? '100vw' : drawerWidth,
                  flexShrink: 0,
                  zIndex: 400,
                  display: open ? 'block' : 'none',
                  '& .MuiDrawer-paper': {
                    width:
                      contentWidth === 'fullScreen' ? '100vw' : drawerWidth,
                    backgroundColor: darkTheme.palette.background.default,
                  },
                }}
                variant="persistent"
                anchor="right"
                open={open}
              >
                <Box
                  style={{
                    width: '100%',
                    background: darkTheme.palette.background.paper,
                    height: headerHeight,
                    position: 'fixed',
                  }}
                ></Box>
                <Box
                  style={{
                    position: 'absolute',
                    top: '108px',
                    right: theme.spacing(3),
                    zIndex: 1100,
                  }}
                >
                  <Button
                    onClick={handleFullScreenToggle}
                    startIcon={<SvgMonitor />}
                    style={typography.smallMedium}
                    color="inherit"
                  >
                    {contentWidth === 'fullScreen'
                      ? 'Exit Full Screen'
                      : 'Open Full Screen'}
                  </Button>
                </Box>
                <Divider />
                <Box
                  style={{
                    position: 'absolute',
                    top: headerHeight,
                    zIndex: 1000,
                    width: '100%',
                  }}
                >
                  <ExpertViewTabs />
                </Box>
              </Drawer>
            </ThemeProvider>
          </Box>
        </ThemeProvider>
      </SnackbarProvider>
    </ThemeProvider>
  );
}

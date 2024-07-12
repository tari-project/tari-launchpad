import { useState, useEffect } from 'react';
import { styled } from '@mui/material/styles';
import Box from '@mui/material/Box';
import Drawer from '@mui/material/Drawer';
import CssBaseline from '@mui/material/CssBaseline';
import Divider from '@mui/material/Divider';
import Button from '@mui/material/Button';
import './theme.css';
import { ThemeProvider, createTheme } from '@mui/material/styles';
import { light, dark, componentSettings } from './tokens';
import useThemeStore from '../store/themeStore';
import ExpertViewTabs from '../containers/Dashboard/ExpertView/ExpertViewTabs';
import { Container } from '@mui/material';
import { SnackbarProvider } from 'notistack';
import { MaterialDesignContent } from 'notistack';
import Fade from '../components/Fade';
import SvgMonitor from '../styles/Icons/Monitor';
import typography from '../styles/styles/typography';
import { DrawerHeader, Main } from './styles';
import { useShallow } from 'zustand/react/shallow';
import TitleBar from '../containers/TitleBar/TitleBar';
import { appBorderRadius } from './tokens';

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

  useEffect(() => {
    const handleResize = () => {
      setDrawerWidth(window.innerWidth * 0.5);
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, []);

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
              borderRadius: `${appBorderRadius}px`,
              overflow: 'hidden',
              backgroundColor:
                theme.palette.mode === 'light'
                  ? theme.palette.background.paper
                  : theme.palette.background.default,
            }}
          >
            <CssBaseline enableColorScheme />
            <Box
              sx={{
                display: 'flex',
                height: '100%',
                overflowY: 'auto',
                boxSizing: 'border-box',
                borderRadius: `${appBorderRadius}px`,
              }}
            >
              <Box
                style={{
                  backgroundColor:
                    theme.palette.mode === 'light'
                      ? theme.palette.background.default
                      : theme.palette.background.paper,
                  height: headerHeight,
                  zIndex: 10,
                  position: 'fixed',
                  width: '100%',
                  borderRadius: `${appBorderRadius}px ${appBorderRadius}px 0 0`,
                }}
              ></Box>
              <TitleBar
                open={open}
                handleDrawerClose={handleDrawerClose}
                handleDrawerOpen={handleDrawerOpen}
                fullScreen={contentWidth === 'fullScreen'}
              />
              <Main open={open} contentWidth={contentWidth} drawerWidth={0}>
                <DrawerHeader />
                <Container>{children}</Container>
              </Main>
              <ThemeProvider theme={darkTheme}>
                <Drawer
                  sx={{
                    // width:
                    //   contentWidth === 'fullScreen' ? '100vw' : drawerWidth,
                    // flexShrink: 0,
                    // zIndex: 400,
                    // opacity: open ? 1 : 0,
                    // visibility: open ? 'visible' : 'hidden',
                    // transition: 'opacity 0.5s ease, visibility 0.5s ease',
                    // '& .MuiDrawer-paper': {
                    //   width:
                    //     contentWidth === 'fullScreen' ? '100vw' : drawerWidth,
                    //   backgroundColor: darkTheme.palette.background.default,
                    // },

                    width: open
                      ? contentWidth === 'fullScreen'
                        ? '100vw'
                        : drawerWidth
                      : 0,
                    // flexShrink: 0,
                    zIndex: 400,
                    opacity: open ? 1 : 0,
                    visibility: open ? 'visible' : 'hidden',
                    transition:
                      'opacity 0.3s ease, visibility 0.3s ease, width 0.3s ease',
                    '& .MuiDrawer-paper': {
                      width: open
                        ? contentWidth === 'fullScreen'
                          ? '100vw'
                          : drawerWidth
                        : 0,
                      backgroundColor: darkTheme.palette.background.default,
                      transition: 'width 0.5s ease', // Ensure smooth transition for width as well
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
          </Box>
        </ThemeProvider>
      </SnackbarProvider>
    </ThemeProvider>
  );
}

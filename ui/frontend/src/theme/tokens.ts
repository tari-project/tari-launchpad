//  Copyright 2022. The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

import { ThemeOptions } from '@mui/material/styles';
import {
  tariPurple,
  grey,
  teal,
  gothic,
  success,
  info,
  warning,
  error,
} from './colors';
import gradients from '../styles/styles/gradients';

// begin
// export const font = {
//   400: 'AvenirRegular',
//   500: 'AvenirMedium',
//   800: 'AvenirHeavy',
// };

// const typography = {
//   header: {
//     fontSize: 32,
//     lineHeight: '44px',
//     fontFamily: 'AvenirHeavy',
//     fontWeight: 500,
//   },
//   subheader: {
//     fontSize: 24,
//     lineHeight: '38px',
//     fontFamily: 'AvenirHeavy',
//     fontWeight: 500,
//   },
//   defaultHeavy: {
//     fontSize: 16,
//     lineHeight: '25.6px',
//     fontFamily: 'AvenirHeavy',
//     fontWeight: 500,
//   },
//   defaultMedium: {
//     fontSize: 16,
//     lineHeight: '25.6px',
//     fontFamily: 'AvenirMedium',
//     fontWeight: 500,
//   },
//   defaultUnder: {
//     fontSize: 16,
//     lineHeight: '25.6px',
//     fontFamily: 'AvenirMedium',
//     textDecoration: 'underline',
//     fontWeight: 500,
//   },
//   smallHeavy: {
//     fontSize: 14,
//     lineHeight: '22.4px',
//     fontFamily: 'AvenirHeavy',
//     fontWeight: 500,
//   },
//   smallMedium: {
//     fontSize: 14,
//     lineHeight: '22.4px',
//     fontFamily: 'AvenirMedium',
//     fontWeight: 500,
//   },
//   smallUnder: {
//     fontSize: 14,
//     lineHeight: '22.4px',
//     fontFamily: 'AvenirMedium',
//     textDecoration: 'underline',
//     fontWeight: 500,
//   },
//   microHeavy: {
//     fontSize: 12,
//     lineHeight: '18px',
//     fontFamily: 'AvenirHeavy',
//     fontWeight: 500,
//   },
//   microMedium: {
//     fontSize: 12,
//     lineHeight: '18px',
//     fontFamily: 'AvenirMedium',
//     fontWeight: 500,
//   },
//   microRegular: {
//     fontSize: 12,
//     lineHeight: '18px',
//     fontFamily: 'AvenirRegular',
//     fontWeight: 500,
//   },
//   microOblique: {
//     fontSize: 12,
//     lineHeight: '18px',
//     fontFamily: 'AvenirRegular',
//     fontStyle: 'italic',
//     fontWeight: 500,
//   },
// };

// end

export const componentSettings: ThemeOptions = {
  shape: {
    borderRadius: 8,
  },
  spacing: 8,
  typography: {
    fontFamily: '"AvenirMedium", sans-serif',
    fontSize: 14,
    body1: {},
    body2: {
      lineHeight: '1.5rem',
    },
    h1: {
      fontSize: '2.2rem',
      lineHeight: '3.2rem',
      fontFamily: '"AvenirHeavy", sans-serif',
    },
    h2: {
      fontSize: '1.9rem',
      lineHeight: '2.9rem',
      fontFamily: '"AvenirHeavy", sans-serif',
    },
    h3: {
      fontSize: '1.6rem',
      lineHeight: '2.6rem',
      fontFamily: '"AvenirHeavy", sans-serif',
    },
    h4: {
      fontSize: '1.3rem',
      lineHeight: '2.3rem',
      fontFamily: '"AvenirHeavy", sans-serif',
    },
    h5: {
      fontSize: '1rem',
      lineHeight: '2em',
      fontFamily: '"AvenirHeavy", sans-serif',
    },
    h6: {
      fontSize: '0.875rem',
      lineHeight: '1.8rem',
      fontFamily: '"AvenirHeavy", sans-serif',
    },
  },
  transitions: {
    duration: {
      enteringScreen: 500,
      leavingScreen: 500,
    },
  },
  components: {
    MuiTab: {
      defaultProps: {
        disableRipple: true,
        sx: {
          textTransform: 'none',
          fontSize: 16,
          border: 'none',
          boxShadow: 'none',
          color: (theme) => theme.palette.text.secondary,
          '&.Mui-selected': {
            color: (theme) => theme.palette.text.primary,
          },
          '&:hover': {
            color: (theme) => theme.palette.text.primary,
          },
        },
      },
    },
    MuiTabs: {
      defaultProps: {
        TabIndicatorProps: {
          style: {
            borderRadius: 4,
          },
        },
      },
    },
    MuiTypography: {
      defaultProps: {
        sx: {
          color: (theme) => theme.palette.text.primary,
          boxShadow: 'none',
          '&.MuiTypography-body1': {
            color: (theme) => theme.palette.text.secondary,
          },
          '&.MuiTypography-body2': {
            color: (theme) => theme.palette.text.secondary,
          },
        },
      },
    },
    // MuiTypography: {
    //   defaultProps: {
    //     sx: (props) => ({
    //       ...props.sx,
    //       color: (theme) => theme.palette.text.primary,
    //       boxShadow: 'none',
    //       '&.MuiTypography-body1': {
    //         // Spread existing styles first
    //         ...props.sx?.['&.MuiTypography-body1'],
    //         // Add your custom styles
    //         color: 'red',
    //       },
    //       '&.MuiTypography-body2': {
    //         // Spread existing styles first
    //         ...props.sx?.['&.MuiTypography-body2'],
    //         // Add your custom styles
    //         color: (theme) => theme.palette.text.secondary,
    //       },
    //     }),
    //   },
    // },
    MuiButton: {
      defaultProps: {
        size: 'large',
        disableElevation: true,
        disableRipple: true,
        sx: {
          textTransform: 'none',
          boxShadow: 'none',
        },
      },
      variants: [
        {
          props: { variant: 'contained' },
          style: {
            background: gradients.tari,
            border: `1px solid ${tariPurple[500]}`,
          },
        },
      ],
    },
    MuiPaper: {
      defaultProps: {
        elevation: 0,
        sx: {
          background: (theme) => theme.palette.background.paper,
        },
      },
    },
    MuiTableCell: {
      defaultProps: {
        sx: {
          borderBottom: (theme) => `1px solid ${theme.palette.divider}`,
        },
      },
    },
    MuiDivider: {
      defaultProps: {
        sx: {
          borderBottom: (theme) => `1px solid ${theme.palette.divider}`,
        },
      },
    },
    MuiFormControlLabel: {
      defaultProps: {
        sx: {
          '& .MuiTypography-root': {
            fontSize: '0.875rem',
            lineHeight: '1.8rem',
            color: (theme) => theme.palette.text.primary,
          },
        },
      },
    },
    MuiCircularProgress: {
      defaultProps: {
        thickness: 4,
        sx: {
          color: (theme) => theme.palette.primary.main,
        },
      },
    },
    MuiTextField: {
      defaultProps: {
        variant: 'outlined',
        fullWidth: true,
        margin: 'normal',
        sx: {
          boxShadow: 'none',
          '& .MuiOutlinedInput-root': {
            '& fieldset': {
              borderColor: (theme) => theme.palette.divider,
            },
          },
          '&:hover fieldset': {
            borderColor: (theme) => theme.palette.divider,
          },
        },
        InputLabelProps: {
          shrink: true,
        },
      },
    },
    MuiDialog: {
      defaultProps: {
        sx: {
          padding: 0,
          '& .MuiDialogContent-root': { padding: 0 },
          '& .MuiDialog-paper': {
            border: (theme) => `1px solid ${theme.palette.divider}`,
            boxShadow: '0 0 40px #00000011',
          },
        },
        BackdropProps: {
          sx: (theme) => ({
            backdropFilter: 'blur(2px)',
            backgroundColor:
              theme.palette.mode === 'light'
                ? 'rgba(250, 250, 250, 0.5)'
                : 'rgba(0, 0, 0, 0.5)',
          }),
        },
      },
    },
    MuiChip: {
      defaultProps: {
        sx: {
          fontSize: 12,
          height: 26,
          '& strong': {
            fontFamily: '"AvenirHeavy", sans-serif',
          },
        },
      },
      variants: [
        {
          props: { color: 'success' },
          style: ({ theme }) => ({
            background:
              theme.palette.mode === 'light'
                ? theme.palette.success.light
                : theme.palette.background.paper,
            color:
              theme.palette.mode === 'light'
                ? theme.palette.success.contrastText
                : theme.palette.success.main,
          }),
        },
        {
          props: { color: 'warning' },
          style: ({ theme }) => ({
            background:
              theme.palette.mode === 'light'
                ? theme.palette.warning.light
                : theme.palette.background.paper,
            color: theme.palette.warning.main,
          }),
        },
        {
          props: { color: 'error' },
          style: ({ theme }) => ({
            background:
              theme.palette.mode === 'light'
                ? theme.palette.error.light
                : theme.palette.background.paper,
            color: theme.palette.error.main,
          }),
        },
        {
          props: { color: 'info' },
          style: ({ theme }) => ({
            background:
              theme.palette.mode === 'light'
                ? theme.palette.info.light
                : theme.palette.background.paper,
            color: theme.palette.info.main,
          }),
        },
      ],
    },
  },
};

export const light: ThemeOptions = {
  palette: {
    mode: 'light',
    primary: {
      main: tariPurple[600],
      dark: tariPurple[800],
      light: tariPurple[500],
    },
    secondary: {
      main: gothic[400],
      dark: gothic[500],
      light: teal[400],
    },
    divider: 'rgba(0,0,0,0.06)',
    text: {
      primary: grey[950],
      secondary: grey[600],
      disabled: grey[400],
    },
    background: {
      default: grey[50],
      paper: '#fff',
    },
    success: {
      main: success[200],
      dark: success[300],
      light: success[100],
      contrastText: success[300],
    },
    warning: {
      main: warning[200],
      dark: warning[300],
      light: warning[100],
      contrastText: warning[300],
    },
    error: {
      main: error[200],
      dark: error[300],
      light: error[100],
      contrastText: error[300],
    },
    info: {
      main: info[200],
      dark: info[300],
      light: info[100],
      contrastText: info[300],
    },
  },
};

export const dark: ThemeOptions = {
  palette: {
    mode: 'dark',
    primary: {
      main: tariPurple[500],
      dark: tariPurple[200],
      light: tariPurple[50],
    },
    secondary: {
      main: teal[400],
      dark: teal[300],
      light: gothic[400],
    },
    divider: 'rgba(255,255,255,0.06)',
    text: {
      primary: '#FFFFFF',
      secondary: grey[300],
      disabled: 'rgba(255,255,255,0.4)',
    },
    background: {
      default: grey[950],
      paper: grey[900],
    },
    success: {
      main: success[200],
      dark: success[100],
      light: success[300],
      contrastText: success[100],
    },
    warning: {
      main: warning[200],
      dark: warning[100],
      light: warning[300],
      contrastText: warning[100],
    },
    error: {
      main: error[200],
      dark: error[100],
      light: error[300],
      contrastText: error[100],
    },
    info: {
      main: info[200],
      dark: info[100],
      light: info[300],
      contrastText: info[100],
    },
  },
};

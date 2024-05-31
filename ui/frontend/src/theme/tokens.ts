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
  success,
  info,
  warning,
  error,
  brightGreen,
} from './colors';
import gradients from '../styles/styles/gradients';
import typography from '../styles/styles/typography';

export const componentSettings: ThemeOptions = {
  shape: {
    borderRadius: 8,
  },
  spacing: 8,
  typography: {
    fontFamily: '"PoppinsMedium", sans-serif',
    fontSize: 14,
    body1: {
      fontSize: '14px',
      // letterSpacing: '0.1px',
    },
    body2: {
      fontSize: '12px',
      lineHeight: '1.5rem',
    },
    h1: {
      fontSize: '2.2rem',
      lineHeight: '3.2rem',
      fontFamily: '"PoppinsBold", sans-serif',
    },
    h2: {
      fontSize: '1.9rem',
      lineHeight: '2.9rem',
      fontFamily: '"PoppinsBold", sans-serif',
    },
    h3: {
      fontSize: '1.6rem',
      lineHeight: '2.6rem',
      fontFamily: '"PoppinsBold", sans-serif',
    },
    h4: {
      fontSize: '1.3rem',
      lineHeight: '2.3rem',
      fontFamily: '"PoppinsBold", sans-serif',
    },
    h5: {
      fontSize: '1rem',
      lineHeight: '2em',
      fontFamily: '"PoppinsBold", sans-serif',
    },
    h6: {
      fontSize: '0.875rem',
      lineHeight: '1.8rem',
      fontFamily: '"PoppinsBold", sans-serif',
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
          fontSize: 14,
          border: 'none',
          boxShadow: 'none',
          borderRadius: '8px',
          color: (theme) => theme.palette.text.secondary,
          '&.Mui-selected': {
            color: (theme) => theme.palette.text.primary,
          },
          '&:hover': {
            color: (theme) => theme.palette.text.primary,
          },
          '&:focus': {
            backgroundColor: (theme) => theme.palette.action.hover,
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
            color: (theme) => theme.palette.text.primary,
          },
          '&.MuiTypography-body2': {
            color: (theme) => theme.palette.text.secondary,
          },
        },
      },
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
            fontSize: '14px',
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
    MuiDialogActions: {
      defaultProps: {
        sx: {
          padding: 2,
        },
      },
    },
    MuiButton: {
      defaultProps: {
        size: 'medium',
        disableElevation: true,
        disableRipple: true,
        sx: {
          textTransform: 'none',
          boxShadow: 'none',
          fontFamily: '"PoppinsMedium", sans-serif',
          height: 44,
          padding: '8px 16px',
        },
      },
      variants: [
        {
          props: { variant: 'contained' },
          style: ({ theme }) => ({
            background:
              theme.palette.mode === 'light'
                ? gradients.tari
                : gradients.tariDark,
            border: `1px solid ${theme.palette.primary.main}`,
            '&:hover': {
              opacity: 0.85,
              borderColor: theme.palette.primary.main,
            },
          }),
        },
        {
          props: { variant: 'contained', disabled: true },
          style: {
            background: 'transparent',
            borderColor: 'transparent',
          },
        },
        {
          props: { variant: 'outlined' },
          style: ({ theme }) => ({
            background:
              theme.palette.mode === 'light'
                ? grey[50]
                : 'rgba(255,255,255,0.08)',
            borderColor:
              theme.palette.mode === 'light'
                ? grey[100]
                : 'rgba(255,255,255,0.12)',
            color: theme.palette.text.primary,
            '&:hover': {
              background:
                theme.palette.mode === 'light'
                  ? grey[100]
                  : 'rgba(255,255,255,0.12)',
              borderColor:
                theme.palette.mode === 'light'
                  ? grey[100]
                  : 'rgba(255,255,255,0.12)',
            },
          }),
        },
      ],
    },
    MuiTextField: {
      defaultProps: {
        variant: 'outlined',
        fullWidth: true,
        // margin: 'normal',
        size: 'small',
        sx: {
          boxShadow: 'none',
          '& .MuiOutlinedInput-root': {
            '& fieldset': {
              borderColor: (theme) => theme.palette.divider,
            },
            '&:hover fieldset': {
              borderColor: (theme) => theme.palette.primary.main,
            },
          },
          '&:hover fieldset': {
            borderColor: (theme) => theme.palette.divider,
          },
        },

        InputProps: {
          style: {
            color: grey[500],
            fontSize: '14px',
            padding: '4px',
          },
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
            backdropFilter: 'blur(1px)',
            backgroundColor:
              theme.palette.mode === 'light'
                ? 'rgba(250, 250, 250, 0.5)'
                : 'rgba(0, 0, 0, 0.5)',
          }),
        },
      },
    },
    MuiDialogTitle: {
      defaultProps: {
        sx: {
          fontFamily: typography.subheader.fontFamily,
          fontSize: typography.subheader.fontSize,
          fontWeight: typography.subheader.fontWeight,
          lineHeight: typography.subheader.lineHeight,
        },
      },
    },
    MuiSwitch: {
      defaultProps: {
        sx: {
          width: 48,
          '&:active': {
            '& .MuiSwitch-thumb': {
              border: '1px solid #000',
            },
            '& + .MuiSwitch-track': {
              opacity: 1,
            },
            '& .MuiSwitch-switchBase.Mui-checked': {},
          },
          '& .MuiSwitch-switchBase': {
            padding: '12px',
            opacity: 1,
            '& + .MuiSwitch-track': {
              border: '1px solid black',
              opacity: 1,
              backgroundColor: (theme) =>
                theme.palette.mode === 'light'
                  ? theme.palette.grey[200]
                  : theme.palette.grey[700],
            },
            '&.Mui-checked': {
              transform: 'translateX(10px)',
              '& + .MuiSwitch-track': {
                opacity: 1,
              },
            },
          },
          '& .MuiSwitch-thumb': {
            border: '1px solid #000',
            backgroundColor: '#fff',
            width: 14,
            height: 14,
          },
        },
      },
    },
    MuiChip: {
      defaultProps: {
        sx: {
          fontSize: 12,
          height: 26,
          '& strong': {
            fontFamily: '"PoppinsMedium", sans-serif',
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
                : 'rgba(255,255,255, 0.08)',
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
                : 'rgba(255,255,255, 0.08)',
            color: theme.palette.warning.main,
          }),
        },
        {
          props: { color: 'error' },
          style: ({ theme }) => ({
            background:
              theme.palette.mode === 'light'
                ? theme.palette.error.light
                : 'rgba(255,255,255, 0.08)',
            color: theme.palette.error.main,
          }),
        },
        {
          props: { color: 'info' },
          style: ({ theme }) => ({
            background:
              theme.palette.mode === 'light'
                ? theme.palette.info.light
                : 'rgba(255,255,255, 0.08)',
            color: theme.palette.info.main,
          }),
        },
        {
          props: { color: 'primary' },
          style: ({ theme }) => ({
            background:
              theme.palette.mode === 'light'
                ? theme.palette.info.light
                : 'rgba(255,255,255, 0.08)',
            color: theme.palette.primary.main,
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
      main: brightGreen[600],
      dark: brightGreen[700],
      light: brightGreen[500],
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
      main: brightGreen[500],
      dark: brightGreen[400],
      light: brightGreen[600],
    },
    divider: 'rgba(255,255,255,0.06)',
    text: {
      primary: '#FFFFFF',
      secondary: grey[300],
      disabled: 'rgba(255,255,255,0.4)',
    },
    background: {
      default: '#040723',
      paper: '#0C0E2A',
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

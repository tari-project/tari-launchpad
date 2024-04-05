import Paper from '@mui/material/Paper';
import TableCell from '@mui/material/TableCell';
import { styled } from '@mui/material/styles';
import Box from '@mui/material/Box';
import IconButton from '@mui/material/IconButton';
import Typography from '@mui/material/Typography';

export const SettingsBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(3),
}));

export const TabStatusLabel = styled(Typography)(({ theme }) => ({
  textTransform: 'none',
  fontSize: 12,
  backgroundColor: theme.palette.success.light,
  color: theme.palette.success.contrastText,
  padding: '5px 15px',
  borderRadius: 20,
}));

export const StyledIconButton = styled(IconButton)(({ theme }) => ({
  color: theme.palette.text.secondary,
  boxShadow: 'none',
}));

export const AccordionIconButton = styled(IconButton)(({ theme }) => ({
  backgroundColor: theme.palette.divider,
  color: theme.palette.primary.main,
  '&:hover': {
    backgroundColor: theme.palette.primary.main,
    color: '#fff',
  },
}));

export const StyledPaper = styled(Paper)(({ theme }) => ({
  padding: theme.spacing(3),
  border: `1px solid ${theme.palette.divider}`,
  // '&:hover': {
  //   border: 'none',
  //   boxShadow: `0px 0px 16px 0px rgba(0, 0, 0, 0.05)`,
  // },
}));

export const InnerHeading = styled(Typography)(({ theme }) => ({
  fontSize: theme.typography.h6.fontSize,
  textTransform: 'uppercase',
  borderBottom: `1px solid ${theme.palette.divider}`,
  padding: theme.spacing(2),
  marginBottom: theme.spacing(2),
  letterSpacing: '1.5px',
}));

export const DataTableCell = styled(TableCell)(() => ({
  fontFamily: "'Courier New', Courier, monospace",
}));

export const CodeBlock = styled(Box)(({ theme }) => ({
  backgroundColor: theme.palette.divider,
  borderRadius: theme.shape.borderRadius,
  padding: theme.spacing(3),
  maxHeight: '400px',
  overflowY: 'scroll',
}));

export const BoxHeading = styled(Box)(({ theme }) => ({
  backgroundColor: '#fafafa',
  borderRadius: theme.shape.borderRadius,
  padding: theme.spacing(3),
  fontFamily: "'Courier New', Courier, monospace",
  boxShadow: '0px 5px 5px rgba(35, 11, 73, 0.10)',
  margin: '10px 5px',
}));

export const BoxHeading2 = styled(Box)(({ theme }) => ({
  padding: theme.spacing(2),
  borderBottom: `1px solid ${theme.palette.divider}`,
}));

export const SubHeading = styled(Typography)(() => ({
  marginTop: '20px',
  marginBottom: '20px',
  textAlign: 'center',
}));

export const DialogContainer = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'center',
  gap: theme.spacing(3),
  width: '100%',
}));

export const GridHeadCell = styled(Box)(({ theme, className }) => ({
  padding: theme.spacing(2),
  fontSize: '0.875rem',
  color: theme.palette.text.primary,
  borderBottom: `1px solid ${theme.palette.divider}`,
  gridArea: `${className}`,
}));

export const GridDataCell = styled(Box)(({ theme, className }) => ({
  padding: theme.spacing(2),
  fontSize: '0.875rem',
  color: theme.palette.text.primary,
  borderBottom: `1px solid ${theme.palette.divider}`,
  fontFamily: "'Courier New', Courier, monospace",
  gridArea: `${className}`,
}));

export const TypographyData = styled(Typography)(({ theme }) => ({
  fontFamily: "'Courier New', Courier, monospace",
  padding: theme.spacing(2),
}));

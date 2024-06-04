import { styled } from '@mui/material/styles';
import {
  Paper,
  TableCell,
  Box,
  IconButton,
  Typography,
  Chip,
  Button,
  CircularProgress,
} from '@mui/material';
import colors from '../styles/styles/colors';

export const CircularProgressLight = styled(CircularProgress)(({}) => ({
  color: colors.light.textSecondary,
}));

export const TabInnerBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'row',
  gap: theme.spacing(3),
  width: '100%',
}));

export const LabelBoxVertical = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(1),
}));

export const LabelBoxHorisontal = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'row',
  alignItems: 'center',
  gap: theme.spacing(2),
  '& > *:first-child': {
    minWidth: 120,
  },
}));

export const LabelWithChip = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'row',
  alignItems: 'center',
  gap: theme.spacing(1),
}));

export const HorisontalButtons = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'row',
  alignItems: 'center',
  gap: theme.spacing(1),
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

export const IconButtonNoPadding = styled(IconButton)(({ theme }) => ({
  color: theme.palette.text.secondary,
  boxShadow: 'none',
  padding: 0,
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

export const StatusChip = styled(Chip)(({ color }) => ({
  backgroundColor:
    color === 'info' ? colors.secondary.info : colors.secondary.on,
  color: color === 'info' ? colors.secondary.infoText : colors.secondary.onText,
}));

export const TransparentButton = styled(Button)(() => ({
  color: '#fff',
  border: 'none',
  backgroundColor: 'rgba(255, 255, 255, 0.22)',
  padding: '8px 16px',
  minWidth: 120,
  '&:hover': {
    backgroundColor: 'rgba(255, 255, 255, 0.35)',
    color: '#fff',
  },
}));

export const TextButton = styled(Button)(({ theme }) => ({
  color: theme.palette.text.primary,
  height: '2rem',
}));

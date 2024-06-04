import { styled } from '@mui/material/styles';
import { Box } from '@mui/material';

export const MessageBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  alignItems: 'flex-start',
  gap: theme.spacing(2),
  width: '100%',
}));

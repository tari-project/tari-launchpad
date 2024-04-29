import React from 'react';
import { Box } from '@mui/material';
import { StyledPaper } from './StyledComponents';
import { useTheme } from '@mui/material/styles';
import { styled } from '@mui/material/styles';

export const MiningBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(2),
}));

function GradientBox({
  children,
  isActive,
  gradient,
}: {
  children: React.ReactNode;
  isActive: boolean;
  gradient: string;
}) {
  const theme = useTheme();

  return (
    <StyledPaper
      style={{
        background: isActive ? gradient : theme.palette.background.paper,
        color: isActive ? '#fff' : theme.palette.text.primary,
      }}
    >
      <MiningBox>{children}</MiningBox>
    </StyledPaper>
  );
}

export default GradientBox;

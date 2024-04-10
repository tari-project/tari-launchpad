import { styled } from '@mui/material/styles';
import { Box } from '@mui/material';

// export const MiningBoxContent = styled.div`
//   display: flex;
//   flex-direction: column;
//   align-items: flex-start;
//   justify-content: space-between;
//   flex: 1;
// `
export const TabInnerBox = styled(Box)(({ theme }) => ({
  display: 'flex',
  flexDirection: 'row',
  gap: theme.spacing(3),
  width: '100%',
}));

// export const NodeIcons = styled.div<{ $color: string }>`
//   position: absolute;
//   width: 80px;
//   min-height: 80px;
//   right: ${({ theme }) => theme.spacing()};
//   top: ${({ theme }) => theme.spacing()};
//   color: ${({ $color }) => $color};

//   & > * {
//     width: 80px;
//     height: 80px;
//     color: inherit;
//     margin-bottom: ${({ theme }) => theme.spacing(0.4)};
//   }
// `

export const NodeIcons = styled(Box)(({ theme }) => ({
  position: 'absolute',
  width: '80px',
  minHeight: '80px',
  right: theme.spacing(1),
  top: theme.spacing(1),

  '& > *': {
    width: '80px',
    height: '80px',
    color: 'inherit',
    marginBottom: theme.spacing(0.4),
  },
}));

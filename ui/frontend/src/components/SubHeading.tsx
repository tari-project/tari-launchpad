import { styled } from '@mui/material/styles';
import { Typography } from '@mui/material';
import typography from '../styles/styles/typography';

const HeadingContainer = styled('div')({
  display: 'flex',
  alignItems: 'center',
});

const Heading = styled(Typography)(({ theme }) => ({
  marginRight: theme.spacing(2),
  color: `${theme.palette.primary.main} !important`,
}));

const Line = styled('hr')(({ theme }) => ({
  flexGrow: 1,
  borderColor: theme.palette.divider,
}));

const SubHeading = ({ text }: { text: string }) => {
  return (
    <HeadingContainer>
      <Heading style={typography.microHeavy}>{text}</Heading>
      <Line />
    </HeadingContainer>
  );
};

export default SubHeading;

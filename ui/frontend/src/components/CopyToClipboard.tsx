import { useState } from 'react';
import SvgCopy from '../styles/Icons/Copy';
import Tooltip from '@mui/material/Tooltip';
import { StyledIconButton } from '../components/StyledComponents';

interface CopyProps {
  copy: string;
}

const CopyToClipboard = ({ copy }: CopyProps) => {
  const [open, setOpen] = useState(false);
  const handleClick = (copyThis: string) => {
    setOpen(true);
    navigator.clipboard.writeText(copyThis);
    setTimeout(() => {
      setOpen(false);
    }, 2000);
  };

  return (
    <>
      <Tooltip
        title={!open ? copy : 'Copied to clipboard'}
        arrow
        enterTouchDelay={0}
        placement="top"
      >
        <StyledIconButton
          onClick={() => handleClick(copy)}
          size="small"
          aria-label="copy to clipboard"
          style={{
            marginLeft: '8px',
          }}
        >
          <SvgCopy
            color="primary"
            style={{
              width: '16px',
              height: '16px',
            }}
          />
        </StyledIconButton>
      </Tooltip>
    </>
  );
};

export default CopyToClipboard;

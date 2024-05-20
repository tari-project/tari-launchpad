import { useEffect } from 'react';
import { Button, Typography } from '@mui/material';
import t from '../../../locales';
import typography from '../../../styles/styles/typography';
import { MiningButtonBox } from '../styles';
import useAppStateStore from '../../../store/appStateStore';
import { MiningType } from '../../../store/types';

function Timer({
  miningType,
  setTimerOn,
  time,
  setTime,
}: {
  miningType: MiningType;
  setTimerOn: (value: boolean) => void;
  time: number;
  setTime: (value: number) => void;
}) {
  const { stopMining } = useAppStateStore((state) => ({
    stopMining: state.stopMining,
  }));

  const startTimer = () => {
    setTimerOn(true);
  };

  useEffect(() => {
    startTimer();
  }, []);

  // useEffect(() => {
  //   let intervalId: any;
  //   let prevTime = time;

  //   if (timerOn) {
  //     intervalId = setInterval(() => {
  //       prevTime = prevTime + 1;
  //       setTime(prevTime);
  //     }, 1000);
  //   } else {
  //     clearInterval(intervalId);
  //   }

  //   return () => clearInterval(intervalId);
  // }, [timerOn]);

  const stopTimer = (miningType: MiningType) => {
    setTimerOn(false);
    stopMining(miningType);
    setTime(0);
  };

  const formatTime = (timeInSeconds: number): string => {
    const hours = Math.floor(timeInSeconds / 3600);
    const minutes = Math.floor((timeInSeconds % 3600) / 60);
    const seconds = timeInSeconds % 60;

    return `${hours}:${String(minutes).padStart(2, '0')}:${String(
      seconds
    ).padStart(2, '0')}`;
  };

  return (
    <MiningButtonBox>
      <Typography variant="body2" sx={typography.smallHeavy} pr={1}>
        {formatTime(time)}
      </Typography>
      <Typography variant="body2" sx={typography.smallHeavy}>
        |
      </Typography>
      <Button
        variant="text"
        onClick={() => stopTimer(miningType)}
        style={{
          color: '#fff',
        }}
      >
        {t.common.verbs.pause}
      </Button>
    </MiningButtonBox>
  );
}

export default Timer;

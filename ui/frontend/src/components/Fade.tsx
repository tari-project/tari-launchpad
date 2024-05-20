/**
 * Credit to MUI team @ https://mui.com
 */
import * as React from 'react';
import { TransitionProps, TransitionStatus, Transition } from 'notistack';
import {
  useForkRef,
  reflow,
  getTransitionProps,
  createTransition,
} from './shared';

const styles: Partial<Record<TransitionStatus, React.CSSProperties>> = {
  entering: {
    opacity: 1,
  },
  entered: {
    opacity: 1,
  },
};

const Fade = React.forwardRef<HTMLDivElement, TransitionProps>((props, ref) => {
  const {
    children,
    in: inProp,
    timeout = 0,
    style,
    onEnter,
    onEntered,
    onExit,
    onExited,
    direction, // Take this out since this is a Slide-only prop
    ...other
  } = props;

  const nodeRef = React.useRef<HTMLDivElement>(null);
  const handleRefIntermediary = useForkRef((children as any).ref, ref);
  const handleRef = useForkRef(nodeRef, handleRefIntermediary);

  const handleEnter: TransitionProps['onEnter'] = (node, isAppearing) => {
    reflow(node);

    const transitionProps = getTransitionProps({
      style,
      timeout,
      mode: 'enter',
    });
    node.style.webkitTransition = createTransition('opacity', transitionProps);
    node.style.transition = createTransition('opacity', transitionProps);

    if (onEnter) {
      onEnter(node, isAppearing);
    }
  };

  const handleExit: TransitionProps['onExit'] = (node) => {
    const transitionProps = getTransitionProps({
      style,
      timeout,
      mode: 'exit',
    });
    node.style.webkitTransition = createTransition('opacity', transitionProps);
    node.style.transition = createTransition('opacity', transitionProps);

    if (onExit) {
      onExit(node);
    }
  };

  return (
    <Transition
      appear
      in={inProp}
      nodeRef={nodeRef}
      onEnter={handleEnter}
      onEntered={onEntered}
      onExit={handleExit}
      onExited={onExited}
      timeout={timeout}
      {...other}
    >
      {(status, childProps) =>
        React.cloneElement(children as any, {
          style: {
            opacity: 0,
            visibility: status === 'exited' && !inProp ? 'hidden' : undefined,
            ...styles[status],
            ...style,
            ...(children as any).props.style,
          },
          ref: handleRef,
          ...childProps,
        })
      }
    </Transition>
  );
});

Fade.displayName = 'Fade';

export default Fade;

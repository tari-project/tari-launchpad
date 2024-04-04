import { TransitionDuration } from 'notistack';
import React from 'react';

/**
 * passes {value} to {ref}
 *
 * Useful if you want to expose the ref of an inner component to the public API
 * while still using it inside the component.
 * @param ref A ref callback or ref object. If anything falsy, this is a no-op.
 */
function setRef<T>(
  ref:
    | React.MutableRefObject<T | null>
    | ((instance: T | null) => void)
    | null
    | undefined,
  value: T | null
): void {
  if (typeof ref === 'function') {
    ref(value);
  } else if (ref) {
    ref.current = value;
  }
}

export function useForkRef<Instance>(
  refA: React.Ref<Instance> | null | undefined,
  refB: React.Ref<Instance> | null | undefined
): React.Ref<Instance> | null {
  /**
   * This will create a new function if the ref props change and are defined.
   * This means react will call the old forkRef with `null` and the new forkRef
   * with the ref. Cleanup naturally emerges from this behavior.
   */
  return React.useMemo(() => {
    if (refA == null && refB == null) {
      return null;
    }
    return (refValue) => {
      setRef(refA, refValue);
      setRef(refB, refValue);
    };
  }, [refA, refB]);
}

interface ComponentProps {
  style?: React.CSSProperties | undefined;
  /**
   * number: e.g. 400
   * TransitionDuration: e.g. { enter: 200, exit: 400 }
   */
  timeout: number | TransitionDuration;
  mode: 'enter' | 'exit';
}

interface TransitionPropsReturnType {
  duration: number;
  easing: string | undefined;
  delay: string | undefined;
}

export function getTransitionProps(
  props: ComponentProps
): TransitionPropsReturnType {
  const { timeout, style = {}, mode } = props;
  return {
    duration: typeof timeout === 'object' ? timeout[mode] || 0 : timeout,
    easing: style.transitionTimingFunction,
    delay: style.transitionDelay,
  };
}

/**
 * CSS hack to force a repaint
 */
export const reflow = (node: Element): void => {
  // We have to do something with node.scrollTop.
  // Otherwise it's removed from the compiled code by optimisers
  // eslint-disable-next-line no-self-assign
  node.scrollTop = node.scrollTop;
};

interface CreateTransitionOptions {
  duration: number;
  easing?: string;
  delay?: string | number;
}

const defaultEasing = {
  // This is the most common easing curve.
  easeInOut: 'cubic-bezier(0.4, 0, 0.2, 1)',
  // Objects enter the screen at full velocity from off-screen and
  // slowly decelerate to a resting point.
  easeOut: 'cubic-bezier(0.0, 0, 0.2, 1)',
  // Objects leave the screen at full velocity. They do not decelerate when off-screen.
  easeIn: 'cubic-bezier(0.4, 0, 1, 1)',
  // The sharp curve is used by objects that may return to the screen at any time.
  sharp: 'cubic-bezier(0.4, 0, 0.6, 1)',
};

const formatMs = (milliseconds: number) => `${Math.round(milliseconds)}ms`;

export function createTransition(
  props: string | string[] = ['all'],
  options?: CreateTransitionOptions
): string {
  const {
    duration = 300,
    easing = defaultEasing.easeInOut,
    delay = 0,
  } = options || {};

  const properties = Array.isArray(props) ? props : [props];

  return properties
    .map((animatedProp) => {
      const formattedDuration =
        typeof duration === 'string' ? duration : formatMs(duration);
      const formattedDelay =
        typeof delay === 'string' ? delay : formatMs(delay);
      return `${animatedProp} ${formattedDuration} ${easing} ${formattedDelay}`;
    })
    .join(',');
}

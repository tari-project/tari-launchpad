import styled from 'styled-components'

export const StyledBox = styled.div`
  background: ${({ theme }) => theme.backgroundImage};
  border: 1px solid ${({ theme }) => theme.borderColor};
  border-radius: ${({ theme }) => theme.tightBorderRadius()};
  color: ${({ theme }) => theme.secondary};
  padding: ${({ theme }) => theme.spacingVertical()}
    ${({ theme }) => theme.spacingHorizontal()};
  margin: ${({ theme }) => theme.spacingVertical(0.6)} 0;
  box-sizing: border-box;
  display: flex;
  justify-content: space-between;
  column-gap: 0.25em;
`

export const FeedbackContainer = styled.div`
  position: absolute;
  left: 50%;
  bottom: 120%;
  transform: translateX(-50%);
`
export const ValueContainer = styled.div`
  word-break: keep-all;
  -webkit-user-select: none;
  cursor: default;
  font-family: 'AvenirMedium';
  overflow-x: auto;
  overflow-y: hidden;
  white-space: nowrap;
  width: 90%;
  padding: 10px 0;

  ::-webkit-scrollbar {
    height: 10px;
  }

  /* Track */
  ::-webkit-scrollbar-track {
    background: ${({ theme }) => theme.scrollBarThumb};
    border-radius: 6px;
    border: 3px solid transparent;
  }

  /* Handle */
  ::-webkit-scrollbar-thumb {
    background: #ffffff;
    border-radius: 6px;
    background-clip: padding-box;
    border: 3px solid transparent;
  }

  /* Handle on hover */
  ::-webkit-scrollbar-thumb:hover {
    background: #cccccc;
    border-radius: 6px;
    background-clip: padding-box;
    border: 3px solid transparent;
  }
`

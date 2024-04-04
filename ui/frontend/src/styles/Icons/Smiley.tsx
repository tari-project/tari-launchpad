import { SVGProps } from 'react'

const SvgSmiley = (props: SVGProps<SVGSVGElement>) => (
  <svg
    width='1em'
    height='1em'
    viewBox='0 0 24 24'
    fill='none'
    xmlns='http://www.w3.org/2000/svg'
    data-testid='svg-smiley'
    {...props}
  >
    <path
      d='M3.353 8.95A7.511 7.511 0 0 1 8.95 3.353c2.006-.47 4.094-.47 6.1 0a7.511 7.511 0 0 1 5.597 5.597c.47 2.006.47 4.094 0 6.1a7.511 7.511 0 0 1-5.597 5.597c-2.006.47-4.094.47-6.1 0a7.511 7.511 0 0 1-5.597-5.597 13.354 13.354 0 0 1 0-6.1Z'
      stroke='currentColor'
      strokeWidth={1.5}
    />
    <path
      d='M6 11c.3-.598.859-1 1.5-1s1.2.402 1.5 1M16 15c-.798 1.196-2.29 2-4 2s-3.202-.804-4-2M15 11c.3-.598.859-1 1.5-1s1.2.402 1.5 1'
      stroke='currentColor'
      strokeWidth={1.5}
      strokeLinecap='round'
      strokeLinejoin='round'
    />
  </svg>
)

export default SvgSmiley

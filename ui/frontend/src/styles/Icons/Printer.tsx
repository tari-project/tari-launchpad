import { SVGProps } from 'react';

const SvgPrinter = (props: SVGProps<SVGSVGElement>) => (
  <svg
    width="25"
    height="24"
    viewBox="0 0 25 24"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    data-testid="svg-printer"
    {...props}
  >
    <path
      d="M18.5 17.25H19.625C20.1218 17.2485 20.5979 17.0505 20.9492 16.6992C21.3005 16.3479 21.4985 15.8718 21.5 15.375V7.875C21.4985 7.37818 21.3005 6.90212 20.9492 6.55081C20.5979 6.1995 20.1218 6.00148 19.625 6H5.375C4.87818 6.00148 4.40212 6.1995 4.05081 6.55081C3.6995 6.90212 3.50148 7.37818 3.5 7.875V15.375C3.50148 15.8718 3.6995 16.3479 4.05081 16.6992C4.40212 17.0505 4.87818 17.2485 5.375 17.25H6.5"
      stroke="currentColor"
      strokeLinejoin="round"
    />
    <path
      d="M17.36 11.25H7.64C7.0104 11.25 6.5 11.7604 6.5 12.39V19.86C6.5 20.4896 7.0104 21 7.64 21H17.36C17.9896 21 18.5 20.4896 18.5 19.86V12.39C18.5 11.7604 17.9896 11.25 17.36 11.25Z"
      stroke="currentColor"
      strokeLinejoin="round"
    />
    <path
      d="M18.5 6V4.875C18.4985 4.37818 18.3005 3.90212 17.9492 3.55081C17.5979 3.1995 17.1218 3.00148 16.625 3H8.375C7.87818 3.00148 7.40212 3.1995 7.05081 3.55081C6.6995 3.90212 6.50148 4.37818 6.5 4.875V6"
      stroke="currentColor"
      strokeLinejoin="round"
    />
    <path
      d="M18.875 9.75C19.4963 9.75 20 9.24632 20 8.625C20 8.00368 19.4963 7.5 18.875 7.5C18.2537 7.5 17.75 8.00368 17.75 8.625C17.75 9.24632 18.2537 9.75 18.875 9.75Z"
      fill="currentColor"
    />
  </svg>
);

export default SvgPrinter;

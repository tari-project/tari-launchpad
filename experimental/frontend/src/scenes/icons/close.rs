use yew::{html, Html};

// pub fn button() -> Html {
// html! {
// <svg
// width="1em"
// height="1em"
// viewBox="0 0 24 24"
// fill="none"
// xmlns="http://www.w3.org/2000/svg"
// data-testid="svg-close"
// >
// <path
// d="M3.353 8.95A7.511 7.511 0 0 1 8.95 3.353c2.006-.47 4.094-.47 6.1 0a7.511 7.511 0 0 1 5.597 5.597c.47 2.006.47
// 4.094 0 6.1a7.511 7.511 0 0 1-5.597 5.597c-2.006.47-4.094.47-6.1 0a7.511 7.511 0 0 1-5.597-5.597 13.354 13.354 0 0 1
// 0-6.1Z" stroke="currentColor"
// strokeWidth={1.5}
// />
// <path
// d="m13.768 10.232-3.536 3.536m3.536 0-3.536-3.536"
// stroke="currentColor"
// strokeWidth={1.5}
// strokeLinecap="round"
// />
// </svg>
// }
// }

pub fn cross(width: u16, height: u16) -> Html {
    html! {
      <svg
        width={width.to_string()}
        height={height.to_string()}
        viewBox="0 0 6 6"
        fill="none"
        data-testid="svg-closecross"
      >
        <path
          d="M4.76796 1.23242L1.23242 4.76796M4.76796 4.76796L1.23242 1.23242"
          stroke="currentColor"
          strokeWidth="1.5"
          strokeLinecap="round"
        />
      </svg>
    }
}

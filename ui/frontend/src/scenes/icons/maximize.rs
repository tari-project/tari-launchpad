// Copyright 2023. The Tari Project
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
// following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
// disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
// following disclaimer in the documentation and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
// products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

use yew::{html, Html};

pub fn maximize() -> Html {
    html! {
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="17"
          height="16"
          viewBox="0 0 17 16"
          fill="none"
        >
          <path
            d="M4.04504 4.32699C4.04331 3.99321 4.31434 3.72219 4.64812 3.72391L9.99044 3.75145C10.5235 3.7542 10.7885 4.39878 10.4116 4.77571L5.09683 10.0905C4.7199 10.4674 4.07532 10.2024 4.07257 9.66932L4.04504 4.32699Z"
            fill="currentColor"
          />
          <path
            d="M11.7442 12.0263C12.078 12.028 12.349 11.757 12.3473 11.4232L12.3197 6.08085C12.317 5.5478 11.6724 5.28275 11.2955 5.65968L5.98068 10.9745C5.60376 11.3514 5.86881 11.996 6.40185 11.9987L11.7442 12.0263Z"
            fill="currentColor"
          />
        </svg>
    }
}

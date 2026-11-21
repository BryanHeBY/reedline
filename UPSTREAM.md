# Reedline source

The Brush compatibility branch is based on the MIT-licensed Reedline 0.53.0 crates.io release
from [nushell/reedline](https://github.com/nushell/reedline), upstream revision
14be1e0f597b359426b5a5b5ee0b148c701fb38a. Its original license is included. The
published crate's SHA-256 is
`9b061d8059603bc95c569166a79ca60c01ccc8f8b6065035aed66a3753fdb06c`.

Brush uses this fork for input replay and Readline-compatible
editing and display hooks. Changes to upstream source should stay independent
of Brush's shell configuration and key bindings.

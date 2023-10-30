## Description
Rust project that can do diagnostic in ECU-Autosar via ethernet.
Should work on both Linux&Windows
![GUI](documents/GUI.jpg)

## Main Flow
![main_flow](documents/main_flow.jpg)

## Setup
For Linux/MacOS install cargo by "curl https://sh.rustup.rs -sSf | sh". Install some dependencies if needed</br>
For Window, download and install cargo at https://win.rustup.rs/</br>

## Build
There are 2 modes for building, 1 for GUI application and 1 for CLI. Open cmd/terminal and use 1 command below:</br>
cargo build --no-default-features --features "cli" --release</br>
cargo build --no-default-features --features "cli"</br>
cargo build --features "gui" --release</br>
cargo build --features "gui"</br>

## Execute
./target/debug/diag_tool --debug --config json/config.json --sequence json/sequence.json

## JSON explaination
.....

## <span style="color: green;">TODO list</span>

<summary><s>Work well in Linux&Window</s></summary>
<summary><s>Support run Diagnostic sequence</s></summary>
<summary><s>Support Security-Access Diag</s></summary>
<summary><s>Support SWDL for hex, S37, vbf format</s></summary>
<summary><s>Support send tester-present cyclic feature</s></summary>
<summary>Lock json folder by password to protect sensitive data (OEM keys)</summary>
<summary><s>Support execute CLI cmd from terminal (send diag cmd)</s></summary>
<summary>Support IPv6</summary>
<summary><s>Support Debug-log</s></summary>
<summary>Full-compliance for ISO13400</summary>
<summary>Full-compliance for ISO14229-1 3rd</summary>
<summary>Support TLS for DoIp layer</summary>
<summary><s>Support GUI</s></summary>
<summary>Calculate response time</summary>
<summary>Handle error code</summary>
<summary>Support to export test report for sequence</summary>

## <span style="color: orange;">DEMO</span>

## <span style="color: yellow;">TESTING</span>
<details>
    <summary>Test-cases</summary>
</details>

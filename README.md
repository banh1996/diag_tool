## Description
Rust project that can do diagnostic in ECU-Autosar via ethernet.
Should work on both Linux&Windows

## Main Flow
![main_flow](documents/main_flow.jpg)

## Build
cargo build

## Execute
./target/debug/diag_tool --debug --config json/config.json --sequence json/sequence.json

## JSON explaination
.....

## <span style="color: green;">TODO list</span>

<summary><s>Work well in Linux</s></summary>
<summary><s>Work well in Window</s></summary>
<summary><s>Parse json configuration file</s></summary>
<summary><s>Check json configuration parameters before execute</s></summary>
<summary><s>Parse json sequence file</s></summary>
<summary>Check json sequence parameters before execute</summary>
<summary><s>full-sequence for SWDL</s></summary>
<summary>Support SWDL for hex, S37, vbf format</summary>
<summary>Lock json folder by password to protect sensitive data (OEM keys)</summary>
<summary>Execute CLI cmd from terminal (send diag cmd)</summary>
<summary>Support IPv6</summary>
<summary><s>Support Debug-log</s></summary>
<summary><s>Support option argument</s></summary>
<summary>Full-compliance for ISO13400</summary>
<summary>Full-compliance for ISO14229-1 3rd</summary>
<summary>Support TLS for DoIp layer</summary>
<summary>Support GUI?</summary>
<summary>Calculate response time</summary>
<summary>Make it more OOP</summary>
<summary>Handle error code</summary>
<summary>Support to export test report</summary>


## <span style="color: yellow;">TESTING</span>
<details>
    <summary>Test-cases</summary>
</details>

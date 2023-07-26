## Description
Rust project that can do diagnostic in ECU-Autosar via ethernet.
Should work on both Linux&Windows

## Main Flow
![main_flow](documents/main_flow.jpg)

## Build
cargo build

## Run
./target/debug/diag_tool --debug --config json/config.json --sequence json/sequence.json


## <span style="color: green;">TODO list</span>

<summary><s>Work well in Linux</s></summary>
<summary>Work well in Window</summary>
<summary><s>Parse json configuration file</s></summary>
<summary>Parse json sequence file</summary>
<summary>full-sequence for SWDL</summary>
<summary>Execute CLI cmd</summary>
<summary>Support IPv6</summary>
<summary>Full-compliance for ISO13400</summary>
<summary>Full-compliance for ISO14229-3</summary>
<summary>Support TLS for DoIp layer</summary>
<summary>Make it more OOP</summary>
<summary>Handle error code</summary>
<summary>Support to export test report</summary>

<details>
    <summary>Test-cases</summary>
</details>

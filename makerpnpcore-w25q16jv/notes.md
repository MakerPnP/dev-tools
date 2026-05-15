# Notes

## Pinouts

PF4 - P2 CLK
PG12 - P2 NCS
PD4 - P1 IO4
PH3 - P1 IO5
PC3 - P1 IO6
PE10 - P1 IO7

## Flash

Winbond W25Q16JVUXIQ/W25Q16JVUXIM

8192 pages, 256 bytes each.
Block erases (block sizes):
16 pages (4KB).
128 pages (32KB) block.
256 pages (64KB) block.
512 sectors.
32 blocks.
<= 133Mhz Clock

## Project creation

```
$ cargo generate gh:probe-rs/flash-algorithm-template
⚠️   Favorite `gh:probe-rs/flash-algorithm-template` not found in config, using it as a git repository: https://github.com/probe-rs/flash-algorithm-template.git
🤷   Project Name: h7-p2clk-pf4-p2-ncs-pg12-p1-io4-7
🔧   Destination: D:\Users\Hydra\Documents\dev\projects\makerpnp\makerpnpcore\h7-p2clk-pf4-p2-ncs-pg12-p1-io4-7 ...
🔧   project-name: h7-p2clk-pf4-p2-ncs-pg12-p1-io4-7 ...
🔧   Generating template ...
✔ 🤷   What is the target architecture? · thumbv7em-none-eabihf
🤷   What is the target RAM start address where the flash algorithm should be placed?: 0x24000000
🤷   What is the target RAM size in bytes?: 0x10000
🤷   What is the target Flash start address of the device the flash algorithm is for?: 0x0
🤷   What is the target Flash size in bytes?: 0x200000
🤷   What is the target Flash page size in bytes?: 0x100
🤷   What is the target Flash sector size in bytes?: 0x1000
🤷   What is the erased state of a byte in Flash?: 0xff
[ 1/16]   Done: .cargo\config.toml                                                                                      [ 2/16]   Done: .cargo                                                                                                  [ 3/16]   Done: .github\FUNDING.yml                                                                                     [ 4/16]   Done: .github\workflows\ci.yml                                                                                [ 5/16]   Done: .github\workflows                                                                                       [ 6/16]   Done: .github                                                                                                 [ 7/16]   Done: .gitignore                                                                                              [ 8/16]   Done: Cargo.toml                                                                                              [ 9/16]   Done: LICENSE-APACHE                                                                                          [10/16]   Done: LICENSE-MIT                                                                                             [11/16]   Skipped: README.md                                                                                            [12/16]   Ignored: calculate-memory.rhai                                                                                [13/16]   Done: link.x                                                                                                  [14/16]   Done: src\main.rs                                                                                             [15/16]   Done: src                                                                                                     [16/16]   Done: template.yaml                                                                                           🔧   Moving generated files into: `D:\Users\Hydra\Documents\dev\projects\makerpnp\makerpnpcore\h7-p2clk-pf4-p2-ncs-pg12-p1-io4-7`...
🔧   Initializing a fresh Git repository
✨   Done! New project created D:\Users\Hydra\Documents\dev\projects\makerpnp\makerpnpcore\h7-p2clk-pf4-p2-ncs-pg12-p1-io4-7
```

^ IMPORTANT: the above settings were wrong, since this was the first time trying, refer to the current yaml files.

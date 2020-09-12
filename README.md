# WOL

A little wake-on-lan utility

## Usage

```
> wol  --mac 18-C0-4D-42-2D-EA
sent
> echo $?
0

> wol --mac 18-C0-4D-42-2D-EA --to 192.168.0.101
sent
> echo $?
0

> wol --mac 18-C0-4D-42-2D-XX
error
> echo $?
1
```

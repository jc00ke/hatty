# Hatty

A little wake-on-lan utility.

## Usage

```
> hatty  --mac 18-C0-4D-42-2D-EA
sent
> echo $?
0

> hatty --mac 18-C0-4D-42-2D-EA --to 192.168.0.101
sent
> echo $?
0

> hatty --mac 18-C0-4D-42-2D-XX
error
> echo $?
1
```

## Debugging

```
> sudo netcat -ul 9
```

```
> cargo run -- --mac 18-C0-4D-42-2D-EA --to 127.0.0.1
```

Then you should see something from `netcat`.

## The Name

```
wake-on-lan -> wol -> wooly -> wooly bully -> HATTY TOLD MATTY
```
[Sam the Sham and the Pharaohs](https://www.youtube.com/watch?v=uE_MpQhgtQ8)

## Resources & Inspiration

* [Wikipedia](https://en.wikipedia.org/wiki/Wake-on-LAN#Magic_packet)
* [wakey](https://github.com/LesnyRumcajs/wakey)

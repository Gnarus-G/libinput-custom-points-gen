# libinput-custom-points-gen

A CLI utility to generate custom motion points for libinput by specifying the linear acceleration factor and cap like in Raw Accel's interface.

## Usage

```
Usage: libinput-points [OPTIONS] <ACCEL_FACTOR> <CAP> [INPUT_OFFSET]

Arguments:
  <ACCEL_FACTOR>
  <CAP>           Max gain
  [INPUT_OFFSET]  [default: 0]

Options:
  -s <STEP>              [default: 1]
  -x, --print-xorg-conf  Print an xorg .conf configuring libinput accel settings
  -q
  -h, --help             Print help
```

### Save xorg conf file

like

```sh
libinput-points 0.3 2 2 -x > /etc/X11/xorg.conf.d/50-mouse-acceleration.conf
```

# References

https://wayland.freedesktop.org/libinput/doc/latest/pointer-acceleration.html#ptraccel-profile-custom
https://wiki.archlinux.org/title/Mouse_acceleration#Persistent_configuration

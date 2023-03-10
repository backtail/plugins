# AUDIO PLUGINS
Hey, here are my plugins. Written in Rust because I had to. I actually really like [nih-plug](https://github.com/robbert-vdh/nih-plug) as a dev platform. Backed by [Yanel DSP](https://github.com/backtail/yanel_dsp), my embedded oriented effects library, which means these plugins come with some caviats like non-changeable sampling rate (sometimes).

These effects have **no** GUI. Can still be used (at least) with Reaper and Live!

### Compile it yourself
This will yield a vst3 plugin
```shell
$ cargo xtask bundle {project_name} --release
```

# Simple Delay
Does what is says. Has some little clicks when changing the delay time too fast. Subject to change!

### Known Bugs
Currently doesn't work under Live 10.

# Multi Filter
Varaible state filter with common filter types. Currently only Butterworth like behaviour. Pretty stable, though and sounds not so bad.

# Freeverb
An implementation of the famous Freeverb. Take a look at [Yanel DSP](https://github.com/backtail/yanel_dsp), to learn more about the origins of this code.
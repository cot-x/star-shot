#!/bin/sh

mkdir -p rsc/fonts
curl -o rsc/fonts/ipag00303.zip https://moji.or.jp/wp-content/ipafont/IPAfont/ipag00303.zip
unzip rsc/fonts/ipag00303.zip -d rsc/fonts/
rm rsc/fonts/ipag00303.zip

mkdir -p rsc/wav
curl -o rsc/wav/bgm.mp3 http://www.hmix.net/music/n/n35.mp3
ffmpeg -i rsc/wav/bgm.mp3 rsc/wav/bgm.wav
rm rsc/wav/bgm.mp3
curl -o rsc/wav/beam.mp3 https://taira-komori.jpn.org/sound/sf01/laputa.mp3
curl -o rsc/wav/crash.mp3 https://taira-komori.jpn.org/sound/magic01/vanishing2.mp3
curl -o rsc/wav/vanish.mp3 https://taira-komori.jpn.org/sound/magic01/voice_of_light.mp3
curl -o rsc/wav/dead.mp3 https://taira-komori.jpn.org/sound/sf01/ele_shock_wave.mp3

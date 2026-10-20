# Demo Script

- Event:
- Limit: ___ minutes
- Target length: ___ minutes
- Resolution: 1080p
- Recording tool: OBS / Chrome DevTools / ffmpeg
- Backup recording: yes

## Narration

| Time | Narration | Screen action |
|---|---|---|
| 0:00 | | |
| 0:20 | | |
| 0:40 | | |
| 2:15 | | |
| 2:45 | | |

## Pre-record checklist

- [ ] Testnet deployed
- [ ] Wallet funded
- [ ] Realistic data seeded
- [ ] Notifications off
- [ ] Browser zoom 110-125%
- [ ] Microphone tested
- [ ] Backup video recorded

## Post-record verification

```bash
ffprobe -v error -show_entries format=duration -show_entries stream=width,height -of default=noprint_wrappers=1 demo.mp4
```

- [ ] Length inside limit
- [ ] At least 720p
- [ ] Audio clear
- [ ] No speed-up
- [ ] No AI voiceover
- [ ] Public link opens without login

to open devices as a user instead of root. Try adding this to `/utc/udev/rules.d/72-hidraw-access.rules`

```
KERNEL=="hidraw*", SUBSYSTEM=="hidraw", MODE="0660", TAG+="uaccess"
```

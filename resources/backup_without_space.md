---
title: Backup without free space
date: 2025-11-25
author: Hong Quan
tags: [linux]
summary: A way to do backup if a serve is out of disk space
---

Sometimes your server runs out of disk space and you can't create a `.zip` or
`.tar.gz` file, you can still *compress and send it directly to another
server* without using extra space:

```
tar czf --C /home/user public_html | ssh user@server-backup "cat >
/opt/website_backup.tar.gz"
```

## Explanation

- `tar czf --C /home/user public_html` -> Compress the public_html folder and
    output to stdout.
- | -> pipe the compressed data to the next command.
- `ssh user@server-backup "cat > /opt/website_backup.tar.gz"` -> write the
    data directly into a backup file on the remote server.

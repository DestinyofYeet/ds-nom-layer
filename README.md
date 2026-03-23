Using bash

```bash
nix build ... --log-format internal-json -v | ds-nom-layer | nom --json
```

Using nushell
```nu
nix build ... --log-format internal-json -v o+e>| ds-nom-layer | nom --json
```

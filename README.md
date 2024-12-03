# TermProxy

A simple cli tool for setting up proxy environment variables. Automatically reads the proxy settings from the system and sets them up in the environment variables.

## Usage

### 1. Install

TODO

### 2. Set up

<details>
<summary>Bash</summary>

Add the following line to your `.bashrc` or `.bash_profile`:

```bash
eval "$(termproxy init bash)"
```

</details>

<details>
<summary>Cmd</summary>

1. Add a string value to the registry key:

   - Apply `.cmdrc.cmd` to all users:

   ```cmd
   reg add "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Command Processor" /v "AutoRun" /t REG_SZ /d "if exist %USERPROFILE%\.cmdrc.cmd %USERPROFILE%\.cmdrc.cmd"
   ```

   - Or apply to the current user only:

   ```cmd
   reg add "HKEY_CURRENT_USER\SOFTWARE\Microsoft\Command Processor" /v "AutoRun" /t REG_SZ /d "if exist %USERPROFILE%\.cmdrc.cmd %USERPROFILE%\.cmdrc.cmd"
   ```

2. Add the following content to the file `%USERPROFILE%\.cmdrc.cmd`:

```cmd
@echo off && termproxy init cmd -o "%TEMP%/proxy.cmd" && call "%TEMP%/proxy.cmd" && @echo on
```

</details>

<details>
<summary>Elvish</summary>

Add the following line to your `~/.elvish/rc.elv`:

```elvish
eval (termproxy init elvish)
```

</details>

<details>
<summary>Fish</summary>

Add the following line to your `~/.config/fish/config.fish`:

```fish
termproxy init fish | source
```

</details>

<details>
<summary>Ion</summary>

Add the following line to your `~/.config/ion/initrc`:

```sh
eval $(termproxy init ion)
```

</details>

<details>
<summary>Nu</summary>

1. Add the following line to `$nu.env-path`:

```sh
# Create `.cache` directory first if it doesn't exist
termproxy init nu -o ~/.cache/__proxy.nu
```

2. Add the following line to your `$nu.config-path`:

```sh
use ~/.cache/__proxy.nu
```

</details>

<details>
<summary>PowerShell</summary>

Add the following line to your `$PROFILE`:

```powershell
Invoke-Expression (termproxy init powershell | Out-String)
```

</details>

<details>
<summary>Tcsh</summary>

Add the following line to your `~/.cshrc`:

```csh
eval `termproxy init tcsh`
```

</details>

<details>
<summary>Xonsh</summary>

Add the following line to your `~/.xonshrc`:

```python
execx($(termproxy init xonsh))
```

</details>

<details>
<summary>Zsh</summary>

Add the following line to your `~/.zshrc`:

```zsh
eval "$(termproxy init zsh)"
```

</details>

## Limitations

1. There is no real standard for the `no_proxy` environment variable. So we follow the behavior of `curl`.
   - Windows: wildcard domains and wildcard ips on windows will be converted into stripped domains and CIDRs, and wildcard forms like `www.*.com`, `192.168.*.1` might not work.

## Tips

1. Using `sudo`

If you are using `sudo` to run a command, you can use the `-E` option to preserve the environment variables, or you can specify the environment variables you want to preserve in `/etc/sudoers`:

```
Defaults env_keep += "http_proxy https_proxy all_proxy no_proxy"
```

## TODOs

- [ ] Add support for `macos`
- [ ] Publish to scoop, brew and aur.

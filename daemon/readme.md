
# initial env setup

```
sudo useradd -r -s /sbin/nologin damnfinetoot
```

# install

Simply run the `./daemon/install.sh` script with the appropriate flags

- Logs are written to `/var/log/damnfinetoot/*.log`
- Valid login data toml should be available when running as a daemon
  - If missing, simply `cargo run` and follow the steps. The login data
    should be written to the right location to be used by the daemon.

# Analysis

```
sudo systemctl status damnfinetoot.service  # status of service, also 'restart' etc
sudo journalctl                             # see systemd logs
```

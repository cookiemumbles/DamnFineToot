#!/bin/bash
#
# file location conventions: https://unix.stackexchange.com/a/493363/160958

help() {
  cat << EOF
installs damnfinetoot as daemon

Usage:
./daemon/install.sh [flags]

-h                Print this help
-e                Build and install the executable
-d                Install the daemon
EOF
exit
}

main() {
  while getopts "hed" opt; do
    case $opt in
      h)
        help
        exit
        ;;
      e)
        export EXE=true
        ;;
      d)
        export DAE=true
        ;;
    esac
  done
  shift "$((OPTIND - 1))"

  if [[ ! -d /etc/damnfinetoot/ ]]; then
    echo "WARN: credentials file doesn't exist. Run 'cargo run' and follow the steps first."
  fi
  # TODO: check if user 'damnfinetoot' exists

  if [[ ! -f /etc/damnfinetoot/mastodon-data.toml ]]; then
    if [[ ! -d /etc/damnfinetoot/ ]]; then
      echo "WARN: Creating missing parent dir."
      sudo mkdir -p /etc/damnfinetoot/
    fi

    if [[ -f './mastodon-data.toml' ]];then
      echo "Copying local credentials file to system location..."
      sudo cp ./mastodon-data.toml /etc/damnfinetoot/
    else
      echo "WARN: No credential file found locally or on the system."
      echo "      Run 'cargo run' and follow the steps to create a local file,"
      echo "      then run this script again and the file will be copied correctly."
    fi
  fi

  if [[ "$EXE" == "true" ]];then
    cargo build --profile release
    echo "copying to /user/sbin..."
    sudo cp target/release/damnfinetoot /usr/sbin/
  else
    echo "skipping installing damnfinetoot..."
  fi
  if [[ "$DAE" == "true" ]];then
    echo "copying service file to system location..."
    sudo cp ./daemon/damnfinetoot.service  /etc/systemd/system/
    echo "reloading daemon..."
    sudo systemctl daemon-reload
    echo "restarting daemon..."
    sudo systemctl restart damnfinetoot.service
  else
    echo "skipping updating daemon scripts"
    if [[ "$EXE" == "true" ]];then
      echo "restarting daemon"
      sudo systemctl restart damnfinetoot.service
    fi
  fi
}


# Main entry point (excluded from tests)
if [[ "$0" == "$BASH_SOURCE" ]]; then
  main "$@"
fi
